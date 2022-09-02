#![no_std]
#![no_main]
#![macro_use]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

mod battery;
mod button;
mod display;
mod sensor;

use battery::*;
use btmesh_device::BluetoothMeshModel;
use btmesh_macro::{device, element};
use btmesh_nrf_softdevice::*;
use button::*;
use core::future::Future;
use display::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use microbit_async::*;
use sensor::*;

extern "C" {
    static __storage: u8;
}

use defmt_rtt as _;
use panic_probe as _;

// Application main entry point. The spawner can be used to start async tasks.
#[embassy_executor::main]
async fn main(s: Spawner) {
    // A board type to access peripherals on the microbit.
    let board = Microbit::new(config());

    // Don't remove. Give flash some time before accessing
    Timer::after(Duration::from_millis(100)).await;

    // An instance of the Bluetooth Mesh stack
    let driver = Driver::new(
        "drogue",
        unsafe { &__storage as *const u8 as u32 },
        100,
        None,
    );

    // An instance of the sensor module implementing the SensorServer model.
    let sensor = Sensor::new(driver.softdevice());

    // An instance of the battery module implementing the GenericBattery model.
    let battery = Battery::new();

    // An instance of the display module implementing the OnOff model.
    let display = DisplayOnOff::new(board.display);

    // An instance of our device with the models we'd like to expose.
    let device = Device::new(board.btn_a, board.btn_b, display, battery, sensor);

    // Spawn the driver worker task as a separate task.
    s.spawn(driver_task(driver, device)).unwrap();
}

#[embassy_executor::task]
async fn driver_task(mut driver: Driver, mut device: Device) {
    loop {
        let _ = driver.run(&mut device).await;
    }
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
    display: DisplayOnOff,
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
        display: DisplayOnOff,
        battery: Battery,
        sensor: Sensor,
    ) -> Self {
        Self {
            front: Front {
                display,
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
