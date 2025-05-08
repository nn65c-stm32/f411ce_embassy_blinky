#![no_std]
#![no_main]

use defmt::{error, info, trace};
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::{
    Peripherals,
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn blink_led(mut led: Output<'static>) {
    loop {
        led.toggle();
        trace!("LED blink!");
        Timer::after_millis(250).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p: Peripherals = embassy_stm32::init(Default::default());

    info!("Start");

    let led = Output::new(p.PC13, Level::Low, Speed::High);
    if let Err(e) = spawner.spawn(blink_led(led)) {
        error!("Failed to spawn blink_led task: {:?}", e);
    }

    let mut button = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up);
    loop {
        button.wait_for_any_edge().await;
        info!("Button change!");
    }
}
