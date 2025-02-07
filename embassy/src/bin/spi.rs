#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;

pub mod ledmatrix;

use crate::ledmatrix::setup::setup_display;
use crate::ledmatrix::api::write_fullscreen_voltage;

use max7219::connectors::Connector;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!"); 

    let mut cs = Output::new(p.PB12, Level::High, Speed::VeryHigh);
    let mut sck = Output::new(p.PB13, Level::High, Speed::VeryHigh);
    let mut data = Output::new(p.PB15, Level::High, Speed::VeryHigh);

    let mut display = setup_display(cs, sck, data);
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        led.set_low();
        write_fullscreen_voltage(729, &mut display);
        led.set_high();

        Timer::after_millis(1000).await;
    }
}
