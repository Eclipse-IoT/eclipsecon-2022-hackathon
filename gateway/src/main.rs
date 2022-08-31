#![feature(generic_associated_types)]

use bluer::mesh::{
    application::{Application, ApplicationMessage},
    element::*,
};
use btmesh_models::{
    generic::{
        battery::GenericBatteryClient,
        onoff::{
            GenericOnOffClient, GenericOnOffMessage, GenericOnOffServer, Set as GenericOnOffSet,
        },
    },
    sensor::SensorClient,
    Message, Model,
};
use clap::Parser;
use dbus::Path;
use futures::{pin_mut, StreamExt};
use paho_mqtt as mqtt;
use sensor_model::*;
use serde_json::Value;
use std::{sync::Arc, time::Duration};
use tokio::{signal, sync::mpsc, time::sleep};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    token: String,
    #[clap(long, env, default_value = "ssl://mqtt.sandbox.drogue.cloud:8883")]
    drogue_mqtt_uri: String,
    #[clap(long, env, default_value = "ble-demo")]
    drogue_application: String,
    #[clap(long, env, default_value = "gateway")]
    drogue_device: String,
    #[clap(long, env, default_value = "hey-rodney")]
    drogue_password: String,
    #[clap(long, env, default_value = "/etc/ssl/certs/ca-bundle.crt")]
    ca_path: String,
    #[clap(long, env, parse(try_from_str), default_value = "false")]
    disable_tls: bool,
    #[clap(long, env, parse(try_from_str), default_value = "false")]
    insecure_tls: bool,
}

type Sensor = SensorClient<MicrobitSensorConfig, 1, 1>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let session = bluer::Session::new().await?;

    let mesh = session.mesh().await?;

    let (element_control, element_handle) = element_control();
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

    let node = mesh.attach(root_path.clone(), &args.token).await?;

    let mqtt_uri = args.drogue_mqtt_uri;

    let mqtt_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(mqtt_uri.clone())
        .client_id("btmesh-gateway")
        .persistence(mqtt::PersistenceType::None)
        .finalize();
    let mut mqtt_client = mqtt::AsyncClient::new(mqtt_opts)?;

    let mut conn_opts = mqtt::ConnectOptionsBuilder::new();
    conn_opts.user_name(format!(
        "{}@{}",
        args.drogue_device, args.drogue_application
    ));
    conn_opts.password(args.drogue_password);
    conn_opts.keep_alive_interval(Duration::from_secs(30));
    conn_opts.automatic_reconnect(Duration::from_millis(100), Duration::from_secs(5));

    if !args.disable_tls {
        let ca = args.ca_path;
        let ssl_opts = if args.insecure_tls {
            mqtt::SslOptionsBuilder::new()
                .trust_store(&ca)?
                .enable_server_cert_auth(false)
                .verify(false)
                .finalize()
        } else {
            mqtt::SslOptionsBuilder::new().trust_store(&ca)?.finalize()
        };
        conn_opts.ssl_options(ssl_opts);
    }

    let conn_opts = conn_opts.finalize();

    mqtt_client.set_disconnected_callback(|c, _, _| {
        log::info!("Disconnected");
        let t = c.reconnect();
        if let Err(_e) = t.wait_for(Duration::from_secs(10)) {
            //log::warn!("Error reconnecting to broker ({:?}), exiting", e);
            std::process::exit(1);
        }
    });

    mqtt_client.set_connection_lost_callback(|c| {
        log::info!("Connection lost");
        let t = c.reconnect();
        if let Err(e) = t.wait_for(Duration::from_secs(10)) {
            log::warn!("Error reconnecting to broker ({:?}), exiting", e);
            std::process::exit(1);
        }
    });

    mqtt_client.connect(conn_opts).await?;

    log::info!("Gateway ready. Press Ctrl+C to quit.");
    pin_mut!(element_control);

    let mut commands = mqtt_client.get_stream(100);
    mqtt_client.subscribe("command/inbox/#", 1).await?;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            evt = element_control.next() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ElementMessage::Received(received) => {
                                log::trace!("Received message with opcode {:?} and {} parameter bytes!", received.opcode, received.parameters.len());
                                match SensorClient::<MicrobitSensorConfig, 1, 1>::parse(&received.opcode, &received.parameters).map_err(|_| std::fmt::Error)? {
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
                                println!("Received dev key message: {:?}", received);
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
                            println!("Failed to join: {}", reason);
                            break;
                        },
                    }
                },
                None => break,
            },
            command = commands.next() => {
                if let Some(Some(message)) = command {
                    let topic = message.topic();
                    log::info!("Received on {}: {:?}", topic, message);
                    let mut parts = topic.rsplit("/");
                    if let Some(channel) = parts.next() {
                        if channel == "provisioning" {
                            log::info!("Received provisioning command for gateway: {:?}", message.payload());
                        } else if channel == "sensor" {
                            log::info!("Got message on sensor channel");
                            if let Some(device) = parts.next() {
                                log::info!("Command is for {}", device);
                                // Check if it's a device'y destination
                                if let Ok(destination) = u16::from_str_radix(device, 16) {
                                    log::info!("Destination is {}", destination);
                                    if let Ok(command) = serde_json::from_slice(message.payload()) {
                                        log::info!("Parsed command payload: {:?}", command);
                                        if let Some(raw) = json2command(&command) {
                                            log::info!("Raw command parsed!");
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
                                            match node.send(raw, path, destination, app_key).await {
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
                }
            }
        }
    }

    println!("Shutting down!");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

// Converts JSON message to BLE mesh message
// TODO: This should eventually be done by the model-converter, but support
// calling command hooks in drogue-cloud is not yet available.
fn json2command(data: &Value) -> Option<RawMessage> {
    if let Value::Object(data) = data {
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
            return Some(message);
        }
    }
    None
}
