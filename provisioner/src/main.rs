#![feature(generic_associated_types)]
//! Attach and send/receive BT Mesh messages
//!
use bluer::{
    mesh::{
        application::Application,
        element::*,
        provisioner::{Provisioner, ProvisionerControlHandle, ProvisionerMessage},
    },
    Uuid,
};
use btmesh_common::address::LabelUuid;
use btmesh_models::{
    foundation::configuration::{
        app_key::AppKeyMessage, model_publication::PublishAddress, ConfigurationClient,
        ConfigurationMessage, ConfigurationServer,
    },
    generic::{battery::GENERIC_BATTERY_SERVER, onoff::GENERIC_ONOFF_SERVER},
    sensor::SENSOR_SETUP_SERVER,
    Message, Model,
};
use btmesh_operator::{BtMeshCommand, BtMeshDeviceState, BtMeshEvent, BtMeshOperation};
use clap::Parser;
use clap_num::maybe_hex;
use dbus::Path;
use futures::{pin_mut, StreamExt};
use paho_mqtt as mqtt;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::{
    signal,
    sync::{mpsc, Mutex},
    time::sleep,
};
use tokio_stream::wrappers::ReceiverStream;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    token: String,
    #[clap(long, env, default_value = "ssl://mqtt.sandbox.drogue.cloud:8883")]
    drogue_mqtt_uri: String,
    #[clap(long, env, default_value = "eclipsecon-hackathon")]
    drogue_application: String,
    #[clap(long, env, default_value = "provisioner")]
    drogue_device: String,
    #[clap(long, env, default_value = "hey-rodney")]
    drogue_password: String,
    #[clap(long, env, default_value = "/etc/ssl/certs/ca-bundle.crt")]
    ca_path: String,
    #[clap(long, env, parse(try_from_str), default_value = "false")]
    disable_tls: bool,
    #[clap(long, env, parse(try_from_str), default_value = "false")]
    insecure_tls: bool,
    #[clap(long, parse(try_from_str=maybe_hex))]
    start_address: Option<u32>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let session = bluer::Session::new().await?;

    let mesh = session.mesh().await?;

    let (element_control, element_handle) = element_control();
    let (app_tx, _app_rx) = mpsc::channel(1);

    let root_path = Path::from("/mesh/cfgclient");
    let app_path = Path::from(format!("{}/{}", root_path.clone(), "application"));
    let element_path = Path::from(format!("{}/{}", root_path.clone(), "ele00"));

    let (prov_tx, prov_rx) = mpsc::channel(1);

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
            start_address: args.start_address.unwrap_or(0xbf) as i32,
        }),
        events_tx: app_tx,
    };

    let registered = mesh.application(root_path.clone(), sim).await?;

    let node = mesh.attach(root_path.clone(), &args.token).await?;

    //node.management.add_node(Uuid::parse_str(&args.uuid)?).await?;

    let mqtt_uri = args.drogue_mqtt_uri;

    let mqtt_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(mqtt_uri.clone())
        .client_id("btmesh-provisioner")
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
    log::info!("Provisioner ready. Press Ctrl+C to quit.");

    let mut prov_stream = ReceiverStream::new(prov_rx);
    pin_mut!(element_control);
    let mut commands = mqtt_client.get_stream(100);
    mqtt_client.subscribe("command/inbox/#", 1).await?;

    let provisioning: Mutex<HashSet<Uuid>> = Mutex::new(HashSet::new());

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            evt = prov_stream.next() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ProvisionerMessage::AddNodeComplete(uuid, unicast, count) => {
                                println!("Successfully added node {:?} to the address {:#04x} with {:?} elements", uuid, unicast, count);

                                sleep(Duration::from_secs(5)).await;

                                println!("Add app key");
                                node.add_app_key(element_path.clone(), unicast, 0, 0, false).await?;
                                sleep(Duration::from_secs(3)).await;
                                println!("Bind sensor server");
                                node.bind(element_path.clone(), unicast, 0, SENSOR_SETUP_SERVER).await?;
                                sleep(Duration::from_secs(3)).await;
                                println!("Bind onoff server");
                                node.bind(element_path.clone(), unicast, 0, GENERIC_ONOFF_SERVER).await?;
                                sleep(Duration::from_secs(3)).await;
                                println!("Bind battery server");
                                node.bind(element_path.clone(), unicast, 0, GENERIC_BATTERY_SERVER).await?;
                                sleep(Duration::from_secs(3)).await;

                                // let label = LabelUuid {
                                //     uuid: Uuid::parse_str("f0bfd803cde184133096f003ea4a3dc2")?.into_bytes(),
                                //     address: VirtualAddress::new(8f32 as u16).map_err(|_| std::fmt::Error)?
                                // };
                                let label = LabelUuid::new(Uuid::parse_str("f0bfd803cde184133096f003ea4a3dc2")?.into_bytes()).map_err(|_| std::fmt::Error)?;
                                let pub_address = PublishAddress::Virtual(label);
                                println!("Add pub-set for sensor server");
                                node.pub_set(element_path.clone(), unicast, pub_address, 0, 0x29, 5, SENSOR_SETUP_SERVER).await?;
                                sleep(Duration::from_secs(3)).await;
                                println!("Add pub-set for battery server");
                                node.pub_set(element_path.clone(), unicast, pub_address, 0, 0x29, 5, GENERIC_BATTERY_SERVER).await?;
                                sleep(Duration::from_secs(3)).await;


                                let topic = format!("btmesh/{}", uuid.as_simple().to_string());
                                println!("Sending message to topic {}", topic);
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
                                println!("Failed to add node {:?}: '{:?}'", uuid, reason);

                                 let status = BtMeshEvent {
                                   status: BtMeshDeviceState::Provisioning { error: Some(reason) }
                                 };

                                let topic = format!("btmesh/{}", uuid.as_simple().to_string());
                                println!("Sending message to topic {}", topic);
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
            evt = element_control.next() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ElementMessage::Received(received) => {
                                println!("Received element message: {:?}", received);
                            },
                            ElementMessage::DevKey(received) => {
                                println!("Received dev key message: {:?}", received);
                                match ConfigurationServer::parse(&received.opcode, &received.parameters).map_err(|_| std::fmt::Error)? {
                                    Some(message) => {
                                        match message {
                                            ConfigurationMessage::AppKey(key) => {
                                                match key {
                                                    AppKeyMessage::Status(status) => {
                                                        println!("Received keys {:?} {:?}", status.indexes.net_key(), status.indexes.app_key());
                                                    },
                                                    _ => println!("Received key message {:?}", key.opcode()),
                                                }
                                            },
                                            _ => {
                                                println!("Received configuration message {:?}", message.opcode());
                                            }
                                        }
                                    },
                                    None => {
                                        println!("Received no configuration message");
                                    },
                                }
                            }
                        }
                    },
                    None => break,
                }
            },
            command = commands.next() => {
                if let Some(Some(message)) = command {
                    let topic = message.topic();
                    log::info!("Received on {}: {:?}", topic, message);
                    let mut parts = topic.rsplit("/");
                    if let Some(channel) = parts.next() {
                        if channel == "btmesh" {
                            log::info!("Received btmesh command: {:?}", message.payload());
                            if let Ok(data) = serde_json::from_slice::<BtMeshCommand>(message.payload()) {
                                log::info!("Parsed command payload: {:?}", data);
                                match data.command {
                                    BtMeshOperation::Provision {
                                        device
                                    } => {
                                        if let Ok(uuid) = Uuid::parse_str(&device) {
                                            // Ensuring that we don't provision multiple times
                                            let doit = {
                                                let mut set = provisioning.lock().await;
                                                if !set.contains(&uuid) {
                                                    set.insert(uuid.clone());
                                                    true
                                                } else {
                                                    false
                                                }
                                            };
                                            if doit {
                                                log::info!("Provisioning {:?}", device);
                                                match node.management.add_node(uuid).await {
                                                    Ok(_) => {
                                                        // Handled in AddNodeComplete
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
                                        }
                                    }
                                    BtMeshOperation::Reset {
                                        address
                                    } => {
                                        if let Some(device) = parts.next() {
                                            let topic = "btmesh";
                                            log::info!("Resetting device, publishing response to {}", topic);
                                            let error = match node.reset(element_path.clone(), address).await {
                                                Ok(_) => {
                                                    None
                                                }
                                                Err(e) => {
                                                    Some(e.to_string())
                                                }
                                            };
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
                        }
                    }
                }
            },
        }
    }

    println!("Shutting down");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
