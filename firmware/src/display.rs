use btmesh_device::{BluetoothMeshModel, BluetoothMeshModelContext, InboundModelPayload};
use btmesh_models::generic::onoff::{GenericOnOffMessage, GenericOnOffServer};
use core::future::Future;
use embassy_futures::select::{select, Either};
use embassy_time::{Duration, Instant, Timer};
use microbit_async::{
    display::{fonts, Brightness, Frame},
    LedMatrix,
};

pub struct DisplayOnOff {
    display: LedMatrix,
}

impl DisplayOnOff {
    pub fn new(display: LedMatrix) -> Self {
        Self { display }
    }

    async fn process<C: BluetoothMeshModelContext<GenericOnOffServer>>(ctx: &mut C) -> bool {
        loop {
            match ctx.receive().await {
                InboundModelPayload::Message(message, _) => {
                    match message {
                        GenericOnOffMessage::Get => {}
                        GenericOnOffMessage::Set(val) => {
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

    async fn blinker(display: &mut LedMatrix) {
        const BITMAP: Frame<5, 5> =
            fonts::frame_5x5(&[0b11111, 0b11111, 0b11111, 0b11111, 0b11111]);

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
}

impl BluetoothMeshModel<GenericOnOffServer> for DisplayOnOff {
    type RunFuture<'f, C> = impl Future<Output=Result<(), ()>> + 'f
    where
        Self: 'f,
        C: BluetoothMeshModelContext<GenericOnOffServer> + 'f;

    fn run<'run, C: BluetoothMeshModelContext<GenericOnOffServer> + 'run>(
        &'run mut self,
        mut ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            loop {
                let mut enable = false;
                loop {
                    if enable {
                        match select(Self::blinker(&mut self.display), Self::process(&mut ctx))
                            .await
                        {
                            Either::First(_) => {}
                            Either::Second(e) => enable = e,
                        }
                    } else {
                        enable = Self::process(&mut ctx).await;
                    }
                }
            }
        }
    }
}
