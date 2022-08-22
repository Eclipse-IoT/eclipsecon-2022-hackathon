#![no_std]
#![no_main]
#![macro_use]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use btmesh_device::{BluetoothMeshModel, BluetoothMeshModelContext};
use btmesh_macro::{device, element};
use btmesh_models::{
    generic::{
        battery::{
            GenericBatteryFlags, GenericBatteryFlagsCharging, GenericBatteryFlagsIndicator,
            GenericBatteryFlagsPresence, GenericBatteryMessage, GenericBatteryServer,
            Status as BatteryStatus,
        },
        onoff::{
            GenericOnOffClient, GenericOnOffMessage, GenericOnOffServer, Set as GenericOnOffSet,
        },
    },
    sensor::{SensorMessage, SensorSetupMessage, SensorSetupServer, SensorStatus},
};
use btmesh_nrf_softdevice::*;
use core::future::Future;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker, Timer};
use embassy_util::{select, Either};
use futures::StreamExt;
use microbit_async::*;
use nrf_softdevice::{temperature_celsius, Softdevice};
use sensor_model::*;

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
async fn main(_s: Spawner) {
    let board = Microbit::new(config());

    let mut driver = Driver::new("drogue", unsafe { &__storage as *const u8 as u32 }, 100);

    let sd = driver.softdevice();
    let sensor = Sensor::new(Duration::from_secs(5), sd);
    let battery = Battery::new(Duration::from_secs(5));

    let mut device = Device::new(board.btn_a, board.btn_b, board.display, battery, sensor);

    // Give flash some time before accessing
    Timer::after(Duration::from_millis(100)).await;

    driver.run(&mut device).await.unwrap();
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
        display: LedMatrix,
        battery: Battery,
        sensor: Sensor,
    ) -> Self {
        Self {
            front: Front {
                display: DisplayOnOff::new(display),
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

struct ButtonOnOff {
    button: Button,
}

impl ButtonOnOff {
    fn new(button: Button) -> Self {
        Self { button }
    }
}

impl BluetoothMeshModel<GenericOnOffClient> for ButtonOnOff {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericOnOffClient> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<GenericOnOffClient> + 'run>(
        &'run mut self,
        ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            loop {
                self.button.wait_for_any_edge().await;
                let message = GenericOnOffMessage::Set(GenericOnOffSet {
                    on_off: if self.button.is_low() { 1 } else { 0 },
                    tid: 0,
                    transition_time: None,
                    delay: None,
                });
                match ctx.publish(message).await {
                    Ok(_) => {
                        defmt::info!("Published button status ");
                    }
                    Err(e) => {
                        defmt::warn!("Error publishing button status: {:?}", e);
                    }
                }
            }
        }
    }
}

struct DisplayOnOff {
    display: LedMatrix,
}

impl DisplayOnOff {
    fn new(display: LedMatrix) -> Self {
        Self { display }
    }
}

impl BluetoothMeshModel<GenericOnOffServer> for DisplayOnOff {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericOnOffServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<GenericOnOffServer> + 'run>(
        &'run mut self,
        ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            loop {
                let (message, _meta) = ctx.receive().await;
                match message {
                    GenericOnOffMessage::Get => {}
                    GenericOnOffMessage::Set(val) => {
                        if val.on_off != 0 {
                            self.display.scroll("ON").await;
                        }
                    }
                    GenericOnOffMessage::SetUnacknowledged(val) => {
                        if val.on_off != 0 {
                            self.display.scroll("OFF").await;
                        }
                    }
                    GenericOnOffMessage::Status(_) => {
                        // not applicable
                    }
                }
            }
        }
    }
}

pub struct Battery {
    interval: Duration,
}

impl Battery {
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }

    async fn read(&mut self) -> BatteryStatus {
        BatteryStatus::new(
            0,
            0,
            0,
            GenericBatteryFlags {
                presence: GenericBatteryFlagsPresence::Unknown,
                indicator: GenericBatteryFlagsIndicator::Unknown,
                charging: GenericBatteryFlagsCharging::Unknown,
            },
        )
    }
}

impl BluetoothMeshModel<GenericBatteryServer> for Battery {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericBatteryServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<GenericBatteryServer> + 'run>(
        &'run mut self,
        ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            let mut tick = Ticker::every(self.interval);
            loop {
                match select(ctx.receive(), tick.next()).await {
                    Either::First((message, meta)) => {
                        defmt::info!("Received message: {:?}", message);
                        match message {
                            GenericBatteryMessage::Get => {
                                let message = GenericBatteryMessage::Status(self.read().await);
                                match ctx.send(message, meta.reply()).await {
                                    Ok(_) => {
                                        defmt::info!("Published battery status ");
                                    }
                                    Err(e) => {
                                        defmt::warn!("Error publishing battery status: {:?}", e);
                                    }
                                }
                            }
                            GenericBatteryMessage::Status(_) => {}
                        }
                    }
                    Either::Second(_) => {
                        let message = GenericBatteryMessage::Status(self.read().await);
                        match ctx.publish(message).await {
                            Ok(_) => {
                                defmt::info!("Published battery status ");
                            }
                            Err(e) => {
                                defmt::warn!("Error publishing battery status: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}

type SensorServer = SensorSetupServer<MicrobitSensorConfig, 1, 1>;

pub struct Sensor {
    interval: Duration,
    sd: &'static Softdevice,
}

impl Sensor {
    pub fn new(interval: Duration, sd: &'static Softdevice) -> Self {
        Self { interval, sd }
    }

    async fn read(&mut self) -> Result<SensorPayload, ()> {
        let temperature: i8 = temperature_celsius(self.sd).map_err(|_| ())?.to_num();
        Ok(SensorPayload {
            temperature: temperature * 2,
        })
    }
}

impl BluetoothMeshModel<SensorServer> for Sensor {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<SensorServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<SensorServer> + 'run>(
        &'run mut self,
        ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            let mut tick = Ticker::every(self.interval);
            loop {
                match select(ctx.receive(), tick.next()).await {
                    Either::First(msg) => {
                        defmt::info!("Received message: {:?}", msg);
                    }
                    Either::Second(_) => match self.read().await {
                        Ok(result) => {
                            defmt::info!("Read sensor data: {:?}", result);
                            let message = SensorSetupMessage::Sensor(SensorMessage::Status(
                                SensorStatus::new(result),
                            ));
                            match ctx.publish(message).await {
                                Ok(_) => {
                                    defmt::info!("Published sensor reading");
                                }
                                Err(e) => {
                                    defmt::warn!("Error publishing sensor reading: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            defmt::warn!("Error reading sensor data: {:?}", e);
                        }
                    },
                }
            }
        }
    }
}
