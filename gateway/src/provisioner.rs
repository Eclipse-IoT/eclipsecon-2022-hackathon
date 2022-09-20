//! Attach and send/receive BT Mesh messages
use super::node_configurator;
use bluer::{
    mesh::{
        application::Application,
        element::*,
        network::Network,
        node::Node,
        provisioner::{Provisioner, ProvisionerControlHandle, ProvisionerMessage},
    },
    Uuid,
};
use btmesh_common::address::LabelUuid;
use btmesh_models::{
    foundation::configuration::{
        model_publication::{PublishAddress, PublishPeriod, PublishRetransmit, Resolution},
        ConfigurationClient, ConfigurationMessage, ConfigurationServer,
    },
    generic::{battery::GENERIC_BATTERY_SERVER, onoff::GENERIC_ONOFF_SERVER},
    sensor::SENSOR_SETUP_SERVER,
};
use btmesh_operator::{BtMeshCommand, BtMeshDeviceState, BtMeshEvent, BtMeshOperation};
use dbus::Path;
use paho_mqtt as mqtt;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::{
    sync::{broadcast, mpsc, Mutex},
    time::sleep,
};

pub struct Config {
    start_address: u16,
    token: String,
}

impl Config {
    pub fn new(token: String, start_address: u16) -> Self {
        Self {
            token,
            start_address,
        }
    }
}

pub async fn run(
    mesh: Network,
    config: Config,
    mut commands: broadcast::Receiver<(String, Vec<u8>)>,
    mqtt_client: mqtt::AsyncClient,
) -> Result<(), anyhow::Error> {
    let (element_control, element_handle) = element_control(10);
    let (app_tx, _app_rx) = mpsc::channel(4);

    let root_path = Path::from("/mesh/cfgclient");
    let app_path = Path::from(format!("{}/{}", root_path.clone(), "application"));
    let element_path = Path::from(format!("{}/{}", root_path.clone(), "ele00"));

    let (prov_tx, mut prov_rx) = mpsc::channel(4);

    let sim = Application {
        path: app_path,
        elements: vec![Element {
            path: element_path.clone(),
            location: None,
            models: vec![
                Arc::new(FromDrogue::new(ConfigurationServer::default())),
                Arc::new(FromDrogue::new(ConfigurationClient::default())),
            ],
            control_handle: Some(element_handle),
        }],
        provisioner: Some(Provisioner {
            control_handle: ProvisionerControlHandle {
                messages_tx: prov_tx,
            },
            // TODO fix bluer
            start_address: config.start_address as i32,
        }),
        events_tx: app_tx,
    };

    let registered = mesh.application(root_path.clone(), sim).await?;

    let node = mesh.attach(root_path.clone(), &config.token).await?;

    // TODO: Persist this state
    let mut provisioned: HashSet<Uuid> = HashSet::new();

    let (provision_tx, mut provision_rx) = mpsc::channel(32);
    let (configure_tx, configure_rx) = mpsc::channel(32);

    let mut tasks = Vec::new();
    tasks.push(tokio::spawn(node_configurator::run(
        configure_rx,
        mqtt_client.clone(),
        element_control,
        node.clone(),
    )));

    log::info!("Starting provisioner event loop");
    loop {
        tokio::select! {
            evt = prov_rx.recv() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ProvisionerMessage::AddNodeComplete(uuid, unicast, count) => {
                                log::info!("Successfully added node {:?} to the address {:#04x} with {:?} elements", uuid, unicast, count);

                                log::info!("Add app key");
                                node.add_app_key(element_path.clone(), unicast, 0, 0, false).await?;

                                log::info!("Bind sensor server");
                                let msg = Node::bind_create(unicast, 0, SENSOR_SETUP_SERVER)?;
                                let config = NodeConfigurationMessage::Configure(
                                    NodeConfiguration {
                                        message: msg,
                                        path: element_path.clone(),
                                        address: unicast,
                                    }
                                );
                                configure_tx.send(config).await?;
                                log::info!("Bind onoff server");
                                let msg = Node::bind_create(unicast, 0, GENERIC_ONOFF_SERVER)?;
                                let config = NodeConfigurationMessage::Configure(
                                    NodeConfiguration {
                                        message: msg,
                                        path: element_path.clone(),
                                        address: unicast,
                                    }
                                );
                                configure_tx.send(config).await?;

                                log::info!("Bind battery server");
                                let msg = Node::bind_create(unicast, 0, GENERIC_BATTERY_SERVER)?;
                                let config = NodeConfigurationMessage::Configure(
                                    NodeConfiguration {
                                        message: msg,
                                        path: element_path.clone(),
                                        address: unicast,
                                    }
                                );
                                configure_tx.send(config).await?;

                                let label = LabelUuid::new(Uuid::parse_str("f0bfd803cde184133096f003ea4a3dc2")?.into_bytes()).map_err(|_| std::fmt::Error)?;
                                let pub_address = PublishAddress::Virtual(label);
                                log::info!("Add pub-set for sensor server");
                                let msg = Node::pub_set_create(unicast, pub_address, 0, PublishPeriod::new(3, Resolution::Seconds1), PublishRetransmit::from(0), SENSOR_SETUP_SERVER)?;
                                let config = NodeConfigurationMessage::Configure(
                                    NodeConfiguration {
                                        message: msg,
                                        path: element_path.clone(),
                                        address: unicast,
                                    }
                                );
                                configure_tx.send(config).await?;
                                log::info!("Add pub-set for battery server");
                                let msg = Node::pub_set_create(unicast, pub_address, 0, PublishPeriod::new(3, Resolution::Seconds1), PublishRetransmit::from(0), GENERIC_BATTERY_SERVER)?;
                                let config = NodeConfigurationMessage::Configure(
                                    NodeConfiguration {
                                        message: msg,
                                        path: element_path.clone(),
                                        address: unicast,
                                    }
                                );
                                configure_tx.send(config).await?;

                                configure_tx.send(NodeConfigurationMessage::Finish(uuid, unicast)).await?;
                            },
                            ProvisionerMessage::AddNodeFailed(uuid, reason) => {
                                log::info!("Failed to add node {:?}: '{:?}'", uuid, reason);

                                 let status = BtMeshEvent {
                                   status: BtMeshDeviceState::Provisioning { error: Some(reason) }
                                 };

                                let topic = format!("btmesh/{}", uuid.as_simple().to_string());
                                log::info!("Sending message to topic {}", topic);
                                let data = serde_json::to_string(&status)?;
                                let message = mqtt::Message::new(topic, data.as_bytes(), 1);
                                if let Err(e) = mqtt_client.publish(message).await {
                                    log::warn!(
                                        "Error publishing provisioning status: {:?}",
                                        e
                                    );
                                }
                            }
                        }
                    },
                    None => break,
                }
            },
            Some(device) = provision_rx.recv() => {
                if !provisioned.contains(&device) {
                    provisioned.insert(device.clone());
                    log::info!("Provisioning {:?}", device);
                    match node.management.add_node(device.clone()).await {
                        Ok(_) => {
                        }
                        Err(e) => {
                            let status = BtMeshEvent {
                                status: BtMeshDeviceState::Provisioning {
                                    error: Some(e.to_string())
                                }
                            };

                            let topic = format!("btmesh/{}", device);
                            let data = serde_json::to_string(&status)?;
                            let message = mqtt::Message::new(topic, data.as_bytes(), 1);
                            log::info!("Provisioning failed: {:?}, publishing status", e);
                            if let Err(e) = mqtt_client.publish(message).await {
                                log::warn!(
                                    "Error publishing provisioning error status: {:?}",
                                    e
                                );
                            }
                            provisioned.remove(&device);
                        }
                    }
                } else {
                    log::warn!("Device {} already provisioned, ignoring", device);
                }
            },
            command = commands.recv() => {
                match command {
                    Ok((_, command)) => {
                        if let Ok(data) = serde_json::from_slice::<BtMeshCommand>(&command[..]) {
                            log::info!("Parsed command payload: {:?}", data);
                            match data.command {
                                BtMeshOperation::Provision {
                                    device
                                } => {
                                    if let Ok(uuid) = Uuid::parse_str(&device) {
                                        provision_tx.send(uuid).await?;
                                    }
                                }
                                BtMeshOperation::Reset {
                                    address,
                                    device,
                                } => {
                                    let topic = "btmesh";
                                    log::info!("Resetting device, publishing response to {}", topic);
                                    let mut error = None;

                                    for _ in 0..5 {
                                        error = match node.reset(element_path.clone(), address).await {
                                            Ok(_) => {
                                                None
                                            }
                                            Err(e) => {
                                                Some(e.to_string())
                                            }
                                        };
                                    }
                                    let status = BtMeshEvent {
                                        status: BtMeshDeviceState::Reset {
                                            error,
                                            device: device.to_string(),
                                        }
                                    };

                                    let data = serde_json::to_string(&status)?;
                                    let message = mqtt::Message::new(topic, data.as_bytes(), 1);
                                    if let Err(e) = mqtt_client.publish(message).await {
                                        log::warn!(
                                            "Error publishing reset status: {:?}",
                                            e
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        break
                    }
                }
            }
        }
    }

    futures::future::join_all(tasks).await;
    log::info!("Shutting down provisioner");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

#[derive(Debug)]
pub enum NodeConfigurationMessage<'a> {
    Configure(NodeConfiguration<'a>),
    Finish(Uuid, u16),
}

#[derive(Debug)]
pub struct NodeConfiguration<'a> {
    pub message: ConfigurationMessage,
    pub path: Path<'a>,
    pub address: u16,
}
