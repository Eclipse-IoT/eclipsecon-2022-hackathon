#![feature(generic_associated_types)]

use bluer::{
    mesh::{
        application::{Application, ApplicationMessage},
        element::*,
        node::Node,
    },
    Uuid,
};
use btmesh_models::{
    generic::{
        battery::{
            GenericBatteryFlags, GenericBatteryFlagsCharging, GenericBatteryFlagsIndicator,
            GenericBatteryFlagsPresence, GenericBatteryMessage, GenericBatteryServer,
            GenericBatteryStatus,
        },
        onoff::{GenericOnOffClient, GenericOnOffMessage, GenericOnOffServer},
    },
    sensor::SensorStatus,
    Model,
};
use clap::Parser;
use dbus::Path;
use futures::StreamExt;
use sensor_model::*;
use std::{sync::Arc, time::Duration};
use tokio::{signal, sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use serde_derive::{Serialize, Deserialize};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    token: Option<String>,
    #[clap(short, long, conflicts_with = "token")]
    join: bool,
    #[clap(short, long, default_value_t = 10)]
    publish_interval: u64,
    #[clap(short, long, conflicts_with = "token")]
    config: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct NodeConfig {
    token:  Option<String>,
    unicast_address: Option<String>,
}

type Sensor = SensorServer;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let session = bluer::Session::new().await?;

    let mesh = session.mesh().await?;

    let (mut element_control, element_handle) = element_control();
    let (app_tx, app_rx) = mpsc::channel(1);

    let root_path = Path::from("/simulator");
    let app_path = Path::from(format!("{}/{}", root_path.clone(), "application"));

    let front = Path::from(format!("{}/ele{}", root_path.clone(), "00"));
    let left = Path::from(format!("{}/ele{}", root_path.clone(), "01"));
    let right = Path::from(format!("{}/ele{}", root_path.clone(), "02"));
    let sim = Application {
        path: app_path,
        elements: vec![
            Element {
                path: front.clone(),
                location: Some(0x0100),
                models: vec![
                    Arc::new(FromDrogue::new(GenericOnOffServer)),
                    Arc::new(FromDrogue::new(GenericBatteryServer)),
                    Arc::new(FromDrogue::new(Sensor::new())),
                ],
                control_handle: Some(element_handle.clone()),
            },
            Element {
                path: left,
                location: Some(0x010D),
                models: vec![Arc::new(FromDrogue::new(GenericOnOffClient))],
                control_handle: Some(element_handle.clone()),
            },
            Element {
                path: right,
                location: Some(0x010E),
                models: vec![Arc::new(FromDrogue::new(GenericOnOffClient))],
                control_handle: Some(element_handle),
            },
        ],
        events_tx: app_tx,
        provisioner: None,
    };

    let registered = mesh.application(root_path.clone(), sim).await?;

    let mut node: Option<Node> = None;
    let mut cfg: NodeConfig = confy::load("node", None)?;
    let file = confy::get_configuration_file_path("node", None)?;
    println!("The configuration file path is: {:#?}", file);

    match args.token {
        Some(token) => {
            println!("Attaching with token {}", token);
            node = Some(mesh.attach(root_path.clone(), &token).await?);
        }
        None => {
            match cfg.token {
                Some(tk) => {
                    println!("Attaching with token {}", tk);
                    node = Some(mesh.attach(root_path.clone(), &tk).await?);
                },
                None => {
                    let device_id = Uuid::new_v4();
                    println!("Joining device: {}", device_id.as_simple());
                    mesh.join(root_path.clone(), device_id).await?;
                }
            }
        }
    }

    println!("Device ready. Press Ctrl+C to quit.");

    let mut interval = tokio::time::interval(Duration::from_secs(args.publish_interval));
    let mut app_stream = ReceiverStream::new(app_rx);
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            _ = interval.tick() => {
                if let Some(ref node) = node {
                    let battery = GenericBatteryMessage::Status(GenericBatteryStatus::new(0, 0, 0, GenericBatteryFlags {
                        presence: GenericBatteryFlagsPresence::NotPresent,
                        indicator: GenericBatteryFlagsIndicator::Unknown,
                        charging: GenericBatteryFlagsCharging::NotChargeable
                    }));

                    let data = SensorPayload {
                        temperature: 22,
                        acceleration: Default::default(),
                        noise: 0,
                    };

                    let sensor = SensorMessage::Status(SensorStatus::new(data));
                    println!("Publishing battery status");
                    node.publish::<GenericBatteryServer>(battery, front.clone()).await?;

                    println!("Publishing sensor status");
                    node.publish::<Sensor>(sensor, front.clone()).await?;
                }
            }
            evt = element_control.next() => {
                match evt {
                    Some(msg) => {
                        match msg {
                            ElementMessage::Received(received) => {
                                if let Ok(Some(GenericOnOffMessage::Set(m))) = GenericOnOffServer::parse(&received.opcode, &received.parameters) {
                                    if m.on_off == 1 {
                                        println!("Turn ON");
                                    } else {
                                        println!("Turn OFF");
                                    }
                                }
                            },
                            ElementMessage::DevKey(received) => {
                                println!("Received dev key message: {:?}", received);
                            }
                        }
                    },
                    None => break,
                }
                println!("Got message?!");
            }
            app_evt = app_stream.next() => {
                match app_evt {
                    Some(msg) => {
                        match msg {
                            ApplicationMessage::JoinComplete(token) => {
                                println!("Joined with token {:016x}", token);
                                cfg.token = Some(format!("{:016x}", token));
                                confy::store("node",None, &cfg)?;
                                println!("Attaching");
                                node = Some(mesh.attach(root_path.clone(), &format!("{:016x}", token)).await?);
                            },
                            ApplicationMessage::JoinFailed(reason) => {
                                println!("Failed to join: {}", reason);
                                break;
                            },
                        }
                    },
                    None => break,
                }
            }
        }
    }

    println!("Shutting down!");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
