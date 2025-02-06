#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;

/* max7219 digit helpers */
fn output_digit(digit: u8) -> [u8; 8] {
    let to_write = match digit {
        0 => [
            0b11100000,
            0b10100000,
            0b10100000,
            0b10100000,
            0b10100000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
        1 => [
            0b01000000,
            0b11000000,
            0b01000000,
            0b01000000,
            0b01000000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
        2 => [
            0b11100000,
            0b00100000,
            0b11100000,
            0b10000000,
            0b10000000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
        3 => [
            0b11100000,
            0b00100000,
            0b11100000,
            0b00100000,
            0b00100000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
        4 => [
            0b10100000,
            0b10100000,
            0b11100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00000000,
            0b00000000,
        ],
        5 => [
            0b11100000,
            0b10000000,
            0b11100000,
            0b00100000,
            0b00100000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
        6 => [
            0b11100000,
            0b10000000,
            0b11100000,
            0b10100000,
            0b10100000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
        7 => [
            0b11100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00000000,
            0b00000000,
        ],
        8 => [
            0b11100000,
            0b10100000,
            0b11100000,
            0b10100000,
            0b10100000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
        9 => [
            0b11100000,
            0b10100000,
            0b11100000,
            0b00100000,
            0b00100000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
        _=> [
            0b11100000,
            0b11100000,
            0b11100000,
            0b11100000,
            0b11100000,
            0b11100000,
            0b00000000,
            0b00000000,
        ],
    };
    to_write
}

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
    display.set_intensity(0, 0x0).unwrap();

    let mut counter = 0u8;
    loop {
        led.set_low();
        // first value is first row, bits are cols
        display.write_raw(0, &output_digit(counter));
        display.write_raw(1, &output_digit(counter+1));
        display.write_raw(3, &output_digit(counter+2));

        counter += 1;
        if (counter > 9) {
            counter = 0;
        }

        led.set_high();

        Timer::after_millis(1000).await;
    }
}
