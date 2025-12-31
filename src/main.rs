#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::Peri; 
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// --- BLINK TASK ---
#[embassy_executor::task(pool_size = 2)]
async fn blink_task(pin: Peri<'static, AnyPin>, interval_ms: u64) {
    // Output::new handles the Peri type directly
    let mut led = Output::new(pin, Level::Low, Speed::Low);

    loop {
        info!("Blink task toggling LED");
        led.toggle();
        Timer::after_millis(interval_ms).await;
    }
}

// --- MAIN ---
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Main started!");

    // Blue LED (PC8)
    spawner.spawn(blink_task(p.PC8.into(), 500)).unwrap();

    // Green LED (PC9)
    spawner.spawn(blink_task(p.PC9.into(), 200)).unwrap();

    loop {
        Timer::after_secs(10).await;
        info!("Main loop heartbeat");
    }
}