#![allow(dead_code)]
use embassy_nrf::{
    peripherals::{P0_00, PWM0},
    pwm,
};
use microbit_async::speaker::*;
pub use microbit_async::speaker::{Note, Pitch};

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
}
