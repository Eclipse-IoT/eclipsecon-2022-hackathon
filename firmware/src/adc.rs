use embassy_nrf::{interrupt, peripherals::SAADC, saadc::*};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

pub type SharedAdc = Mutex<CriticalSectionRawMutex, Adc>;

pub struct Adc {
    saadc: SAADC,
    irq: interrupt::SAADC,
}

impl Adc {
    pub fn new(saadc: SAADC, irq: interrupt::SAADC) -> Self {
        Self { saadc, irq }
    }

    pub fn configure<'m, const N: usize>(
        &'m mut self,
        config: Config,
        channels: [ChannelConfig; N],
    ) -> Saadc<'m, N> {
        let saadc = Saadc::new(&mut self.saadc, &mut self.irq, config, channels);
        saadc
    }
}
