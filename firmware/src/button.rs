use btmesh_device::{BluetoothMeshModel, BluetoothMeshModelContext};
use btmesh_models::generic::onoff::{
    GenericOnOffClient, GenericOnOffMessage, Set as GenericOnOffSet,
};
use core::future::Future;
use microbit_async::*;

pub struct ButtonOnOff {
    button: Button,
}

impl ButtonOnOff {
    pub fn new(button: Button) -> Self {
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
