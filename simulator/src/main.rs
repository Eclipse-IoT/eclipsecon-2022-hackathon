#![feature(generic_associated_types)]

use bluer::mesh::{application::Application, element::*};
use btmesh_models::{
    generic::{
        battery::{
            GenericBatteryFlags, GenericBatteryFlagsCharging, GenericBatteryFlagsIndicator,
            GenericBatteryFlagsPresence, GenericBatteryMessage, GenericBatteryServer,
            Status as GenericBatteryStatus,
        },
        onoff::{GenericOnOffClient, GenericOnOffMessage, GenericOnOffServer},
    },
    sensor::{SensorMessage, SensorSetupMessage, SensorSetupServer, SensorStatus},
    Model,
};
use clap::Parser;
use dbus::Path;
use futures::StreamExt;
use sensor_model::*;
use std::{sync::Arc, time::Duration};
use tokio::{signal, time::sleep};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    token: String,
    #[clap(short, long, default_value_t = 10)]
    publish_interval: u64,
}

type Sensor = SensorSetupServer<MicrobitSensorConfig, 1, 1>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let session = bluer::Session::new().await?;

    let mesh = session.mesh().await?;

    let (mut element_control, element_handle) = element_control();

    let root_path = Path::from("/mesh_client");
    let app_path = Path::from(format!("{}/{}", root_path.clone(), "application"));

    let front = Path::from(format!("{}/{}", root_path.clone(), "front"));
    let left = Path::from(format!("{}/{}", root_path.clone(), "left"));
    let right = Path::from(format!("{}/{}", root_path.clone(), "right"));
    let sim = Application {
        path: app_path,
        elements: vec![
            Element {
                path: front.clone(),
                models: vec![
                    Arc::new(FromDrogue::new(GenericOnOffServer)),
                    Arc::new(FromDrogue::new(GenericBatteryServer)),
                    Arc::new(FromDrogue::new(Sensor::new())),
                ],
                control_handle: Some(element_handle.clone()),
            },
            Element {
                path: left,
                models: vec![Arc::new(FromDrogue::new(GenericOnOffClient))],
                control_handle: Some(element_handle.clone()),
            },
            Element {
                path: right,
                models: vec![Arc::new(FromDrogue::new(GenericOnOffClient))],
                control_handle: Some(element_handle),
            },
        ],
        ..Default::default()
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
                    temperature: 22
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
                        if let Ok(Some(GenericOnOffMessage::Set(m))) = GenericOnOffServer::parse(msg.opcode, &msg.parameters) {
                            if m.on_off == 1 {
                                println!("Turn ON");
                            } else {
                                println!("Turn OFF");
                            }
                        }
                    }
                    None => break,
                }
                println!("Got message?!");
            }
        }
    }

    println!("Shutting down!");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
