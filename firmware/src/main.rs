#![no_std]
#![no_main]
#![macro_use]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

mod battery;
mod button;
mod onoff;
mod sensor;
mod speaker;

use battery::*;
use btmesh_common::Uuid;
use btmesh_device::BluetoothMeshModel;
use btmesh_macro::{device, element};
use btmesh_nrf_softdevice::*;
use button::*;
use core::future::Future;
use embassy_executor::Spawner;
use embassy_nrf::interrupt;
use embassy_time::{Duration, Timer};
use microbit_async::*;
use onoff::*;
use sensor::*;
use speaker::*;

extern "C" {
    static __storage: u8;
    static __config: u8;
}

use defmt_rtt as _;
use panic_probe as _;

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
        100,
        unprovisioned_uuid(),
    );

    // An instance of the sensor module implementing the SensorServer model.
    let accelerometer = accelerometer::Accelerometer::new(
        board.twispi0,
        interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0),
        board.p23,
        board.p22,
    )
    .unwrap();
    let sensor = Sensor::new(driver.softdevice(), accelerometer);

    // An instance of the battery module implementing the GenericBattery model.
    let battery = Battery::new();

    // An instance of the onoff module implementing the OnOff model.
    let onoff = OnOff::new(board.display, Speaker::new(board.pwm0, board.speaker));

    // An instance of our device with the models we'd like to expose.
    let mut device = Device::new(board.btn_a, board.btn_b, onoff, battery, sensor);

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
    onoff: OnOff,
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
        onoff: OnOff,
        battery: Battery,
        sensor: Sensor,
    ) -> Self {
        Self {
            front: Front {
                onoff,
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

// Loading any compiled-in UUID to use for provisioning
fn unprovisioned_uuid() -> Option<Uuid> {
    const DEVICE_UUID: Option<&str> = option_env!("DEVICE_UUID");

    // Attempt to decode the UUID
    DEVICE_UUID
        .map(|uuid| {
            let mut data = [0; 16];
            if let Ok(_) = hex::decode_to_slice(uuid, &mut data) {
                Some(data)
            } else {
                None
            }
        })
        .flatten()
        .map(|data| Uuid::new(data))
}
