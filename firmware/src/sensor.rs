use crate::mic::*;
use btmesh_device::{
    BluetoothMeshModel, BluetoothMeshModelContext, Control, InboundModelPayload, PublicationCadence,
};
use btmesh_models::sensor::SensorStatus;
use core::future::Future;
use embassy_futures::select::{select, Either};
use embassy_time::Ticker;
use futures::StreamExt;
use microbit_bsp::accelerometer::Accelerometer;
use nrf_softdevice::{temperature_celsius, Softdevice};

use sensor_model::*;

// A sensor type implementing the SensorSetupServer model.
#[allow(dead_code)]
pub struct Sensor {
    // This field is required to access some peripherals that is also controlled by the radio driver
    sd: &'static Softdevice,
    ticker: Option<Ticker>,
    xl: Accelerometer<'static>,
    mic: Microphone<'static>,
}

impl Sensor {
    pub fn new(
        sd: &'static Softdevice,
        xl: Accelerometer<'static>,
        mic: Microphone<'static>,
    ) -> Self {
        Self {
            sd,
            ticker: None,
            xl,
            mic,
        }
    }

    // Read the current on-chip temperature
    async fn read(&mut self) -> Result<SensorPayload, ()> {
        let temperature: i8 = temperature_celsius(self.sd).map_err(|_| ())?.to_num();

        // TODO Accelerometer - Read the accelerometer data and add to the sensor payload,
        let accel = Acceleration::default();

        // TODO Microphone - Read the sound level data and add to sensor payload,
        let noise: u8 = 0;

        Ok(SensorPayload {
            temperature: temperature * 2,
            acceleration: accel,
            noise,
        })
    }

    // Process an inbound control message
    async fn process(&mut self, data: &InboundModelPayload<SensorMessage>) {
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
        ctx: C,
    ) -> Self::RunFuture<'_, C> {
        async move {
            loop {
                if let Some(ticker) = self.ticker.as_mut() {
                    // When ticker is enabled, we emit sensor readings on each tick.
                    match select(ctx.receive(), ticker.next()).await {
                        Either::First(data) => self.process(&data).await,
                        Either::Second(_) => match self.read().await {
                            Ok(result) => {
                                defmt::info!("Read sensor data: {:?}", result);
                                let message = SensorMessage::Status(SensorStatus::new(result));
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
                    // When ticker is disabled, we wait for commands.
                    let m = ctx.receive().await;
                    self.process(&m).await;
                }
            }
        }
    }
}
