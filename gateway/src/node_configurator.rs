use std::time::Duration;

use bluer::mesh::{
    element::{ElementControl, ElementMessage},
    node::Node,
};
use btmesh_models::foundation::configuration::ConfigurationClient;
use btmesh_operator::{BtMeshDeviceState, BtMeshEvent};
use paho_mqtt as mqtt;
use tokio::sync::mpsc::Receiver;

use crate::provisioner::NodeConfigurationMessage;
use btmesh_models::Model;
use futures::StreamExt;

pub async fn run<'a>(
    mut config_rx: Receiver<NodeConfigurationMessage<'a>>,
    mqtt_client: mqtt::AsyncClient,
    mut element_control: ElementControl,
    node: Node,
) -> Result<(), anyhow::Error> {
    loop {
        match config_rx.recv().await {
            Some(conf) => {
                match conf {
                    NodeConfigurationMessage::Configure(msg) => {
                        // TODO appkey?
                        node.dev_key_send(&msg.message, msg.path.clone(), msg.address, true, 0)
                            .await?;
                        let mut retries = 0;
                        loop {
                            if retries >= 3 {
                                log::error!("Failed to configure node");
                                // TODO send failure back to cloud
                                // TODO cache node address and discard other configuration messages
                                break;
                            };
                            let response = tokio::time::timeout(
                                Duration::from_secs(10),
                                element_control.next(),
                            )
                            .await;
                            match response {
                                Ok(Some(msg)) => match msg {
                                    ElementMessage::Received(received) => {
                                        log::info!("Received element message: {:?}", received);
                                    }
                                    ElementMessage::DevKey(received) => {
                                        log::info!(
                                            "Received devkey message with opcode {:?}",
                                            received.opcode
                                        );
                                        // TODO parse the message and make sure we got the right one (and indication of success) before continuing
                                        // match ConfigurationClient::parse(
                                        //     &received.opcode,
                                        //     &received.parameters,
                                        // )
                                        // .map_err(|_| std::fmt::Error)?
                                        // {
                                        //     Some(message) => {
                                        //         log::info!(
                                        //             "Received configuration message: {:?}",
                                        //             message
                                        //         );
                                        //     }
                                        //     None => {}
                                        // }
                                        break;
                                    }
                                },
                                Ok(None) => {
                                    log::info!("No element message received");
                                }
                                Err(_) => {
                                    log::info!(
                                        "Timeout waiting for the configuration message repsone"
                                    );
                                    node.dev_key_send(
                                        &msg.message,
                                        msg.path.clone(),
                                        msg.address,
                                        true,
                                        0,
                                    )
                                    .await?;
                                    retries += 1;
                                }
                            }
                        }
                    }
                    NodeConfigurationMessage::Finish(uuid, address) => {
                        log::info!("Finished configuring {:?}", uuid);
                        let topic = format!("btmesh/{}", uuid.as_simple().to_string());
                        log::info!("Sending message to topic {}", topic);
                        let status = BtMeshEvent {
                            status: BtMeshDeviceState::Provisioned { address },
                        };

                        let data = serde_json::to_string(&status)?;
                        let message = mqtt::Message::new(topic, data.as_bytes(), 1);
                        if let Err(e) = mqtt_client.publish(message).await {
                            log::warn!("Error publishing provisioning status: {:?}", e);
                        }
                    }
                    NodeConfigurationMessage::Reset(device, address, error) => {
                        let topic = "btmesh";
                        log::info!(
                            "Resetting device {} (address {}), publishing response",
                            device,
                            address
                        );
                        let status = BtMeshEvent {
                            status: BtMeshDeviceState::Reset {
                                error,
                                device: device.to_string(),
                            },
                        };

                        let data = serde_json::to_string(&status)?;
                        let message = mqtt::Message::new(topic, data.as_bytes(), 1);
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
            }
            None => {
                log::info!("No configuration message received");
                break;
            }
        }
    }
    Ok(())
}
