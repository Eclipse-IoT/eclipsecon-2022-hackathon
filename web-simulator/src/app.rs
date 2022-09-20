use crate::{
    http::HttpPublisher,
    mqtt::{MqttOptions, MqttPublisher},
    publisher::{Publisher, PublisherExt},
    utils::InitParams,
};
use btmesh_common::opcode::Opcode;
use btmesh_models::{
    generic::{
        battery::{
            GenericBatteryFlags, GenericBatteryFlagsCharging, GenericBatteryFlagsIndicator,
            GenericBatteryFlagsPresence, GenericBatteryMessage, GenericBatteryStatus,
        },
        onoff::{GenericOnOffMessage, GenericOnOffServer},
    },
    sensor::SensorStatus,
    Model,
};
use gloo_timers::callback::Interval;
use gloo_utils::{document, history, window};
use rand::prelude::random;
use reqwest::Url;
use sensor_model::{RawMessage, SensorMessage, SensorPayload};
use std::{str::FromStr, string::ToString, sync::Arc};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlInputElement as InputElement, Node};
use yew::prelude::*;

pub struct Simulator {
    _battery: Interval,
    _sensor: Interval,
    publisher: Arc<dyn Publisher>,
}

pub enum SimulatorState {
    Running(Simulator),
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixState {
    on: bool,
    brightness: u8,
}

impl core::fmt::Display for SimulatorState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Self::Running(_) => write!(f, "Running"),
            Self::Stopped => write!(f, "Stopped"),
        }
    }
}

#[derive(Clone, Default)]
struct Refs {
    pub url: NodeRef,
    pub application: NodeRef,
    pub device: NodeRef,
    pub password: NodeRef,
    pub interval: NodeRef,
    pub temperature: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FieldProps {
    pub label: String,
    pub children: Children,
}

#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
    html!(
        <div class="field">
            <label class="label">{ &props.label }</label>
            <div class="control">
                { for props.children.iter() }
            </div>
        </div>
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct LedMatrixProps {
    pub state: MatrixState,
}

#[function_component(LedMatrix)]
pub fn led_matrix(props: &LedMatrixProps) -> Html {
    let dotcolor = if props.state.on { "doton" } else { "dotoff" };
    let opacity = if props.state.on {
        props.state.brightness as f32 / 255.0
    } else {
        1.0
    };
    let style = format!("opacity:{}", opacity);

    html!(
        <>
            <span id="0x0" class={dotcolor} style={style.clone()}/>
            <span id="0x1" class={dotcolor} style={style.clone()}/>
            <span id="0x2" class={dotcolor} style={style.clone()}/>
            <span id="0x3" class={dotcolor} style={style.clone()}/>
            <span id="0x4" class={dotcolor} style={style.clone()}/>
        <br />
            <span id="1x0" class={dotcolor} style={style.clone()} />
            <span id="1x1" class={dotcolor} style={style.clone()} />
            <span id="1x2" class={dotcolor} style={style.clone()} />
            <span id="1x3" class={dotcolor} style={style.clone()} />
            <span id="1x4" class={dotcolor} style={style.clone()} />
        <br />
            <span id="2x0" class={dotcolor} style={style.clone()} />
            <span id="2x1" class={dotcolor} style={style.clone()} />
            <span id="2x2" class={dotcolor} style={style.clone()} />
            <span id="2x3" class={dotcolor} style={style.clone()} />
            <span id="2x4" class={dotcolor} style={style.clone()} />
        <br />
            <span id="3x0" class={dotcolor} style={style.clone()} />
            <span id="3x1" class={dotcolor} style={style.clone()} />
            <span id="3x2" class={dotcolor} style={style.clone()} />
            <span id="3x3" class={dotcolor} style={style.clone()} />
            <span id="3x4" class={dotcolor} style={style.clone()} />
        <br />
            <span id="4x0" class={dotcolor} style={style.clone()} />
            <span id="4x1" class={dotcolor} style={style.clone()} />
            <span id="4x2" class={dotcolor} style={style.clone()} />
            <span id="4x3" class={dotcolor} style={style.clone()} />
            <span id="4x4" class={dotcolor} style={style.clone()} />
        <br />
        </>
    )
}

pub struct App {
    state: SimulatorState,
    connection_state: Html,
    matrix: MatrixState,

    url: String,
    application: String,
    device: String,
    password: String,
    temperature: i8,

    refs: Refs,
}

pub enum Msg {
    Set(Box<dyn FnOnce(&mut App)>),
    Start,
    Stop,
    PublishSensor,
    PublishBattery,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let InitParams {
            application,
            device,
            url,
            password,
        } = InitParams::from_location();

        // clean up document url\
        let _ = history().push_state_with_url(
            &JsValue::null(),
            &document().title(),
            Some(&window().location().pathname().ok().unwrap_or_default()),
        );

        Self {
            state: SimulatorState::Stopped,
            matrix: MatrixState {
                on: false,
                brightness: 255,
            },
            temperature: 22,

            url: url.unwrap_or_else(|| {
                "wss://mqtt-endpoint-ws-browser-drogue-iot.apps.wonderful.iot-playground.org/mqtt"
                    .to_string()
            }),
            application: application.unwrap_or_else(|| "eclipsecon-hackathon".to_string()),
            device: device.unwrap_or_default(),
            password: password.unwrap_or_default(),
            connection_state: html!("Stopped"),

            refs: Refs::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Set(f) => {
                f(self);
            }
            Msg::Start => {
                self.start(ctx);
            }
            Msg::Stop => {
                self.set_disabled(false);
                self.state = SimulatorState::Stopped;
            }
            Msg::PublishSensor => {
                if let SimulatorState::Running(simulator) = &self.state {
                    let sensor: SensorMessage =
                        SensorMessage::Status(SensorStatus::new(SensorPayload {
                            temperature: self.temperature,
                            acceleration: Default::default(),
                            noise: 0,
                        }));

                    let _ = simulator.publisher.publish(&sensor);
                }
            }
            Msg::PublishBattery => {
                if let SimulatorState::Running(simulator) = &self.state {
                    let battery = GenericBatteryMessage::Status(GenericBatteryStatus::new(
                        0,
                        0,
                        0,
                        GenericBatteryFlags {
                            presence: GenericBatteryFlagsPresence::NotPresent,
                            indicator: GenericBatteryFlagsIndicator::Unknown,
                            charging: GenericBatteryFlagsCharging::NotChargeable,
                        },
                    ));

                    let _ = simulator.publisher.publish(&battery);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let temperature_ref = self.refs.temperature.clone();
        let on_change_temp = ctx.link().batch_callback(move |_| {
            if let Some(temp) = temperature_ref
                .cast::<InputElement>()
                .and_then(|input| input.value().parse::<i8>().ok())
            {
                vec![Msg::Set(Box::new(move |app| app.temperature = temp))]
            } else {
                vec![]
            }
        });

        let mut violations = 0;

        html!(
        <>
            <section class="hero is-primary">
              <div class="hero-body">
                <h1 class="title">
                  { "Device Simulator" }
                </h1>
                <p class="subtitle">
                  { "This application simulates the payloads generated by the EclipseCon 2022 Hackathon mesh network. "}
                </p>
              </div>
            </section>

            <section class="section"><div class="container">

            <div class="columns">
            <div class="column">

            <Field label="URL">
                {{
                    let mut classes = Classes::from("input");
                    if self.url.is_empty() {
                        classes.push("is-danger");
                        violations += 1;
                    }
                    html!(
                        <input id="url" type="text" class={classes} value={self.url.clone()} ref={self.refs.url.clone()}
                            oninput={Self::value_setter(&ctx, &self.refs.url, |app, value| app.url=value).reform(|_|())}
                        />
                    )
                }}
            </Field>

            <Field label="Application">
                {{
                    let mut classes = Classes::from("input");
                    if self.application.is_empty() {
                        classes.push("is-danger");
                        violations += 1;
                    }
                    html!(
                        <input id="application" type="text" class={classes} value={self.application.clone()} ref={self.refs.application.clone()}
                            oninput={Self::value_setter(&ctx, &self.refs.application, |app, value| app.application=value).reform(|_|())}
                        />
                    )
                }}
            </Field>

            <Field label="Device">
                {{
                    let mut classes = Classes::from("input");
                    if self.device.is_empty() {
                        classes.push("is-danger");
                        violations += 1;
                    }
                    html!(
                        <input id="device" type="text" class={classes} value={(self.device).clone()} ref={self.refs.device.clone()}
                            oninput={Self::value_setter(&ctx, &self.refs.device, |app, value| app.device=value).reform(|_|())}
                        />
                    )
                }}
            </Field>

            <Field label="Password">
                {{
                    let mut classes = Classes::from("input");
                    if self.password.is_empty() {
                        classes.push("is-danger");
                        violations += 1;
                    }
                    html!(
                        <input id="password" type="password" class={classes} value={self.password.clone()} pattern="[0-9]+" ref={self.refs.password.clone()}
                            oninput={Self::value_setter(&ctx, &self.refs.password, |app, value| app.password=value).reform(|_|())}
                        />
                    )
                }}
            </Field>

            <Field label="Interval (seconds)">
                <input id="interval" type="text" class="input" value="5" ref={self.refs.interval.clone()} />
            </Field>

            <div class="field is-grouped">
                <p class="control">
                    <button
                        class="button is-primary"
                        disabled={matches!(self.state, SimulatorState::Running(_)) || violations > 0}
                        onclick={ctx.link().callback(|_|Msg::Start)}>
                        {"Start"}
                    </button>
                </p>
                <p class="control">
                    <button
                        class="button is-light"
                        disabled={!matches!(self.state, SimulatorState::Running(_))}
                        onclick={ctx.link().callback(|_| Msg::Stop)}>
                        {"Stop"}
                    </button>
                </p>
            </div>

            </div>

            <div class="column">

            <Field label="State">
                <p>{self.connection_state.clone()}</p>
            </Field>

            <Field label="Temperature">
                <input
                    id="temp-slider"
                    class="slider is-half has-output"
                    step="1" min={i8::MIN.to_string()} max={i8::MAX.to_string()}
                    value={self.temperature.to_string()}
                    type="range"
                    onchange={on_change_temp.clone().reform(|_|())}
                    oninput={on_change_temp.reform(|_|())}
                    ref={self.refs.temperature.clone()}
                />
                <output class="slider" for="temp-slider">{self.temperature as f32 / 2.0}</output>
            </Field>

            <Field label="Display">
                <LedMatrix state={self.matrix.clone()} />
            </Field>

            </div></div>

            </div></section>
            </>
        )
    }
}

impl App {
    fn setter<F, T, H, S>(ctx: &Context<Self>, r#ref: &NodeRef, f: F, s: S) -> Callback<()>
    where
        T: 'static,
        F: FnOnce(H) -> Option<T> + 'static,
        H: AsRef<Node> + From<JsValue>,
        S: FnOnce(&mut App, T) + 'static,
    {
        let r = r#ref.clone();
        ctx.link().batch_callback_once(move |_| {
            if let Some(value) = r.cast::<H>().and_then(|ele| f(ele)) {
                vec![Msg::Set(Box::new(|app| s(app, value)))]
            } else {
                vec![]
            }
        })
    }

    fn value_setter<T, S>(ctx: &Context<Self>, r#ref: &NodeRef, s: S) -> Callback<()>
    where
        T: FromStr + 'static,
        S: FnOnce(&mut App, T) + 'static,
    {
        Self::setter(
            ctx,
            r#ref,
            |ele: InputElement| T::from_str(&ele.value()).ok(),
            s,
        )
    }

    fn set_disabled(&self, state: bool) {
        let inputs = [
            &self.refs.url,
            &self.refs.application,
            &self.refs.device,
            &self.refs.password,
            &self.refs.interval,
        ];

        set_disabled(state, inputs);
    }

    fn start(&mut self, ctx: &Context<Self>) {
        let interval = document()
            .get_element_by_id("interval")
            .map(|e| e.dyn_ref::<InputElement>().map(|input| input.value()))
            .flatten()
            .filter(|s| !s.is_empty());

        let url = Some(self.url.clone()).filter(|s| !s.is_empty());
        let application = Some(self.application.clone()).filter(|s| !s.is_empty());
        let device = Some(self.device.clone()).filter(|s| !s.is_empty());
        let password = Some(self.password.clone()).filter(|s| !s.is_empty());

        match (url, application, device, password, interval) {
            (Some(url), Some(application), Some(device), Some(password), Some(interval)) => {
                let url = Url::parse(&format!("{}/v1/sensor", url)).unwrap();
                let username = format!("{}@{}", device, application);

                let on_command = ctx.link().batch_callback(|command: RawMessage| {
                    log::info!("Received command: {command:?}");
                    let (opcode, _) = Opcode::split(&command.opcode[..]).unwrap();
                    if let Ok(Some(GenericOnOffMessage::Set(msg))) =
                        GenericOnOffServer::parse(&opcode, &command.parameters)
                    {
                        vec![Msg::Set(Box::new(move |app| {
                            app.matrix = MatrixState {
                                on: msg.on_off == 1,
                                brightness: app.matrix.brightness,
                            }
                        }))]
                    } else {
                        vec![]
                    }
                });

                let on_connection_state = ctx
                    .link()
                    .callback(|state| Msg::Set(Box::new(|app| app.connection_state = state)));

                let publisher: Arc<dyn Publisher> = match url.scheme() {
                    "ws" | "wss" => Arc::new(MqttPublisher::new(
                        url,
                        username,
                        password,
                        MqttOptions {
                            on_command,
                            on_connection_state,
                        },
                    )),
                    _ => Arc::new(HttpPublisher::new(
                        url,
                        username,
                        password,
                        on_command,
                        on_connection_state,
                    )),
                };

                // Battery
                let interval = interval.parse::<u32>().unwrap();
                let start_rand: u32 = random::<u32>() % 2000;
                let send_interval = start_rand + (interval * 1000);
                log::info!("Publishing battery data at interval {} ms", send_interval);

                let link = ctx.link().clone();
                let _battery = Interval::new(send_interval, move || {
                    link.send_message(Msg::PublishBattery);
                });

                // Sensor
                let start_rand: u32 = random::<u32>() % 2000;
                let send_interval = start_rand + (interval * 1000);
                log::info!("Publishing sensor data at interval {} ms", send_interval);

                let link = ctx.link().clone();
                let _sensor = Interval::new(send_interval, move || {
                    link.send_message(Msg::PublishSensor);
                });

                let sim = Simulator {
                    _battery,
                    _sensor,
                    publisher,
                };
                self.set_disabled(true);
                self.state = SimulatorState::Running(sim);
            }
            _ => {
                gloo_dialogs::alert("One or more fields are missing a value");
            }
        }
    }
}

fn set_disabled<'e, I>(state: bool, elements: I)
where
    I: IntoIterator<Item = &'e NodeRef>,
{
    for e in elements {
        if let Some(ele) = e.cast::<InputElement>() {
            ele.set_disabled(state);
        }
    }
}
