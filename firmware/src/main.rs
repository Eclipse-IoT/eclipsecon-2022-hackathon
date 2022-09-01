#![no_std]
#![no_main]
#![macro_use]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use btmesh_device::{
    BluetoothMeshModel, BluetoothMeshModelContext, Control, InboundModelPayload, PublicationCadence,
};
use btmesh_macro::{device, element};
use btmesh_models::{
    generic::{
        battery::{
            GenericBatteryFlags, GenericBatteryFlagsCharging, GenericBatteryFlagsIndicator,
            GenericBatteryFlagsPresence, GenericBatteryMessage, GenericBatteryServer,
            GenericBatteryStatus,
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
use embassy_futures::{select, Either};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::{Duration, Instant, Ticker, Timer};
use futures::StreamExt;
use microbit_async::{
    display::{fonts, Brightness, Frame},
    *,
};
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

type CS = CriticalSectionRawMutex;
type BlinkChannel = Channel<CS, BlinkCommand, 1>;
type BlinkSender = Sender<'static, CS, BlinkCommand, 1>;
type BlinkReceiver = Receiver<'static, CS, BlinkCommand, 1>;

enum BlinkCommand {
    Start,
    Stop,
}

#[embassy_executor::main]
async fn main(s: Spawner) {
    let board = Microbit::new(config());

    let mut driver = Driver::new("drogue", unsafe { &__storage as *const u8 as u32 }, 100);

    static BLINKCHAN: BlinkChannel = BlinkChannel::new();
    let sd = driver.softdevice();
    let sensor = Sensor::new(sd);
    let battery = Battery::new();
    let display = DisplayOnOff::new(BLINKCHAN.sender());

    s.spawn(blinker(board.display, BLINKCHAN.receiver()))
        .unwrap();

    let mut device = Device::new(board.btn_a, board.btn_b, display, battery, sensor);

    // Give flash some time before accessing
    Timer::after(Duration::from_millis(100)).await;

    driver.run(&mut device).await.unwrap();
}

#[embassy_executor::task]
async fn blinker(mut display: LedMatrix, commands: BlinkReceiver) {
    let mut enable = false;
    loop {
        if enable {
            match select(rendering(&mut display), commands.recv()).await {
                Either::First(_) => {}
                Either::Second(BlinkCommand::Start) => enable = true,
                Either::Second(BlinkCommand::Stop) => enable = false,
            }
        } else {
            match commands.recv().await {
                BlinkCommand::Start => enable = true,
                BlinkCommand::Stop => enable = false,
            }
        }
    }
}

async fn rendering(display: &mut LedMatrix) {
    const BITMAP: Frame<5, 5> = fonts::frame_5x5(&[0b11111, 0b11111, 0b11111, 0b11111, 0b11111]);
    loop {
        display.set_brightness(Brightness::MIN);
        display.apply(BITMAP);

        let interval = Duration::from_millis(50);
        let end = Instant::now() + Duration::from_millis(600);
        while Instant::now() < end {
            let _ = display.increase_brightness();
            display.display(BITMAP, interval).await;
        }

        let end = Instant::now() + Duration::from_millis(400);
        while Instant::now() < end {
            let _ = display.decrease_brightness();
            display.display(BITMAP, interval).await;
        }
        display.clear();

        Timer::after(Duration::from_secs(1)).await;
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

pub struct DisplayOnOff {
    sender: BlinkSender,
}

impl DisplayOnOff {
    fn new(sender: BlinkSender) -> Self {
        Self { sender }
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
                match ctx.receive().await {
                    InboundModelPayload::Message(message, _) => {
                        match message {
                            GenericOnOffMessage::Get => {}
                            GenericOnOffMessage::Set(val) => {
                                if val.on_off != 0 {
                                    defmt::info!("Display ON");
                                    self.sender.send(BlinkCommand::Start).await;
                                } else {
                                    defmt::info!("Display OFF");
                                    self.sender.send(BlinkCommand::Stop).await;
                                }
                            }
                            GenericOnOffMessage::SetUnacknowledged(val) => {
                                if val.on_off != 0 {
                                    defmt::info!("Display ON");
                                    self.sender.send(BlinkCommand::Start).await;
                                } else {
                                    defmt::info!("Display OFF");
                                    self.sender.send(BlinkCommand::Stop).await;
                                }
                            }
                            GenericOnOffMessage::Status(_) => {
                                // not applicable
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct Battery {
    ticker: Option<Ticker>,
}

impl Battery {
    pub fn new() -> Self {
        Self { ticker: None }
    }

    async fn read(&mut self) -> GenericBatteryStatus {
        GenericBatteryStatus::new(
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

    async fn process<C: BluetoothMeshModelContext<GenericBatteryServer>>(
        &mut self,
        ctx: &mut C,
        data: &InboundModelPayload<GenericBatteryMessage>,
    ) {
        match data {
            InboundModelPayload::Message(message, meta) => {
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
            InboundModelPayload::Control(Control::PublicationCadence(cadence)) => match cadence {
                PublicationCadence::Periodic(cadence) => {
                    defmt::info!("Enabling battery publish at {:?}", cadence.as_secs());
                    self.ticker.replace(Ticker::every(*cadence));
                }
                PublicationCadence::OnChange => {
                    defmt::info!("Battery publish on change!");
                    self.ticker.take();
                }
                PublicationCadence::None => {
                    defmt::info!("Disabling battery publish");
                    self.ticker.take();
                }
            },
            _ => {}
        }
    }
}

impl BluetoothMeshModel<GenericBatteryServer> for Battery {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericBatteryServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<GenericBatteryServer> + 'run>(
        &'run mut self,
        mut ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            loop {
                if let Some(ticker) = self.ticker.as_mut() {
                    match select(ctx.receive(), ticker.next()).await {
                        Either::First(data) => self.process(&mut ctx, &data).await,
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
                } else {
                    let m = ctx.receive().await;
                    self.process(&mut ctx, &m).await;
                }
            }
        }
    }
}

type SensorServer = SensorSetupServer<MicrobitSensorConfig, 1, 1>;

pub struct Sensor {
    sd: &'static Softdevice,
    ticker: Option<Ticker>,
}

impl Sensor {
    pub fn new(sd: &'static Softdevice) -> Self {
        Self { sd, ticker: None }
    }

    async fn read(&mut self) -> Result<SensorPayload, ()> {
        let temperature: i8 = temperature_celsius(self.sd).map_err(|_| ())?.to_num();
        Ok(SensorPayload {
            temperature: temperature * 2,
        })
    }

    async fn process<C: BluetoothMeshModelContext<SensorServer>>(
        &mut self,
        _ctx: &mut C,
        data: &InboundModelPayload<SensorSetupMessage<MicrobitSensorConfig, 1, 1>>,
    ) {
        match data {
            InboundModelPayload::Control(Control::PublicationCadence(cadence)) => match cadence {
                PublicationCadence::Periodic(cadence) => {
                    defmt::info!("Enabling sensor publish at {:?}", cadence.as_secs());
                    self.ticker.replace(Ticker::every(*cadence));
                }
                PublicationCadence::OnChange => {
                    defmt::info!("Sensor publish on change!");
                    self.ticker.take();
                }
                PublicationCadence::None => {
                    defmt::info!("Disabling sensor publish");
                    self.ticker.take();
                }
            },
            _ => {}
        }
    }
}

impl BluetoothMeshModel<SensorServer> for Sensor {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<SensorServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<SensorServer> + 'run>(
        &'run mut self,
        mut ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            loop {
                if let Some(ticker) = self.ticker.as_mut() {
                    match select(ctx.receive(), ticker.next()).await {
                        Either::First(data) => self.process(&mut ctx, &data).await,
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
                } else {
                    let m = ctx.receive().await;
                    self.process(&mut ctx, &m).await;
                }
            }
        }
    }
}
