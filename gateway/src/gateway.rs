use bluer::mesh::{
    application::{Application, ApplicationMessage},
    element::*,
    network::Network,
};
use btmesh_models::{
    self,
    generic::{
        battery::GenericBatteryClient,
        onoff::{
            GenericOnOffClient, GenericOnOffMessage, GenericOnOffServer, Set as GenericOnOffSet,
        },
    },
    Message, Model,
};
use dbus::Path;
use futures::StreamExt;
use paho_mqtt as mqtt;
use sensor_model::*;
use serde_json::Value;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};

type Sensor = SensorClient;

pub struct Config {
    token: String,
}

impl Config {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

pub async fn run(
    mesh: Network,
    config: Config,
    mut commands: broadcast::Receiver<(String, Vec<u8>)>,
    mqtt_client: mqtt::AsyncClient,
) -> Result<(), anyhow::Error> {
    let (mut element_control, element_handle) = element_control();
    let (app_tx, mut app_rx) = mpsc::channel(1);

    let root_path = Path::from("/gateway");
    let app_path = Path::from(format!("{}/{}", root_path.clone(), "application"));

    let front = Path::from(format!("{}/ele{}", root_path.clone(), "00"));
    let left = Path::from(format!("{}/ele{}", root_path.clone(), "01"));
    let right = Path::from(format!("{}/ele{}", root_path.clone(), "02"));

    let front_loc = 0x0100;
    let left_loc = 0x010D;
    let right_loc = 0x010E;

    let sim = Application {
        path: app_path,
        elements: vec![
            Element {
                path: front.clone(),
                location: Some(front_loc),
                models: vec![
                    Arc::new(FromDrogue::new(GenericOnOffClient)),
                    Arc::new(FromDrogue::new(GenericBatteryClient)),
                    Arc::new(FromDrogue::new(Sensor::new())),
                ],
                control_handle: Some(element_handle.clone()),
            },
            Element {
                path: left.clone(),
                location: Some(left_loc),
                models: vec![Arc::new(FromDrogue::new(GenericOnOffServer))],
                control_handle: Some(element_handle.clone()),
            },
            Element {
                path: right.clone(),
                location: Some(right_loc),
                models: vec![Arc::new(FromDrogue::new(GenericOnOffServer))],
                control_handle: Some(element_handle),
            },
        ],
        events_tx: app_tx,
        provisioner: None,
    };

    let registered = mesh.application(root_path.clone(), sim).await?;

    let node = mesh.attach(root_path.clone(), &config.token).await?;

    log::info!("Starting gateway event loop");
    loop {
        tokio::select! {
            evt = element_control.next() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ElementMessage::Received(received) => {
                                log::trace!("Received message with opcode {:?} and {} parameter bytes!", received.opcode, received.parameters.len());
                                match SensorClient::parse(&received.opcode, &received.parameters).map_err(|_| std::fmt::Error)? {
                                    Some(message) => {
                                        log::trace!("Received {:?}", message);
                                    },
                                    None => {}
                                }

                                match GenericBatteryClient ::parse(&received.opcode, &received.parameters).map_err(|_| std::fmt::Error)? {
                                    Some(message) => {
                                        log::trace!("Received {:?}", message);
                                    },
                                    None => {}
                                }
                                let mut opcode: heapless::Vec<u8, 16> = heapless::Vec::new();
                                received.opcode.emit(&mut opcode).map_err(|_| std::fmt::Error)?;

                                let mut parameters = Vec::new();
                                parameters.extend_from_slice(&received.parameters);
                                let message = RawMessage {
                                    location: received.location.unwrap(),
                                    opcode: opcode.to_vec(),
                                    parameters,
                                };
                                let data = serde_json::to_string(&message)?;

                                let src = received.src.as_bytes();
                                let topic = format!("sensor/{:02x}{:02x}", src[0], src[1]);

                                let message = mqtt::Message::new(topic, data.as_bytes(), 1);
                                if let Err(e) = mqtt_client.publish(message).await {
                                    log::warn!(
                                        "Error publishing command back to device: {:?}",
                                        e
                                    );
                                }
                            },
                            ElementMessage::DevKey(received) => {
                                log::info!("Gateway Received dev key message: {:?}", received);
                            }
                        }
                    },
                    None => break,
                }
            },
            app_evt = app_rx.recv() => match app_evt {
                Some(msg) => {
                    match msg {
                        ApplicationMessage::JoinComplete(token) => {
                            log::debug!("Joined with token {:016x}", token);
                            // TODO: When provisioning works?
                            //_node = mesh.attach(root_path.clone(), &format!("{:016x}", token)).await?;
                        },
                        ApplicationMessage::JoinFailed(reason) => {
                            log::info!("Failed to join: {}", reason);
                            break;
                        },
                    }
                },
                None => break,
            },
            command = commands.recv() => {
                match command {
                    Ok((topic, payload)) => {
                        let mut parts = topic.rsplit("/");
                        if let Some(channel) = parts.next() {
                            if channel == "sensor" {
                                log::info!("Got message on sensor channel");
                                if let Some(device) = parts.next() {
                                    log::info!("Command is for {}", device);
                                    // Check if it's a device'y destination
                                    if let Ok(command) = serde_json::from_slice(&payload[..]) {
                                        log::info!("Parsed command payload: {:?}", command);
                                        if let Some((address, raw)) = json2command(&command) {
                                            log::info!("Destination is {}", address);
                                            let path = if raw.location == front_loc {
                                                front.clone()
                                            } else if raw.location == left_loc {
                                                left.clone()
                                            } else if raw.location == right_loc {
                                                right.clone()
                                            } else {
                                                front.clone()
                                            };
                                            // TODO: Hmm, where to get this?
                                            let app_key = 0;
                                            match node.send(raw, path, address, app_key).await {
                                                Ok(_) => {
                                                    log::info!("Forwarded message to device");
                                                }
                                                Err(e) => {
                                                    log::warn!("Error forwarding message to device: {:?}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        log::info!("Got error waiting for event");
                        break
                    }
                }
            }
        }
    }

    log::info!("Shutting down gateway!");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

// Converts JSON message to BLE mesh message
// TODO: This should eventually be done by the model-converter, but support
// calling command hooks in drogue-cloud is not yet available.
fn json2command(data: &Value) -> Option<(u16, RawMessage)> {
    if let Value::Object(data) = data {
        if let Some(Value::Number(address)) = data.get("address") {
            if let Some(Value::Object(state)) = data.get("display") {
                let location = state["location"].as_u64().unwrap_or(0);
                let on = state["on"].as_bool().unwrap_or(false);
                let set = GenericOnOffSet {
                    on_off: if on { 1 } else { 0 },
                    tid: 0,
                    transition_time: None,
                    delay: None,
                };
                let msg = GenericOnOffMessage::Set(set);

                let mut opcode: heapless::Vec<u8, 16> = heapless::Vec::new();
                msg.opcode().emit(&mut opcode).unwrap();

                let mut parameters: heapless::Vec<u8, 386> = heapless::Vec::new();
                msg.emit_parameters(&mut parameters).unwrap();
                let message = RawMessage {
                    location: location as u16,
                    opcode: opcode.to_vec(),
                    parameters: parameters.to_vec(),
                };
                return Some((address.as_u64().unwrap() as u16, message));
            }
        }
    }
    None
}
