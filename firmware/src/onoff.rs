use crate::speaker::*;
use btmesh_device::{BluetoothMeshModel, BluetoothMeshModelContext, InboundModelPayload};
use btmesh_models::generic::onoff::{GenericOnOffMessage, GenericOnOffServer};
use core::future::Future;
use embassy_futures::select::{select3, Either3};
use embassy_time::{Duration, Instant, Timer};
use microbit_bsp::{
    display::{fonts, Brightness, Frame},
    LedMatrix,
};

/// A display type implementing the GenericOnOffServer model.
pub struct OnOff {
    display: LedMatrix,
    speaker: Speaker,
}

impl OnOff {
    pub fn new(display: LedMatrix, speaker: Speaker) -> Self {
        Self { display, speaker }
    }

    /// Wait for onoff messages and return whether display should be enabled or not
    async fn process<C: BluetoothMeshModelContext<GenericOnOffServer>>(ctx: &mut C) -> bool {
        loop {
            match ctx.receive().await {
                InboundModelPayload::Message(message, _) => {
                    match message {
                        GenericOnOffMessage::Get => {}
                        GenericOnOffMessage::Set(val) => {
                            defmt::info!("Enabling display: {}", val.on_off != 0);
                            return val.on_off != 0;
                        }
                        GenericOnOffMessage::SetUnacknowledged(val) => {
                            return val.on_off != 0;
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

    /// Rendering loop for the blinker process.
    async fn blinker(display: &mut LedMatrix) {
        // Enable all LEDs
        const BITMAP: Frame<5, 5> =
            fonts::frame_5x5(&[0b11111, 0b11111, 0b11111, 0b11111, 0b11111]);

        // For each blink iteration does the following:
        // - Set brightness to minimum
        // - Enable bitmap to frame buffer
        // - Gradually increase brightness until reaching max, then
        // - Gradually decrease brightness until reaching min.
        // - Pause for 1 second before next iteration
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

    #[allow(unused_variables)]
    async fn jukebox(speaker: &mut Speaker) {
        loop {
            // TODO Speaker
            // - Modify this section to play a riff (a collection of notes) on the speaker using the speaker instance
            Timer::after(Duration::from_secs(60)).await;
        }
    }
}

// Required trait implementation to be enabled in a Bluetooth mesh device.
impl BluetoothMeshModel<GenericOnOffServer> for OnOff {
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
                    match select3(
                        Self::blinker(&mut self.display),
                        Self::jukebox(&mut self.speaker),
                        Self::process(&mut ctx),
                    )
                    .await
                    {
                        Either3::First(_) => {}
                        Either3::Second(_) => {}
                        Either3::Third(e) => {
                            self.display.clear();
                            enable = e
                        },
                    }
                } else {
                    // When blinking is disabled, we just await incoming messages.
                    enable = Self::process(&mut ctx).await;
                }
            }
        }
    }
}
