#![feature(generic_associated_types)]

use bluer::mesh::{
    application::{Application, ApplicationMessage},
    element::*,
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
    sensor::{SensorMessage, SensorSetupMessage, SensorStatus},
    Model,
};
use clap::Parser;
use dbus::Path;
use futures::StreamExt;
use sensor_model::*;
use std::{sync::Arc, time::Duration};
use tokio::{signal, sync::mpsc, time::sleep};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    token: String,
    #[clap(short, long, default_value_t = 10)]
    publish_interval: u64,
}

type Sensor = SensorServer;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let session = bluer::Session::new().await?;

    let mesh = session.mesh().await?;

    let (mut element_control, element_handle) = element_control();
    let (app_tx, mut app_rx) = mpsc::channel(1);

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

    let node = mesh.attach(root_path.clone(), &args.token).await?;

    println!("Device ready. Press Ctrl+C to quit.");

    let mut interval = tokio::time::interval(Duration::from_secs(args.publish_interval));
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            _ = interval.tick() => {
                let battery = GenericBatteryMessage::Status(GenericBatteryStatus::new(0, 0, 0, GenericBatteryFlags {
                    presence: GenericBatteryFlagsPresence::NotPresent,
                    indicator: GenericBatteryFlagsIndicator::Unknown,
                    charging: GenericBatteryFlagsCharging::NotChargeable
                }));

                let data = SensorPayload {
                    temperature: 22,
                    acceleration: Default::default(),
                };

                let sensor = SensorMessage::Status(SensorStatus::new(data));
                println!("Publishing battery status");
                node.publish::<GenericBatteryServer>(battery, front.clone()).await?;

                println!("Publishing sensor status");
                node.publish::<Sensor>(SensorSetupMessage::Sensor(sensor), front.clone()).await?;
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
            app_evt = app_rx.recv() => match app_evt {
                Some(msg) => {
                    match msg {
                        ApplicationMessage::JoinComplete(token) => {
                            println!("Joined with token {:016x}", token);
                            println!("Attaching");
                            let _node = mesh.attach(root_path.clone(), &format!("{:016x}", token)).await?;
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

    println!("Shutting down!");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
