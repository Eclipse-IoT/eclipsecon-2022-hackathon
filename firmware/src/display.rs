use btmesh_device::{BluetoothMeshModel, BluetoothMeshModelContext, InboundModelPayload};
use btmesh_models::generic::level::*;
use core::future::Future;
use embassy_futures::select::{select, Either};
use embassy_time::{Duration, Instant, Ticker, Timer};
use futures::StreamExt;
use microbit_bsp::{
    display::{fonts, Brightness, Frame},
    LedMatrix,
};

/// A display type implementing the GenericLevelServer model.
pub struct Display {
    display: LedMatrix,
}

impl Display {
    pub fn new(display: LedMatrix) -> Self {
        Self { display }
    }

    /// Wait for onoff messages and return whether display should be enabled or not
    async fn process<C: BluetoothMeshModelContext<GenericLevelServer>>(
        ctx: &mut C,
    ) -> Option<Brightness> {
        loop {
            match ctx.receive().await {
                InboundModelPayload::Message(message, _) => match message {
                    GenericLevelMessage::Set(val) => {
                        let level: u8 = val.level.clamp(u8::MIN as i16, u8::MAX as i16) as u8;
                        defmt::info!("Setting display level: {}", level);
                        if level == 0 {
                            return None;
                        } else {
                            return Some(Brightness::new(level));
                        }
                    }
                    GenericLevelMessage::SetUnacknowledged(val) => {
                        let level: u8 = val.level.clamp(u8::MIN as i16, u8::MAX as i16) as u8;
                        defmt::info!("Setting display level: {}", level);
                        if level == 0 {
                            return None;
                        } else {
                            return Some(Brightness::new(level));
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    /// Display the provided frame for the duration. Handles screen refresh
    /// in an async display loop.
    pub async fn display(display: &mut LedMatrix, frame: Frame<5, 5>, length: Duration) {
        display.apply(frame);
        let end = Instant::now() + length;
        let mut ticker = Ticker::every(Duration::from_millis(5));
        while Instant::now() < end {
            display.render();
            ticker.next().await;
        }
        display.clear();
    }

    /// Rendering loop for the blinker process.
    async fn blinker(display: &mut LedMatrix) {
        // Enable all LEDs
        const BITMAP: Frame<5, 5> =
            fonts::frame_5x5(&[0b11111, 0b11111, 0b11111, 0b11111, 0b11111]);

        // For each blink iteration does the following:
        // - Enable bitmap to frame buffer for 1 second
        // - Clear the display
        // - Pause for 1 second before next iteration
        loop {
            Self::display(display, BITMAP, Duration::from_millis(500)).await;
            display.clear();
            Timer::after(Duration::from_secs(1)).await;
        }
    }
}

// Required trait implementation to be enabled in a Bluetooth mesh device.
impl BluetoothMeshModel<GenericLevelServer> for Display {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericLevelServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<GenericLevelServer> + 'run>(
        &'run mut self,
        mut ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            let mut brightness: Option<Brightness> = None;
            loop {
                if let Some(b) = brightness {
                    self.display.set_brightness(b);
                    // When blinking is enabled, we need to await both the rendering loop and the inbound message processing.
                    match select(Self::blinker(&mut self.display), Self::process(&mut ctx)).await {
                        Either::First(_) => {}
                        Either::Second(b) => {
                            brightness = b;
                        }
                    }
                } else {
                    self.display.clear();

                    // When blinking is disabled, we just await incoming messages.
                    brightness = Self::process(&mut ctx).await;
                }
            }
        }
    }
}
