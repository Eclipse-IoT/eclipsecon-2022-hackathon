use btmesh_device::{
    BluetoothMeshModel, BluetoothMeshModelContext, Control, InboundModelPayload, PublicationCadence,
};
use btmesh_models::generic::battery::{
    GenericBatteryFlags, GenericBatteryFlagsCharging, GenericBatteryFlagsIndicator,
    GenericBatteryFlagsPresence, GenericBatteryMessage, GenericBatteryServer, GenericBatteryStatus,
};
use core::future::Future;
use embassy_futures::select::{select, Either};
use embassy_time::Ticker;
use futures::StreamExt;

/// A type implementing the GenericBatteryServer model, capable of periodically publishing the status of the battery.
pub struct Battery {
    ticker: Option<Ticker>,
}

impl Battery {
    pub fn new() -> Self {
        Self { ticker: None }
    }

    // Read the current battery status.
    //
    // TODO: read actual battery status instead of faking it
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

    // Process an incoming command.
    async fn process<C: BluetoothMeshModelContext<GenericBatteryServer>>(
        &mut self,
        ctx: &mut C,
        data: &InboundModelPayload<GenericBatteryMessage>,
    ) {
        match data {
            InboundModelPayload::Message(message, meta) => {
                defmt::info!("Received message: {:?}", message);
                match message {
                    // A request asking our device specifically for the battery status. Send the status back.
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
            // A control message setting or adjusting the publication settings, which may enable the ticker.
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
                    // When ticker is enabled, we publish a status message on each tick.
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
                    // When ticker is disabled, we just wait for commands.
                    let m = ctx.receive().await;
                    self.process(&mut ctx, &m).await;
                }
            }
        }
    }
}
