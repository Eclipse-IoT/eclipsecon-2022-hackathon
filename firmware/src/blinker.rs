use embassy_futures::select::{select, Either};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::{Duration, Instant, Timer};
use microbit_async::{
    display::{fonts, Brightness, Frame},
    LedMatrix,
};

type CS = CriticalSectionRawMutex;
pub type BlinkChannel = Channel<CS, BlinkCommand, 1>;
pub type BlinkSender = Sender<'static, CS, BlinkCommand, 1>;
pub type BlinkReceiver = Receiver<'static, CS, BlinkCommand, 1>;

pub enum BlinkCommand {
    Start,
    Stop,
}

#[embassy_executor::task]
pub async fn blinker(mut display: LedMatrix, commands: BlinkReceiver) {
    let mut enable = false;
    loop {
        if enable {
            match select(rendering(&mut display), commands.recv()).await {
                Either::First(_) => {}
                Either::Second(BlinkCommand::Start) => enable = true,
                Either::Second(BlinkCommand::Stop) => enable = false,
            }
        } else {
            match commands.recv().await {
                BlinkCommand::Start => enable = true,
                BlinkCommand::Stop => enable = false,
            }
        }
    }
}

async fn rendering(display: &mut LedMatrix) {
    const BITMAP: Frame<5, 5> = fonts::frame_5x5(&[0b11111, 0b11111, 0b11111, 0b11111, 0b11111]);
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
