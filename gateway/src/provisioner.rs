//! Attach and send/receive BT Mesh messages
use bluer::{
    mesh::{
        application::Application,
        element::*,
        network::Network,
        provisioner::{Provisioner, ProvisionerControlHandle, ProvisionerMessage},
    },
    Uuid,
};
use btmesh_common::address::LabelUuid;
use btmesh_models::{
    foundation::configuration::{
        app_key::AppKeyMessage,
        model_publication::{PublishAddress, PublishPeriod, PublishRetransmit, Resolution},
        ConfigurationClient, ConfigurationMessage, ConfigurationServer,
    },
    generic::{battery::GENERIC_BATTERY_SERVER, onoff::GENERIC_ONOFF_SERVER},
    sensor::SENSOR_SETUP_SERVER,
    Message, Model,
};
use btmesh_operator::{BtMeshCommand, BtMeshDeviceState, BtMeshEvent, BtMeshOperation};
use dbus::Path;
use futures::{pin_mut, StreamExt};
use paho_mqtt as mqtt;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::{
    sync::{broadcast, mpsc, Mutex},
    time::sleep,
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{broadcast, mpsc, oneshot, Mutex},
    time::{sleep, Instant},
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
    pin_mut!(element_control);
    let provisioning: Mutex<HashSet<Uuid>> = Mutex::new(HashSet::new());
    let (provision_tx, mut provision_rx) = mpsc::channel(32);

    log::info!("Starting provisioner event loop");
    loop {
        tokio::select! {
            evt = prov_rx.recv() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ProvisionerMessage::AddNodeComplete(uuid, unicast, count) => {
                                log::info!("Successfully added node {:?} to the address {:#04x} with {:?} elements", uuid, unicast, count);

                                sleep(Duration::from_secs(6)).await;

                                log::info!("Add app key");
                                node.add_app_key(element_path.clone(), unicast, 0, 0, false).await?;
                                sleep(Duration::from_secs(4)).await;
                                log::info!("Bind sensor server");
                                node.bind(element_path.clone(), unicast, 0, SENSOR_SETUP_SERVER).await?;
                                sleep(Duration::from_secs(4)).await;
                                log::info!("Bind onoff server");
                                node.bind(element_path.clone(), unicast, 0, GENERIC_ONOFF_SERVER).await?;
                                sleep(Duration::from_secs(4)).await;
                                log::info!("Bind battery server");
                                node.bind(element_path.clone(), unicast, 0, GENERIC_BATTERY_SERVER).await?;
                                sleep(Duration::from_secs(4)).await;


                                let label = LabelUuid::new(Uuid::parse_str("f0bfd803cde184133096f003ea4a3dc2")?.into_bytes()).map_err(|_| std::fmt::Error)?;
                                let pub_address = PublishAddress::Virtual(label);
                                log::info!("Add pub-set for sensor server");
                                node.pub_set(element_path.clone(), unicast, pub_address, 0, PublishPeriod::new(3, Resolution::Seconds1), PublishRetransmit::from(0), SENSOR_SETUP_SERVER).await?;
                                sleep(Duration::from_secs(4)).await;
                                log::info!("Add pub-set for battery server");
                                node.pub_set(element_path.clone(), unicast, pub_address, 0, PublishPeriod::new(60, Resolution::Seconds1), PublishRetransmit::from(5), GENERIC_BATTERY_SERVER).await?;
                                sleep(Duration::from_secs(5)).await;


                                let topic = format!("btmesh/{}", uuid.as_simple().to_string());
                                log::info!("Sending message to topic {}", topic);
                                let status = BtMeshEvent {
                                    status: BtMeshDeviceState::Provisioned {
                                        address: unicast,
                                    },
                                };

                                let data = serde_json::to_string(&status)?;
                                let message = mqtt::Message::new(topic, data.as_bytes(), 1);
                                if let Err(e) = mqtt_client.publish(message).await {
                                    log::warn!(
                                        "Error publishing provisioning status: {:?}",
                                        e
                                    );
                                }
                                provisioning.lock().await.remove(&uuid);
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
                                provisioning.lock().await.remove(&uuid);
                            }
                        }
                    },
                    None => break,
                }
            },
            Some(device) = provision_rx.recv() => {
                log::info!("Provisioning {:?}", device);
                match node.management.add_node(device).await {
                    Ok(_) => {
                        log::info!("Provisioning for {:?} is done", device);
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
                        if let Err(e) = mqtt_client.publish(message).await {
                            log::warn!(
                                "Error publishing provisioning status: {:?}",
                                e
                            );
                        }
                    }
                }
            }
            evt = element_control.next() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ElementMessage::Received(received) => {
                                log::info!("Received element message: {:?}", received);
                            },
                            ElementMessage::DevKey(received) => {
                                log::info!("Received devkey message with opcode {:?}", received.opcode);
                                match ConfigurationClient::parse(&received.opcode, &received.parameters).map_err(|_| std::fmt::Error)? {
                                    Some(message) => {
                                        log::info!("Received configuration message: {:?}", message);
                                    },
                                    None => {},
                                }
                            }
                        }
                    },
                    None => break,
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
                                            let mut set = provisioning.lock().await;
                                            if !set.contains(&uuid) {
                                                set.insert(uuid.clone());
                                                provision_tx.send(uuid.clone()).await?;
                                            }
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

    log::info!("Shutting down provisioner");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
