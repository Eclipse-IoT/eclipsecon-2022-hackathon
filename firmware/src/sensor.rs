use btmesh_device::{
    BluetoothMeshModel, BluetoothMeshModelContext, Control, InboundModelPayload, PublicationCadence,
};
use btmesh_models::sensor::{SensorMessage, SensorSetupMessage, SensorSetupServer, SensorStatus};
use core::future::Future;
use embassy_futures::select::{select, Either};
use embassy_time::Ticker;
use futures::StreamExt;
use nrf_softdevice::{temperature_celsius, Softdevice};
use sensor_model::*;

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
