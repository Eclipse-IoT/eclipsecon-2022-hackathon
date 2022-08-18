#![feature(generic_associated_types)]

use bluer::mesh::{application::Application, element::*};
use btmesh_models::{
    generic::{
        battery::GenericBatteryClient,
        onoff::{GenericOnOffClient, GenericOnOffServer},
    },
    sensor::{SensorClient},
    Model,
};
use clap::Parser;
use dbus::Path;
use futures::{pin_mut, StreamExt};
use paho_mqtt as mqtt;
use sensor_model::*;
use std::{sync::Arc, time::Duration};
use tokio::{signal, time::sleep};

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

    let root_path = Path::from("/gateway");
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
                    Arc::new(FromDrogue::new(GenericOnOffClient)),
                    Arc::new(FromDrogue::new(GenericBatteryClient)),
                    Arc::new(FromDrogue::new(Sensor::new())),
                ],
                control_handle: Some(element_handle.clone()),
            },
            Element {
                path: left,
                location: Some(0x010D),
                models: vec![Arc::new(FromDrogue::new(GenericOnOffServer))],
                control_handle: Some(element_handle.clone()),
            },
            Element {
                path: right,
                location: Some(0x010E),
                models: vec![Arc::new(FromDrogue::new(GenericOnOffServer))],
                control_handle: Some(element_handle),
            },
        ],
        ..Default::default()
    };

    let registered = mesh.application(root_path.clone(), sim).await?;

    let _node = mesh.attach(root_path.clone(), &args.token).await?;

    let mqtt_uri = args.drogue_mqtt_uri;

    let mqtt_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(mqtt_uri.clone())
        .client_id("btmesh-gateway")
        .persistence(mqtt::PersistenceType::None)
        .finalize();
    let mqtt_client = mqtt::AsyncClient::new(mqtt_opts)?;

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

    println!("Gateway ready. Press Ctrl+C to quit.");
    pin_mut!(element_control);

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => break,
            evt = element_control.next() => {
                match evt {
                    Some(msg) => {
                        println!("Received message with opcode {:?} and {} parameter bytes!", msg.opcode, msg.parameters.len());
                        match SensorClient::<MicrobitSensorConfig, 1, 1>::parse(msg.opcode, &msg.parameters).map_err(|_| std::fmt::Error)? {
                            Some(message) => {
                                println!("Received {:?}", message);
                            },
                            None => {}
                        }

                        match GenericBatteryClient ::parse(msg.opcode, &msg.parameters).map_err(|_| std::fmt::Error)? {
                            Some(message) => {
                                println!("Received {:?}", message);
                            },
                            None => {}
                        }
                        let mut opcode: heapless::Vec<u8, 16> = heapless::Vec::new();
                        msg.opcode.emit(&mut opcode).map_err(|_| std::fmt::Error)?;

                        let mut parameters = Vec::new();
                        parameters.extend_from_slice(&msg.parameters);
                        let message = RawMessage {
                            opcode: opcode.to_vec(),
                            parameters,
                        };
                        let data = serde_json::to_string(&message)?;

                        let src = msg.src.as_bytes();
                        let topic = format!("sensor/{:02x}{:02x}", src[0], src[1]);

                        let message = mqtt::Message::new(topic, data.as_bytes(), 1);
                        if let Err(e) = mqtt_client.publish(message).await {
                            log::warn!(
                                "Error publishing command back to device: {:?}",
                                e
                            );
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
