use btmesh_models::{
    generic::battery::{
        GenericBatteryFlags, GenericBatteryFlagsCharging, GenericBatteryFlagsIndicator,
        GenericBatteryFlagsPresence, GenericBatteryMessage, Status as GenericBatteryStatus,
    },
    sensor::{SensorMessage, SensorSetupMessage, SensorStatus},
    Message,
};
use gloo_timers::callback::Interval;
use gloo_utils::document;
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
    let state = use_state(|| SimulatorState::Stopped);
    let onclick = {
        let state = state.clone();
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

                        let interval = interval.parse::<u32>().unwrap();
                        let u = url.clone();
                        let user = username.clone();
                        let pass = password.clone();
                        let _battery = Interval::new(interval * 1000, move || {
                            let u = u.clone();
                            let user = user.clone();
                            let pass = pass.clone();
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
                                match publish(&battery, &u, &user, &pass).await {
                                    Ok(_) => log::info!("Published battery data"),
                                    Err(e) => log::warn!("Error publishing battery data: {:?}", e),
                                }
                            });
                        });

                        let u = url.clone();
                        let user = username.clone();
                        let pass = password.clone();
                        let _sensor = Interval::new(interval * 1000, move || {
                            let u = u.clone();
                            let user = user.clone();
                            let pass = pass.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let sensor: SensorSetupMessage<MicrobitSensorConfig, 1, 1> =
                                    SensorSetupMessage::Sensor(SensorMessage::Status(
                                        SensorStatus::new(SensorPayload { temperature: 22 }),
                                    ));

                                match publish(&sensor, &u, &user, &pass).await {
                                    Ok(_) => log::info!("Published sensor data"),
                                    Err(e) => log::warn!("Error publishing sensor data: {:?}", e),
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
        </>
    }
}

async fn publish<M: Message>(
    msg: &M,
    url: &reqwest::Url,
    username: &str,
    password: &str,
) -> Result<(), std::fmt::Error> {
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
    client
        .post(url.clone())
        .basic_auth(username, Some(password))
        .body(data)
        .send()
        .await
        .map_err(|_| std::fmt::Error)?;
    Ok(())
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
