use btmesh_common::opcode::Opcode;
use btmesh_models::{
    generic::{
        battery::{
            GenericBatteryFlags, GenericBatteryFlagsCharging, GenericBatteryFlagsIndicator,
            GenericBatteryFlagsPresence, GenericBatteryMessage, GenericBatteryStatus,
        },
        onoff::{GenericOnOffMessage, GenericOnOffServer},
    },
    sensor::{SensorMessage, SensorSetupMessage, SensorStatus},
    Message, Model,
};
use gloo_timers::callback::Interval;
use gloo_utils::document;
use rand::prelude::random;
use sensor_model::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

pub struct Simulator {
    _battery: Interval,
    _sensor: Interval,
}

pub enum SimulatorState {
    Running(Simulator),
    Stopped,
}

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

#[function_component(App)]
fn app() -> Html {
    let matrix = use_state(|| MatrixState {
        on: false,
        brightness: 128,
    });
    let state = use_state(|| SimulatorState::Stopped);
    let onclick = {
        let state = state.clone();
        let matrix = matrix.clone();
        Callback::from(move |_| {
            let url = "https://web-simulator-eclipsecon-2022.apps.sandbox.drogue.world";
            //let url = "http://localhost:8088";
            let application = document()
                .get_element_by_id("application")
                .map(|e| e.dyn_ref::<InputElement>().map(|input| input.value()))
                .flatten()
                .filter(|s| !s.is_empty());
            let device = document()
                .get_element_by_id("device")
                .map(|e| e.dyn_ref::<InputElement>().map(|input| input.value()))
                .flatten()
                .filter(|s| !s.is_empty());
            let password = document()
                .get_element_by_id("password")
                .map(|e| e.dyn_ref::<InputElement>().map(|input| input.value()))
                .flatten()
                .filter(|s| !s.is_empty());
            let interval = document()
                .get_element_by_id("interval")
                .map(|e| e.dyn_ref::<InputElement>().map(|input| input.value()))
                .flatten()
                .filter(|s| !s.is_empty());

            match &*state {
                SimulatorState::Running(_) => {
                    state.set(SimulatorState::Stopped);
                }
                SimulatorState::Stopped => match (application, device, password, interval) {
                    (Some(application), Some(device), Some(password), Some(interval)) => {
                        let url = reqwest::Url::parse(&format!("{}/v1/sensor", url,)).unwrap();
                        let username = format!("{}@{}", device, application);

                        // Battery
                        let interval = interval.parse::<u32>().unwrap();
                        let u = url.clone();
                        let user = username.clone();
                        let pass = password.clone();
                        let start_rand: u32 = random::<u32>() % 2000;
                        let send_interval = start_rand + (interval * 1000);
                        let m = matrix.clone();
                        log::info!(
                            "Publishing battery data at interval {} ms",
                            send_interval / 1000
                        );
                        let _battery = Interval::new(send_interval, move || {
                            let u = u.clone();
                            let user = user.clone();
                            let pass = pass.clone();
                            let m = m.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let battery =
                                    GenericBatteryMessage::Status(GenericBatteryStatus::new(
                                        0,
                                        0,
                                        0,
                                        GenericBatteryFlags {
                                            presence: GenericBatteryFlagsPresence::NotPresent,
                                            indicator: GenericBatteryFlagsIndicator::Unknown,
                                            charging: GenericBatteryFlagsCharging::NotChargeable,
                                        },
                                    ));
                                match publish(&battery, &u, &user, &pass, send_interval / 1000)
                                    .await
                                {
                                    Ok(command) => {
                                        if let Some(command) = command {
                                            let (opcode, _) =
                                                Opcode::split(&command.opcode[..]).unwrap();
                                            if let Ok(Some(GenericOnOffMessage::Set(msg))) =
                                                GenericOnOffServer::parse(
                                                    opcode,
                                                    &command.parameters,
                                                )
                                            {
                                                m.set(MatrixState {
                                                    on: msg.on_off == 1,
                                                    brightness: m.brightness,
                                                });
                                            }
                                        }
                                        log::info!("Published battery data");
                                    }
                                    Err(e) => {
                                        log::warn!("Error publishing battery data: {:?}", e)
                                    }
                                }
                            });
                        });

                        // Sensor
                        let m = matrix.clone();
                        let u = url.clone();
                        let user = username.clone();
                        let pass = password.clone();
                        let start_rand: u32 = random::<u32>() % 2000;
                        let send_interval = start_rand + (interval * 1000);
                        log::info!("Publishing sensor data at interval {} ms", send_interval);
                        let _sensor = Interval::new(send_interval, move || {
                            let u = u.clone();
                            let user = user.clone();
                            let pass = pass.clone();
                            let m = m.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let sensor: SensorSetupMessage<MicrobitSensorConfig, 1, 1> =
                                    SensorSetupMessage::Sensor(SensorMessage::Status(
                                        SensorStatus::new(SensorPayload { temperature: 22 }),
                                    ));

                                match publish(&sensor, &u, &user, &pass, send_interval).await {
                                    Ok(command) => {
                                        if let Some(command) = command {
                                            let (opcode, _) =
                                                Opcode::split(&command.opcode[..]).unwrap();
                                            if let Ok(Some(GenericOnOffMessage::Set(msg))) =
                                                GenericOnOffServer::parse(
                                                    opcode,
                                                    &command.parameters,
                                                )
                                            {
                                                m.set(MatrixState {
                                                    on: msg.on_off == 1,
                                                    brightness: m.brightness,
                                                });
                                            }
                                        }
                                        log::info!("Published sensor data");
                                    }
                                    Err(e) => {
                                        log::warn!("Error publishing sensor data: {:?}", e)
                                    }
                                }
                            });
                        });

                        let sim = Simulator { _battery, _sensor };
                        state.set(SimulatorState::Running(sim));
                    }
                    _ => {
                        gloo_dialogs::alert("One or more fields are missing a value");
                    }
                },
            }
        })
    };

    let dotcolor = if matrix.on { "doton" } else { "dotoff" };
    let opacity = if matrix.on {
        matrix.brightness as f32 / 255.0
    } else {
        1.0
    };
    let style = format!("opacity:{}", opacity);

    html! {
        <>
        <h1>{ "Device Simulator" }</h1>
        <p><b>{"STATE: "}</b>{&*state}</p>
        <p>{ "This application simulates the payloads generated by the EclipseCon 2022 Hackathon mesh network. "}</p>
        <p><b>{"Application: "}</b></p>
        <input id="application" type="text" class="config" value="eclipsecon-hackathon" size="25" />
        <p><b>{"Device: "}</b></p>
        <input id="device" type="text" class="config" value="simulator1" size="25" />
        <p><b>{"Password: "}</b></p>
        <input id="password" type="password" class="config" value="hey-rodney" size="25" pattern="[0-9]+" />
        <p><b>{"Interval (seconds): "}</b></p>
        <input id="interval" type="text" class="config" value="5" size="25" />
        <br />
        <br />
        <button id="submit" {onclick}>{match &*state {
            SimulatorState::Running(_) => "Stop",
            SimulatorState::Stopped => "Run",
        }}</button>
        <br />
        <h2>{"Display"}</h2>
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
    }
}

async fn publish<M: Message>(
    msg: &M,
    url: &reqwest::Url,
    username: &str,
    password: &str,
    timeout: u32,
) -> Result<Option<RawMessage>, std::fmt::Error> {
    let mut opcode: heapless::Vec<u8, 16> = heapless::Vec::new();
    msg.opcode()
        .emit(&mut opcode)
        .map_err(|_| std::fmt::Error)?;

    let mut parameters: heapless::Vec<u8, 386> = heapless::Vec::new();
    msg.emit_parameters(&mut parameters)
        .map_err(|_| std::fmt::Error)?;
    let message = RawMessage {
        opcode: opcode.to_vec(),
        parameters: parameters.to_vec(),
    };
    let data = serde_json::to_string(&message).map_err(|_| std::fmt::Error)?;

    let client = reqwest::Client::new();
    let response = client
        .post(url.clone())
        .query(&["ct", &format!("{}", timeout)])
        .basic_auth(username, Some(password))
        .body(data)
        .send()
        .await;
    if let Ok(response) = response {
        if let Ok(response) = response.json::<RawMessage>().await {
            return Ok(Some(response));
        }
    }
    Ok(None)
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
