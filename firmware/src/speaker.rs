#![allow(dead_code)]
use btmesh_device::{BluetoothMeshModel, BluetoothMeshModelContext, InboundModelPayload};
use btmesh_models::generic::onoff::{GenericOnOffMessage, GenericOnOffServer};
use core::future::Future;
use embassy_futures::select::{select, Either};
use embassy_nrf::{
    peripherals::{P0_00, PWM0},
    pwm,
};
use embassy_time::{Duration, Timer};
use microbit_bsp::speaker::*;
pub use microbit_bsp::speaker::{Note, Pitch};

pub struct Speaker {
    speaker: PwmSpeaker<'static, PWM0>,
}

impl Speaker {
    pub fn new(pwm0: PWM0, speaker: P0_00) -> Self {
        let pwm = pwm::SimplePwm::new_1ch(pwm0, speaker);
        let speaker = PwmSpeaker::new(pwm);
        Self { speaker }
    }

    pub async fn play(&mut self, riff: &[Note]) {
        for note in riff {
            self.speaker.play(note).await;
        }
    }

    /// Wait for onoff messages and return whether display should be enabled or not
    async fn process<C: BluetoothMeshModelContext<GenericOnOffServer>>(ctx: &mut C) -> bool {
        loop {
            match ctx.receive().await {
                InboundModelPayload::Message(message, _) => match message {
                    GenericOnOffMessage::Set(val) => {
                        defmt::info!("Enabling speaker: {}", val.on_off != 0);
                        return val.on_off != 0;
                    }
                    GenericOnOffMessage::SetUnacknowledged(val) => {
                        defmt::info!("Enabling speaker: {}", val.on_off != 0);
                        return val.on_off != 0;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    #[allow(unused_variables)]
    async fn jukebox(speaker: &mut PwmSpeaker<'static, PWM0>) {
        loop {
            // TODO Speaker
            // - Modify this section to play a riff (a collection of notes) on the speaker using the speaker instance
            Timer::after(Duration::from_secs(10)).await;
        }
    }
}

// Required trait implementation to be enabled in a Bluetooth mesh device.
impl BluetoothMeshModel<GenericOnOffServer> for Speaker {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericOnOffServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<GenericOnOffServer> + 'run>(
        &'run mut self,
        mut ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            let mut enable = false;
            loop {
                if enable {
                    // When blinking is enabled, we need to await both the rendering loop and the inbound message processing.
                    match select(Self::jukebox(&mut self.speaker), Self::process(&mut ctx)).await {
                        Either::First(_) => {}
                        Either::Second(e) => enable = e,
                    }
                } else {
                    // When blinking is disabled, we just await incoming messages.
                    enable = Self::process(&mut ctx).await;
                }
            }
        }
    }
}
