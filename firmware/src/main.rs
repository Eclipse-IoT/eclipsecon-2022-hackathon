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

// Application must run at a lower priority than softdevice
fn config() -> Config {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    config
}

#[embassy_executor::main]
async fn main(s: Spawner) {
    let board = Microbit::new(config());

    let driver = Driver::new(
        "drogue",
        unsafe { &__storage as *const u8 as u32 },
        100,
        None,
    );

    let sd = driver.softdevice();
    let sensor = Sensor::new(sd);
    let battery = Battery::new();
    let display = DisplayOnOff::new(board.display);

    let device = Device::new(board.btn_a, board.btn_b, display, battery, sensor);

    // Give flash some time before accessing
    Timer::after(Duration::from_millis(100)).await;

    s.spawn(driver_task(driver, device)).unwrap();
}

#[embassy_executor::task]
async fn driver_task(mut driver: Driver, mut device: Device) {
    loop {
        let _ = driver.run(&mut device).await;
    }
}

#[device(cid = 0x0003, pid = 0x0001, vid = 0x0001)]
pub struct Device {
    front: Front,
    btn_a: ButtonA,
    btn_b: ButtonB,
}

#[element(location = "front")]
struct Front {
    display: DisplayOnOff,
    battery: Battery,
    sensor: Sensor,
}

#[element(location = "left")]
struct ButtonA {
    button: ButtonOnOff,
}

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
