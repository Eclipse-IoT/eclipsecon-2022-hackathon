//! microphone peripheral
use crate::adc::SharedAdc;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive},
    peripherals::{P0_05, P0_20},
    saadc::*,
};
use embassy_time::{Duration, Timer};

/// Microphone interface
pub struct Microphone<'a> {
    adc: &'a SharedAdc,
    enable: Output<'a, P0_20>,
    mic: P0_05,
}

impl<'a> Microphone<'a> {
    /// Create a new microphone instance
    pub fn new(adc: &'a SharedAdc, mic: P0_05, micen: P0_20) -> Self {
        let enable = Output::new(micen, Level::Low, OutputDrive::HighDrive);
        Self { adc, enable, mic }
    }

    /// Enable the microphone and return the sound level as detected by the microphone.
    ///
    /// The returned value is a number between 0 and 255 and does not correspond to any official sound level meter number.
    pub async fn sound_level(&mut self) -> u8 {
        self.enable.set_high();
        Timer::after(Duration::from_millis(10)).await;

        let mut bufs = [[[0; 1]; 1024]; 2];

        let mut channel = ChannelConfig::single_ended(&mut self.mic);
        channel.gain = Gain::GAIN4;
        let mut adc = self.adc.lock().await;
        let mut adc = adc.configure(Config::default(), [channel; 1]);
        adc.run_timer_sampler::<u32, _, 1024>(&mut bufs, 727, move |_| SamplerState::Stopped)
            .await;
        self.enable.set_low();

        let mut max: i16 = i16::MIN;
        let mut min: i16 = i16::MAX;
        for b in bufs[0] {
            if b[0] > max {
                max = b[0];
            }
            if b[0] < min {
                min = b[0];
            }
        }
        let amplitude = max - min;
        // Transpose to u8
        (amplitude / 16) as u8
    }
}
