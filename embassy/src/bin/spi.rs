#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!"); 
    /*
    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);
    let spi = Spi::new_blocking(p.SPI2, p.PB13, p.PB15, p.PB14, spi_config);
    */

    let mut cs = Output::new(p.PB12, Level::High, Speed::VeryHigh);
    let mut sck = Output::new(p.PB13, Level::High, Speed::VeryHigh);
    let mut data = Output::new(p.PB15, Level::High, Speed::VeryHigh);

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    //let mut display = max7219::MAX7219::from_spi(4, spi).unwrap();
    let mut display = max7219::MAX7219::from_pins(4, data, cs, sck).unwrap();

    display.power_on().unwrap();
    display.set_intensity(0, 0x1).unwrap();

    loop {
        //cs.set_low();
        led.set_low();
        //display.write_str(0, b"pls help", 0b00100000).unwrap();
        display.write_raw(0, &[0b100, 1, 0b10, 0, 0, 0, 0, 0]);
        //cs.set_high();
        led.set_high();

        Timer::after_millis(1000).await;
    }
}
