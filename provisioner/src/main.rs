#![feature(generic_associated_types)]
//! Attach and send/receive BT Mesh messages
//!
//! Example meshd
//! [burrboard/gateway]$ sudo /usr/libexec/bluetooth/bluetooth-meshd --config ${PWD}/deploy/bluez/example/meshcfg --storage ${PWD}/deploy/bluez/example/mesh --debug
//!
//! Example device join
//! [burrboard/gateway]$ app/temp-device.py join
//!
//! Example provisioner
//! [bluer]$ RUST_LOG=TRACE cargo run -- --token 84783e12f11c4dcd

use bluer::{
    mesh::{
        application::Application,
        element::*,
        provisioner::{Provisioner, ProvisionerControlHandle, ProvisionerMessage},
    },
    Uuid,
};
use btmesh_models::{
    foundation::configuration::{
        app_key::AppKeyMessage, ConfigurationClient, ConfigurationMessage, ConfigurationServer,
    },
    Message, Model,
};
use clap::Parser;
use dbus::Path;
use futures::{pin_mut, StreamExt};
use paho_mqtt as mqtt;
use serde_json::Value;
use std::{sync::Arc, time::Duration};
use tokio::{signal, sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;

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
            start_address: 0xbd,
        }),
        events_tx: app_tx,
    };

    let registered = mesh.application(root_path.clone(), sim).await?;

    let node = mesh.attach(root_path.clone(), &args.token).await?;

    //node.management.add_node(Uuid::parse_str(&args.uuid)?).await?;

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

    let mut prov_stream = ReceiverStream::new(prov_rx);
    pin_mut!(element_control);
    let mut commands = mqtt_client.get_stream(100);
    mqtt_client.subscribe("command/inbox/#", 1).await?;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            evt = prov_stream.next() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ProvisionerMessage::AddNodeComplete(uuid, unicast, count) => {
                                println!("Successfully added node {:?} to the address {:#04x} with {:?} elements", uuid, unicast, count);

                                sleep(Duration::from_secs(1)).await;
                                node.add_app_key(element_path.clone(), unicast, 0, 0, false).await?;

                                // example composition get
                                // let message = ConfigurationMessage::CompositionData(CompositionDataMessage::Get(0));
                                // node.dev_key_send::<ConfigurationServer>(message, element_path.clone(), unicast, true, 0 as u16).await?;

                                // example bind
                                // let payload = ModelAppPayload {
                                //     element_address: unicast.try_into().map_err(|_| ReqError::Failed)?,
                                //     app_key_index: AppKeyIndex::new(0),
                                //     model_identifier: SENSOR_SERVER,
                                // };

                                // let message = ConfigurationMessage::from(ModelAppMessage::Bind(payload));
                                // node.dev_key_send::<ConfigurationServer>(message, element_path.clone(), unicast, true, 0 as u16).await?;
                            },
                            ProvisionerMessage::AddNodeFailed(uuid, reason) => {
                                println!("Failed to add node {:?}: '{:?}'", uuid, reason);
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
                                                        println!("Received keys {:?} {:?}", status.indexes.net_key(), status.indexes.app_key())
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
                        if channel == "provision" {
                            log::info!("Received provisioning command for gateway: {:?}", message.payload());
                            if let Ok(data) = serde_json::from_slice::<Value>(message.payload()) {
                                log::info!("Parsed command payload: {:?}", data);
                                let device = data.get("device");
                                match device {
                                    Some(uuid) => {
                                        log::info!("Provisioning {:?}", uuid);
                                        node.management.add_node(Uuid::parse_str(uuid.as_str().unwrap())?).await?
                                    },
                                    _ => log::error!("No uuid provided")
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