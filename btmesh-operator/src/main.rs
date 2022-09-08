use anyhow::Context;
use clap::Parser;

use drogue_client::openid::AccessTokenProvider;
use paho_mqtt as mqtt;

use std::time::Duration;

mod health;
mod provisioner;

#[derive(Parser, Debug)]
struct Args {
    /// Mqtt server uri (tcp://host:port)
    #[clap(long)]
    mqtt_uri: String,

    /// Mqtt group id for shared subscription (for horizontal scaling)
    #[clap(long)]
    mqtt_group_id: Option<String>,

    /// Device registry URL
    /// Mqtt server uri (tcp://host:port)
    #[clap(long)]
    device_registry: String,

    /// Name of specific application to manage firmware updates for (will use all accessible from service account by default)
    #[clap(long)]
    application: String,

    /// Token for authenticating ajour to Drogue IoT
    #[clap(long)]
    token: String,

    /// User for authenticating ajour to Drogue IoT
    #[clap(long)]
    user: String,

    /// Path to CA
    #[clap(long)]
    ca_path: Option<String>,

    /// Disable TLS
    #[clap(long)]
    disable_tls: bool,

    /// Ignore cert validation
    #[clap(long)]
    insecure_tls: bool,

    /// Disable /health endpoint
    #[clap(long)]
    disable_health: bool,

    /// Interval reconciling devices
    #[clap(short, long, parse(try_from_str=humantime::parse_duration))]
    interval: Option<Duration>,

    /// Port for health endpoint
    #[clap(long, default_value_t = 8080)]
    health_port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::init();

    let mqtt_uri = args.mqtt_uri;
    let token = args.token;

    let mqtt_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(mqtt_uri)
        .client_id("btmesh-provisioning-operator")
        .persistence(mqtt::PersistenceType::None)
        .finalize();
    let mut mqtt_client = mqtt::AsyncClient::new(mqtt_opts)?;

    let tp = AccessTokenProvider {
        user: args.user.to_string(),
        token: token.to_string(),
    };

    let url = reqwest::Url::parse(&args.device_registry)?;
    let drg = provisioner::DrogueClient::new(reqwest::Client::new(), url, tp);

    let mut conn_opts = mqtt::ConnectOptionsBuilder::new();
    conn_opts.user_name(args.user);
    conn_opts.password(token.clone());
    conn_opts.keep_alive_interval(Duration::from_secs(30));
    conn_opts.automatic_reconnect(Duration::from_millis(100), Duration::from_secs(5));

    if !args.disable_tls {
        let ca = args
            .ca_path
            .unwrap_or("/etc/ssl/certs/ca-bundle.crt".to_string());
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
        if let Err(e) = t.wait_for(Duration::from_secs(10)) {
            log::warn!("Error reconnecting to broker ({:?}), exiting", e);
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

    mqtt_client
        .connect(conn_opts)
        .await
        .context("Failed to connect to MQTT endpoint")?;

    let healthz = if !args.disable_health {
        Some(health::HealthServer::new(args.health_port))
    } else {
        None
    };

    log::info!("Starting server");

    let mut app = provisioner::Operator::new(
        mqtt_client,
        args.mqtt_group_id,
        args.application,
        drg,
        args.interval.unwrap_or(Duration::from_secs(60)),
    );

    if let Some(mut h) = healthz {
        futures::try_join!(app.run(), h.run())?;
    } else {
        app.run().await?;
    }
    Ok(())
}
