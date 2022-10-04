#![no_std]
#![no_main]
#![macro_use]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

mod adc;
mod battery;
mod button;
mod display;
mod mic;
mod sensor;
mod speaker;

use adc::*;
use battery::*;
use btmesh_device::BluetoothMeshModel;
use btmesh_macro::{device, element};
use btmesh_nrf_softdevice::{BluetoothMeshDriverConfig, Driver};
use button::*;
use core::future::Future;
use display::*;
use embassy_executor::Spawner;
use embassy_nrf::interrupt;
use embassy_time::{Duration, Timer};
use mic::*;
use microbit_bsp::*;
use sensor::*;
use speaker::*;
use static_cell::StaticCell;

extern "C" {
    static __storage: u8;
    static __storage_extra: u8;
}

use defmt_rtt as _;

#[cfg(feature = "panic-probe")]
use panic_probe as _;

#[cfg(feature = "panic-reset")]
use panic_reset as _;

// Application main entry point. The spawner can be used to start async tasks.
#[embassy_executor::main]
async fn main(_s: Spawner) {
    // A board type to access peripherals on the microbit.
    let board = Microbit::new(config());

    // Don't remove. Give flash some time before accessing
    Timer::after(Duration::from_millis(100)).await;

    // An instance of the Bluetooth Mesh stack
    let mut driver = Driver::new(
        "drogue",
        unsafe { &__storage as *const u8 as u32 },
        Some(unsafe { &__storage_extra as *const u8 as u32 }),
        100,
        BluetoothMeshDriverConfig {
            uuid: None,
            persist_interval: Some(Duration::from_secs(10)),
        },
    );

    // A safe, shared reference to the Analog To Digital converter
    static ADC: StaticCell<SharedAdc> = StaticCell::new();
    let adc = ADC.init(SharedAdc::new(Adc::new(
        board.saadc,
        interrupt::take!(SAADC),
    )));

    // An accelerometer for recording orientation
    let accelerometer = accelerometer::Accelerometer::new(
        board.twispi0,
        interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0),
        board.p23,
        board.p22,
    )
    .unwrap();

    // A microphone for reading sound levels.
    let mic = Microphone::new(adc, board.microphone, board.micen);

    // An instance of the sensor module implementing the SensorServer model.
    let sensor = Sensor::new(driver.softdevice(), accelerometer, mic);

    // An instance of the battery module implementing the GenericBattery model.
    let battery = Battery::new(adc);

    // An instance of the display implementing the Level model.
    let display = Display::new(board.display);

    // An instance of the speaker implementing the OnOff model.
    let speaker = Speaker::new(board.pwm0, board.speaker);

    // An instance of our device with the models we'd like to expose.
    let mut device = Device::new(board.btn_a, board.btn_b, display, speaker, battery, sensor);

    // Run the mesh stack
    let _ = driver.run(&mut device).await;
}

// A BluetoothMesh device with each field being a Bluetooth Mesh element.
#[device(cid = 0x0003, pid = 0x0001, vid = 0x0001)]
pub struct Device {
    front: Front,
    btn_a: ButtonA,
    btn_b: ButtonB,
}

// An element with multiple models.
#[element(location = "front")]
struct Front {
    speaker: Speaker,
    display: Display,
    battery: Battery,
    sensor: Sensor,
}

// An element for the 'A' button
#[element(location = "left")]
struct ButtonA {
    button: ButtonOnOff,
}

// An element for the 'B' button
#[element(location = "right")]
struct ButtonB {
    button: ButtonOnOff,
}

impl Device {
    pub fn new(
        btn_a: Button,
        btn_b: Button,
        display: Display,
        speaker: Speaker,
        battery: Battery,
        sensor: Sensor,
    ) -> Self {
        Self {
            front: Front {
                display,
                speaker,
                battery,
                sensor,
            },
            btn_a: ButtonA {
                button: ButtonOnOff::new(btn_a),
            },
            btn_b: ButtonB {
                button: ButtonOnOff::new(btn_b),
            },
        }
    }
}

// Application must run at a lower priority than softdevice. DO NOT CHANGE
fn config() -> Config {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    config
}
