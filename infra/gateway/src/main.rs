#![feature(generic_associated_types)]

use clap::Parser;
use clap_num::maybe_hex;
use eclipsecon_gateway::{gateway, provisioner};
use paho_mqtt as mqtt;
use rand::{rngs::OsRng, seq::SliceRandom};
use std::time::Duration;
use tokio::{signal, sync::broadcast};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    token: String,
    #[clap(long, env, default_value = "ssl://mqtt.sandbox.drogue.cloud:8883")]
    drogue_mqtt_uri: String,
    #[clap(long, env, default_value = "eclipsecon-hackathon")]
    drogue_application: String,
    #[clap(long, env)]
    drogue_device: String,
    #[clap(long, env, default_value = "hey-rodney")]
    drogue_password: String,
    #[clap(long, env, default_value = "/etc/ssl/certs/ca-bundle.crt")]
    ca_path: String,
    #[clap(long, env, parse(try_from_str), default_value = "false")]
    disable_tls: bool,
    #[clap(long, env, parse(try_from_str), default_value = "false")]
    insecure_tls: bool,
    #[clap(long)]
    provisioner_token: Option<String>,
    #[clap(long, parse(try_from_str=maybe_hex))]
    provisioner_start_address: Option<u16>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    console_subscriber::init();
    let args = Args::parse();

    let mqtt_uri = args.drogue_mqtt_uri;

    log::info!("Connecting to: {}", mqtt_uri);

    let mqtt_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(mqtt_uri.clone())
        .mqtt_version(mqtt::MQTT_VERSION_5)
        .client_id("btmesh-gateway")
        .persistence(mqtt::PersistenceType::None)
        .finalize();

    let mut mqtt_client = mqtt::AsyncClient::new(mqtt_opts)?;

    let mut conn_opts = mqtt::ConnectOptionsBuilder::new();
    conn_opts.clean_start(true);
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

    mqtt_client.set_connected_callback(|_| {
        log::info!("Gateway connected");
    });

    mqtt_client.connect(conn_opts).await?;

    let mqtt_commands = mqtt_client.get_stream(100);
    mqtt_client.subscribe("command/inbox/#", 1).await?;
    log::info!("Subscribed to commands");

    let session = bluer::Session::new().await?;
    let mesh = session.mesh().await?;

    let (commands_tx, _) = broadcast::channel(10);

    let mut tasks = Vec::new();
    if let Some(token) = args.provisioner_token {
        let start_address: u16 = args.provisioner_start_address.unwrap_or({
            // TODO: Specific for this deployment
            let lowest: u16 = 0x00ab;
            let highest: u16 = 0x7fff;
            let devices = 150;
            let ranges: Vec<u16> = (lowest..highest).step_by(devices).collect();
            *ranges.choose(&mut OsRng).unwrap_or(&0)
        });
        log::info!(
            "Enabling provisioner with start address 0x{:04x}",
            start_address
        );
        tasks.push(tokio::spawn(provisioner::run(
            mesh.clone(),
            provisioner::Config::new(token, start_address),
            commands_tx.subscribe(),
            mqtt_client.clone(),
        )));
    }

    tasks.push(tokio::spawn(gateway::run(
        mesh,
        gateway::Config::new(args.token),
        commands_tx.subscribe(),
        mqtt_client,
    )));

    log::info!("Gateway ready. Press Ctrl+C to quit.");

    tasks.push(tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    log::info!("Got shutdown signal, terminating...");
                    drop(commands_tx);
                    break
                }
                command = mqtt_commands.recv() => {
                    if let Ok(Some(command)) = command {
                        let topic = command.topic().to_string();
                        let payload : Vec<u8>= command.payload().into();
                        if log::log_enabled!(log::Level::Info) {
                            log::info!("Received command: {topic} / {}", String::from_utf8_lossy(&payload) );
                        }
                        commands_tx.send((topic, payload))
                            .map_err(|err| {
                                log::warn!("Failed to queue command: {err}");
                                err
                            }
                        )?;
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        log::info!("Exited main loop, waiting for tasks to complete");

        Ok::<_, anyhow::Error>(())
    }));

    let r = futures::future::select_all(tasks).await.0;

    log::info!("One of the tasks exited, exiting application. Reason: {r:?}");

    Ok(())
}
