use bluer::{
    mesh::{
        element::{ElementControl, ElementMessage},
        node::Node,
    },
    Uuid,
};
use btmesh_common::address::LabelUuid;
use btmesh_models::{
    foundation::configuration::{
        model_publication::{PublishAddress, PublishPeriod, PublishRetransmit, Resolution},
        node_reset::NodeResetMessage,
        ConfigurationClient, ConfigurationMessage,
    },
    generic::{battery::GENERIC_BATTERY_SERVER, onoff::GENERIC_ONOFF_SERVER},
    sensor::SENSOR_SETUP_SERVER,
};
use btmesh_operator::{BtMeshDeviceState, BtMeshEvent};
use dbus::Path;
use paho_mqtt as mqtt;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;

use crate::provisioner::{NodeConfiguration, NodeConfigurationMessage};
use btmesh_models::Model;
use futures::StreamExt;

async fn send_recv(
    node: &Node,
    element_control: &mut ElementControl,
    msg: NodeConfiguration<'_>,
) -> Result<(), anyhow::Error> {
    // TODO appkey?
    node.dev_key_send(&msg.message, msg.path.clone(), msg.address, true, 0)
        .await?;
    let mut retries = 0;
    loop {
        if retries >= 3 {
            log::error!("Failed to configure node");
            // TODO send failure back to cloud
            // TODO cache node address and discard other configuration messages
            return Err(anyhow::anyhow!("Error configuring node"));
        };
        let response = tokio::time::timeout(Duration::from_secs(10), element_control.next()).await;
        match response {
            Ok(Some(msg)) => match msg {
                ElementMessage::Received(received) => {
                    log::info!("Received element message: {:?}", received);
                }
                ElementMessage::DevKey(received) => {
                    log::info!("Received devkey message with opcode {:?}", received.opcode);
                    // TODO parse the message and make sure we got the right one (and indication of success) before continuing
                    match ConfigurationClient::parse(&received.opcode, &received.parameters)
                        .map_err(|_| std::fmt::Error)?
                    {
                        Some(message) => {
                            log::info!("Received configuration message: {:?}", message);
                        }
                        None => {}
                    }
                    return Ok(());
                }
            },
            Ok(None) => {
                log::info!("No element message received");
            }
            Err(_) => {
                log::info!("Timeout waiting for the configuration message repsone");
                node.dev_key_send(&msg.message, msg.path.clone(), msg.address, true, 0)
                    .await?;
                retries += 1;
            }
        }
    }
}

async fn configure(
    node: &Node,
    element_control: &mut ElementControl,
    element_path: &Path<'_>,
    _uuid: &str,
    unicast: u16,
) -> Result<(), anyhow::Error> {
    log::info!("Add app key");
    node.add_app_key(element_path.clone(), unicast, 0, 0, false)
        .await?;

    log::info!("Bind sensor server");
    let msg = Node::bind_create(unicast, 0, SENSOR_SETUP_SERVER)?;
    send_recv(
        node,
        element_control,
        NodeConfiguration {
            message: msg,
            path: element_path.clone(),
            address: unicast,
        },
    )
    .await?;

    log::info!("Bind onoff server");
    let msg = Node::bind_create(unicast, 0, GENERIC_ONOFF_SERVER)?;
    send_recv(
        node,
        element_control,
        NodeConfiguration {
            message: msg,
            path: element_path.clone(),
            address: unicast,
        },
    )
    .await?;

    log::info!("Bind battery server");
    let msg = Node::bind_create(unicast, 0, GENERIC_BATTERY_SERVER)?;
    send_recv(
        node,
        element_control,
        NodeConfiguration {
            message: msg,
            path: element_path.clone(),
            address: unicast,
        },
    )
    .await?;

    let label = LabelUuid::new(Uuid::parse_str("f0bfd803cde184133096f003ea4a3dc2")?.into_bytes())
        .map_err(|_| std::fmt::Error)?;
    let pub_address = PublishAddress::Label(label);

    log::info!("Add pub-set for sensor server");
    let msg = Node::pub_set_create(
        unicast,
        pub_address,
        0,
        PublishPeriod::new(4, Resolution::Seconds1),
        PublishRetransmit::from(0),
        SENSOR_SETUP_SERVER,
    )?;
    send_recv(
        node,
        element_control,
        NodeConfiguration {
            message: msg,
            path: element_path.clone(),
            address: unicast,
        },
    )
    .await?;
    log::info!("Add pub-set for battery server");
    let msg = Node::pub_set_create(
        unicast,
        pub_address,
        0,
        PublishPeriod::new(60, Resolution::Seconds1),
        PublishRetransmit::from(0),
        GENERIC_BATTERY_SERVER,
    )?;
    send_recv(
        node,
        element_control,
        NodeConfiguration {
            message: msg,
            path: element_path.clone(),
            address: unicast,
        },
    )
    .await?;
    Ok(())
}

pub async fn run<'a>(
    mut config_rx: Receiver<NodeConfigurationMessage>,
    mqtt_client: mqtt::AsyncClient,
    element_path: Path<'a>,
    mut element_control: ElementControl,
    node: Node,
) -> Result<(), anyhow::Error> {
    loop {
        match config_rx.recv().await {
            Some(conf) => match conf {
                NodeConfigurationMessage::Configure(uuid, unicast) => {
                    log::info!("Configuring node {:?} (address {:#04x})", uuid, unicast);

                    let uuid = uuid.as_simple().to_string();
                    let status =
                        match configure(&node, &mut element_control, &element_path, &uuid, unicast)
                            .await
                        {
                            Ok(_) => BtMeshEvent {
                                status: BtMeshDeviceState::Provisioned {
                                    device: uuid.clone(),
                                    address: unicast,
                                },
                            },
                            Err(e) => BtMeshEvent {
                                status: BtMeshDeviceState::Provisioning {
                                    device: uuid.clone(),
                                    error: Some(e.to_string()),
                                },
                            },
                        };

                    log::info!(
                        "Finished configuring {:?} assigned address {:04x}. Status: {:?}",
                        uuid,
                        unicast,
                        status,
                    );

                    let data = serde_json::to_string(&status)?;
                    let message = mqtt::Message::new("btmesh", data.as_bytes(), 1);
                    if let Err(e) = mqtt_client.publish(message).await {
                        log::warn!("Error publishing provisioning status: {:?}", e);
                    }
                }
                NodeConfigurationMessage::Reset(device, address, error) => {
                    log::info!(
                        "Resetting device {} (address {}), publishing response",
                        device,
                        address
                    );

                    let msg = ConfigurationMessage::from(NodeResetMessage::Reset);
                    let msg = NodeConfiguration {
                        message: msg,
                        path: element_path.clone(),
                        address,
                    };

                    if let Ok(_) = send_recv(&node, &mut element_control, msg).await {
                        let status = BtMeshEvent {
                            status: BtMeshDeviceState::Reset {
                                error,
                                device: device.to_string(),
                            },
                        };

                        let data = serde_json::to_string(&status)?;
                        let message = mqtt::Message::new("btmesh", data.as_bytes(), 1);
                        if let Err(e) = mqtt_client.publish(message).await {
                            log::warn!("Error publishing reset status: {:?}", e);
                        }
                        log::info!(
                            "Device {} (address {}) reset status published",
                            device,
                            address
                        );
                    }
                }
            },
            None => {
                log::info!("No configuration message received");
                break;
            }
        }
    }
    Ok(())
}
