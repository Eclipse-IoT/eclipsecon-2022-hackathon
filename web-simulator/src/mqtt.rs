use crate::{borrowed::json2command, Publisher};
use anyhow::{anyhow, Context};
use gloo_utils::format::JsValueSerdeExt;
use sensor_model::RawMessage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use wasm_bindgen::{prelude::*, JsCast};
use yew::{html, Callback, Html};

#[wasm_bindgen(module = "/js/paho/wrapper.js")]
extern "C" {

    type Client;

    #[wasm_bindgen(constructor)]
    fn new(endpoint: &str, client_id: &str) -> Client;

    #[wasm_bindgen(method)]
    fn connect(this: &Client, options: &JsValue);

    #[wasm_bindgen(method, getter)]
    fn connected(this: &Client) -> bool;

    #[wasm_bindgen(method)]
    fn disconnect(this: &Client);

    #[wasm_bindgen(method)]
    fn subscribe(this: &Client, filter: &str, options: &JsValue);

    #[wasm_bindgen(method, catch)]
    fn publish(
        this: &Client,
        topic: &str,
        payload: Vec<u8>,
        qos: i32,
        retained: bool,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, setter, js_name = "onMessageArrived")]
    fn set_on_message_arrived(this: &Client, handler: &JsValue);

    #[wasm_bindgen(method, setter, js_name = "onConnectionLost")]
    fn set_on_connection_lost(this: &Client, handler: &JsValue);

    type Message;

    #[wasm_bindgen(method, getter)]
    fn topic(this: &Message) -> String;

    #[wasm_bindgen(method, getter, js_name = "payloadBytes")]
    fn payload_bytes(this: &Message) -> Vec<u8>;
}

#[derive(Clone, Copy, Debug)]
pub enum QoS {
    QoS0,
    QoS1,
    QoS2,
}

impl From<QoS> for i32 {
    fn from(qos: QoS) -> Self {
        match qos {
            QoS::QoS0 => 0,
            QoS::QoS1 => 1,
            QoS::QoS2 => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MqttMessage {
    pub topic: String,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MqttConnectOptions {
    pub username: Option<String>,
    pub password: Option<String>,
    pub clean_session: bool,
    pub reconnect: bool,
    pub keep_alive_interval: Option<Duration>,
    pub timeout: Option<Duration>,
}

impl Default for MqttConnectOptions {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
            clean_session: true,
            reconnect: true,
            keep_alive_interval: None,
            timeout: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConnectOptions {
    #[serde(rename = "userName")]
    username: Option<String>,
    password: Option<String>,
    clean_session: bool,
    reconnect: bool,
    keep_alive_interval: Option<f64>,
    timeout: Option<f64>,
    #[serde(rename = "useSSL")]
    use_ssl: bool,
    #[serde(rename = "mqttVersion")]
    mqtt_version: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubscribeOptions {
    qos: i32,
    timeout: Option<f64>,
}

pub struct MqttClient {
    inner: Inner,
}

impl MqttClient {
    pub fn new(endpoint: &str, client_id: Option<String>) -> Self {
        let client_id = client_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        Self {
            inner: Inner {
                use_ssl: endpoint.starts_with("wss://"),
                client: Client::new(endpoint, &client_id),
                _on_message_arrived: None,
                _on_connection_lost: None,

                _on_connect_success: None,
                _on_connect_failure: None,

                _on_subscribe_success: None,
                _on_subscribe_failure: None,
            },
        }
    }

    pub fn connect(
        &mut self,
        options: MqttConnectOptions,
        on_success: Callback<()>,
        on_failure: Callback<String>,
    ) -> anyhow::Result<()> {
        self.inner.connect(options, on_success, on_failure)
    }

    #[allow(unused)]
    pub fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    pub fn subscribe<S>(
        &mut self,
        filter: S,
        qos: QoS,
        timeout: Duration,
        on_success: Callback<()>,
        on_failure: Callback<String>,
    ) -> anyhow::Result<()>
    where
        S: AsRef<str>,
    {
        self.inner
            .subscribe(filter, qos, timeout, on_success, on_failure)
    }

    pub fn publish<T, P>(&self, topic: T, payload: P, qos: QoS, retain: bool) -> anyhow::Result<()>
    where
        T: AsRef<str>,
        P: Into<Vec<u8>>,
    {
        self.inner.publish(topic, payload, qos, retain)
    }

    pub fn set_on_connection_lost(&mut self, callback: Callback<String>) {
        self.inner.set_on_connection_lost(callback)
    }

    pub fn set_on_message_arrived(&mut self, callback: Callback<MqttMessage>) {
        self.inner.set_on_message_arrived(callback)
    }
}

struct Inner {
    use_ssl: bool,
    client: Client,

    _on_connection_lost: Option<Closure<dyn Fn(JsValue)>>,
    _on_message_arrived: Option<Closure<dyn Fn(JsValue)>>,

    _on_connect_success: Option<Closure<dyn Fn()>>,
    _on_connect_failure: Option<Closure<dyn Fn(JsValue)>>,

    _on_subscribe_success: Option<Closure<dyn Fn()>>,
    _on_subscribe_failure: Option<Closure<dyn Fn(JsValue)>>,
}

impl Inner {
    fn connect(
        &mut self,
        options: MqttConnectOptions,
        on_success: Callback<()>,
        on_failure: Callback<String>,
    ) -> anyhow::Result<()> {
        let MqttConnectOptions {
            username,
            password,
            clean_session,
            reconnect,
            keep_alive_interval,
            timeout,
        } = options;

        let options = JsValue::from_serde(&ConnectOptions {
            username,
            password,
            clean_session,
            reconnect,
            keep_alive_interval: keep_alive_interval.map(|v| v.as_secs_f64()),
            timeout: timeout.map(|v| v.as_secs_f64()),
            use_ssl: self.use_ssl,
            mqtt_version: 4,
        })
        .unwrap();

        let on_failure = Closure::wrap(
            Box::new(move |err| on_failure.emit(convert_error(err))) as Box<dyn Fn(JsValue)>
        );
        let on_success = Closure::wrap(Box::new(move || on_success.emit(())) as Box<dyn Fn()>);

        js_sys::Reflect::set(
            &options,
            &JsValue::from_str("onFailure"),
            on_failure.as_ref(),
        )
        .map_err(str_err)
        .context("Failed to set 'onFailure' handler")?;
        js_sys::Reflect::set(
            &options,
            &JsValue::from_str("onSuccess"),
            on_success.as_ref(),
        )
        .map_err(str_err)
        .context("failed to set 'onSuccess' handler")?;

        // keep reference

        self._on_connect_success = Some(on_success);
        self._on_connect_failure = Some(on_failure);

        // perform connect

        self.client.connect(&options);

        // done

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.client.connected()
    }

    pub fn subscribe<S>(
        &mut self,
        filter: S,
        qos: QoS,
        timeout: Duration,
        on_success: Callback<()>,
        on_failure: Callback<String>,
    ) -> anyhow::Result<()>
    where
        S: AsRef<str>,
    {
        let options = JsValue::from_serde(&SubscribeOptions {
            qos: qos.into(),
            timeout: Some(timeout.as_secs_f64()),
        })
        .context("Failed to convert options")?;

        let on_failure = Closure::wrap(
            Box::new(move |err| on_failure.emit(convert_error(err))) as Box<dyn Fn(JsValue)>
        );
        let on_success = Closure::wrap(Box::new(move || on_success.emit(())) as Box<dyn Fn()>);

        js_sys::Reflect::set(
            &options,
            &JsValue::from_str("onFailure"),
            on_failure.as_ref(),
        )
        .map_err(str_err)
        .context("Failed to set 'onFailure' handler")?;
        js_sys::Reflect::set(
            &options,
            &JsValue::from_str("onSuccess"),
            on_success.as_ref(),
        )
        .map_err(str_err)
        .context("failed to set 'onSuccess' handler")?;

        // keep reference

        self._on_subscribe_success = Some(on_success);
        self._on_subscribe_failure = Some(on_failure);

        // subscribe

        self.client.subscribe(filter.as_ref(), &options);

        // done

        Ok(())
    }

    fn publish<T, P>(&self, topic: T, payload: P, qos: QoS, retain: bool) -> anyhow::Result<()>
    where
        T: AsRef<str>,
        P: Into<Vec<u8>>,
    {
        self.client
            .publish(topic.as_ref(), payload.into(), qos.into(), retain)
            .map_err(str_err)
    }

    pub fn set_on_message_arrived(&mut self, callback: Callback<MqttMessage>) {
        let on_message_arrived = Closure::wrap(Box::new(move |msg| match convert_message(msg) {
            Ok(msg) => callback.emit(msg),
            Err(err) => {
                log::warn!("Failed to parse incoming message: {}", err);
            }
        }) as Box<dyn Fn(JsValue)>);
        self.client
            .set_on_message_arrived(on_message_arrived.as_ref());
        self._on_message_arrived = Some(on_message_arrived);
    }

    pub fn set_on_connection_lost(&mut self, callback: Callback<String>) {
        let on_connection_lost =
            Closure::wrap(
                Box::new(move |err| callback.emit(convert_error(err))) as Box<dyn Fn(JsValue)>
            );
        self.client
            .set_on_connection_lost(on_connection_lost.as_ref());
        self._on_connection_lost = Some(on_connection_lost);
    }
}

fn convert_message(value: JsValue) -> Result<MqttMessage, String> {
    if let Some(msg) = value.dyn_ref::<Message>() {
        Ok(MqttMessage {
            topic: msg.topic(),
            payload: msg.payload_bytes(),
        })
    } else {
        Err("Failed to convert message".into())
    }
}

fn convert_error(value: JsValue) -> String {
    value
        .as_string()
        .or_else(|| {
            value
                .into_serde::<Value>()
                .ok()
                .map(|json| json.to_string())
        })
        .unwrap_or_else(|| "<unknown>".to_string())
}

fn str_err(err: JsValue) -> anyhow::Error {
    match err.as_string() {
        Some(err) => anyhow::Error::msg(err),
        None => match err.into_serde::<Value>() {
            Ok(err) => {
                anyhow!("Unknown error: {err}")
            }
            Err(_) => {
                anyhow!("Unknown error")
            }
        },
    }
}

trait CallbackExt<IN> {
    fn filter_reform<T, F>(&self, f: F) -> Callback<T>
    where
        F: Fn(T) -> Option<IN> + 'static;
}

impl<IN: 'static> CallbackExt<IN> for Callback<IN> {
    fn filter_reform<T, F>(&self, func: F) -> Callback<T>
    where
        F: Fn(T) -> Option<IN> + 'static,
    {
        let this = self.clone();
        let func = move |input| {
            if let Some(output) = func(input) {
                this.emit(output);
            }
        };
        Callback::from(func)
    }
}

pub struct MqttPublisher {
    client: Arc<Mutex<MqttClient>>,
    topic: String,
    qos: QoS,
    opts: MqttOptions,
}

#[derive(Clone, Default)]
pub struct MqttOptions {
    pub on_connection_state: Callback<Html>,
    pub on_command: Callback<RawMessage>,
}

impl MqttPublisher {
    pub fn new(url: reqwest::Url, username: String, password: String, opts: MqttOptions) -> Self {
        opts.on_connection_state.emit(html!("Connecting"));
        let mut client = MqttClient::new(url.as_str(), None);

        let cs = opts.on_connection_state.clone();
        client.set_on_connection_lost(Callback::from(move |err| {
            cs.emit(html!({ format!("Disconnected: {err}") }))
        }));

        client.set_on_message_arrived(opts.on_command.filter_reform(|msg: MqttMessage| {
            log::info!("Received message: {msg:?}");
            if let Some(_) = msg.topic.strip_prefix("command/inbox//") {
                match serde_json::from_slice(&msg.payload) {
                    Ok(command) => Some(command),
                    Err(err) => {
                        log::warn!("Failed to parse command: {err}");
                        if let Ok(json) = serde_json::from_slice(&msg.payload) {
                            log::info!("JSON: {json}");
                            json2command(&json)
                        } else {
                            None
                        }
                    }
                }
            } else {
                None
            }
        }));

        let client = Arc::new(Mutex::new(client));
        let c = client.clone();

        let cs = opts.on_connection_state.clone();

        if let Err(err) = client.lock().unwrap().connect(
            MqttConnectOptions {
                username: Some(username),
                password: Some(password),
                clean_session: true,
                reconnect: true,
                keep_alive_interval: Some(Duration::from_secs(2)),
                timeout: Some(Duration::from_secs(5)),
            },
            {
                let cs = cs.clone();
                cs.emit(html!("Subscribing"));
                Callback::from(move |_| {
                    let _ = c.lock().unwrap().subscribe(
                        "command/inbox/#",
                        QoS::QoS0,
                        Duration::from_secs(5),
                        {
                            let cs = cs.clone();
                            Callback::from(move |_| cs.emit(html!("Running")))
                        },
                        {
                            let cs = cs.clone();
                            Callback::from(move |err| {
                                cs.emit(html!(format!("Failed to subscribe: {err}")));
                            })
                        },
                    );
                })
            },
            {
                let cs = cs.clone();
                Callback::from(move |err| {
                    cs.emit(html!(format!("Failed to connect: {err}")));
                })
            },
        ) {
            log::warn!("Failed to connect: {err}");
        }

        Self {
            client,
            qos: QoS::QoS1,
            topic: "sensor".to_string(),
            opts,
        }
    }
}

impl Publisher for MqttPublisher {
    fn send(&self, payload: String) -> anyhow::Result<()> {
        Ok(self
            .client
            .lock()
            .unwrap()
            .publish(&self.topic, payload, self.qos, false)?)
    }
}

impl Drop for MqttPublisher {
    fn drop(&mut self) {
        self.opts.on_connection_state.emit(html!("Stopped"));
    }
}
