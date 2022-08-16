#![feature(generic_associated_types)]

use bluer::mesh::{application::Application, element::*};
use btmesh_models::{
    sensor::{SensorClient, SensorMessage},
    Model,
};
use clap::Parser;
use dbus::Path;
use futures::{pin_mut, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::{signal, time::sleep};
use sensor_model::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    token: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let session = bluer::Session::new().await?;

    let mesh = session.mesh().await?;

    let (element_control, element_handle) = element_control();

    let root_path = Path::from("/mesh_client");
    let app_path = Path::from(format!("{}/{}", root_path.clone(), "application"));
    let element_path = Path::from(format!("{}/{}", root_path.clone(), "ele00"));

    let sim = Application {
        path: app_path,
        elements: vec![Element {
            path: element_path,
            models: vec![Arc::new(FromDrogue::new(SensorClient::<MicrobitSensorConfig, 1, 1>::new()))],
            control_handle: Some(element_handle),
        }],
        ..Default::default()
    };

    let registered = mesh.application(root_path.clone(), sim).await?;

    let _node = mesh.attach(root_path.clone(), &args.token).await?;

    println!("Gateway ready. Press Ctrl+C to quit.");
    pin_mut!(element_control);

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            evt = element_control.next() => {
                match evt {
                    Some(msg) => {
                        match SensorClient::<MicrobitSensorConfig, 1, 1>::parse(msg.opcode, &msg.parameters).map_err(|_| std::fmt::Error)? {
                            Some(message) => {
                                match message {
                                    SensorMessage::Status(status) => {
                                        println!("Received {:?}", status.data);
                                    },
                                    _ => todo!(),
                                }
                            },
                            None => todo!()
                        }
                    },
                    None => break,
                }
            },
        }
    }

    println!("Shutting down!");
    drop(registered);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
