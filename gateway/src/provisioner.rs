//! Attach and send/receive BT Mesh messages
use super::node_configurator;
use bluer::{
    mesh::{
        application::Application,
        element::*,
        network::Network,
        provisioner::{Provisioner, ProvisionerControlHandle, ProvisionerMessage},
    },
    Uuid,
};
use btmesh_models::foundation::configuration::{
    ConfigurationClient, ConfigurationMessage, ConfigurationServer,
};
use btmesh_operator::{BtMeshCommand, BtMeshDeviceState, BtMeshEvent, BtMeshOperation};
use dbus::Path;
use paho_mqtt as mqtt;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    sync::{broadcast, broadcast::error::RecvError, mpsc},
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

    let mut provisioned: HashMap<Uuid, Instant> = HashMap::new();

    let (provision_tx, mut provision_rx) = mpsc::channel(32);
    let (configure_tx, configure_rx) = mpsc::channel(32);

    let mut tasks = Vec::new();
    tasks.push(tokio::spawn(node_configurator::run(
        configure_rx,
        mqtt_client.clone(),
        element_path.clone(),
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
                            ProvisionerMessage::AddNodeComplete(uuid, unicast, _count) => {
                                configure_tx.send(NodeConfigurationMessage::Configure(uuid, unicast)).await?;
                            },
                            ProvisionerMessage::AddNodeFailed(uuid, reason) => {
                                log::info!("Failed to add node {:?}: '{:?}'", uuid, reason);

                                let device = uuid.as_simple().to_string();
                                 let status = BtMeshEvent {
                                   status: BtMeshDeviceState::Provisioning { device, error: Some(reason) }
                                 };

                                let data = serde_json::to_string(&status)?;
                                let message = mqtt::Message::new("btmesh", data.as_bytes(), 1);
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
            Some(uuid) = provision_rx.recv() => {
                const MAX_CACHED: Duration = Duration::from_secs(30);
                let now = Instant::now();
                let do_provision = provisioned.get(&uuid).map(|s| now.duration_since(*s) > MAX_CACHED).unwrap_or(true);
                if do_provision {
                    provisioned.insert(uuid, now);
                    log::info!("Provisioning {:?}", uuid);
                    match node.management.add_node(uuid).await {
                        Ok(_) => {
                            log::info!("Add node started");
                        }
                        Err(e) => {
                            log::info!("Provisioning failed: {:?}, publishing status", e);
                            let status = BtMeshEvent {
                                status: BtMeshDeviceState::Provisioning {
                                    device: uuid.as_simple().to_string(),
                                    error: Some(e.to_string())
                                }
                            };

                            let data = serde_json::to_string(&status)?;
                            let message = mqtt::Message::new("btmesh", data.as_bytes(), 1);
                            log::info!("Provisioning failed: {:?}, publishing status", e);
                            if let Err(e) = mqtt_client.publish(message).await {
                                log::warn!(
                                    "Error publishing provisioning error status: {:?}",
                                    e
                                );
                            }
                            provisioned.remove(&uuid);
                        }
                    }
                } else {
                    log::warn!("Device {} already provisioned, ignoring", uuid);
                }
            },
            command = commands.recv() => {
                match command {
                    Ok((_, command)) => {
                        if let Ok(data) = serde_json::from_slice::<BtMeshCommand>(&command[..]) {
                            log::info!("Parsed command payload: {:?}", data);
                            match data.command {
                                BtMeshOperation::Provision {
                                    device,
                                } => {
                                    if let Ok(uuid) = Uuid::parse_str(&device) {
                                        provision_tx.send(uuid).await?;
                                    } else {
                                        log::error!("Wrong device uuid {:?}", device);
                                    }
                                }
                                BtMeshOperation::Reset {
                                    address,
                                    device,
                                } => {
                                    configure_tx.send(NodeConfigurationMessage::Reset(device, address, None)).await?;
                                }
                            }
                        }
                    }
                    Err(RecvError::Lagged(n))=> {
                        log::info!("Commanded channel lagged, missed {n} commands");
                    }
                    Err(RecvError::Closed) => {
                        log::warn!("Command channel closed, exiting...");
                        drop(configure_tx);
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
pub enum NodeConfigurationMessage {
    Configure(Uuid, u16),
    Reset(String, u16, Option<String>),
}

#[derive(Debug)]
pub struct NodeConfiguration<'a> {
    pub message: ConfigurationMessage,
    pub path: Path<'a>,
    pub address: u16,
}
