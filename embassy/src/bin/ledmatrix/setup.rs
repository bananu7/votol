use embassy_stm32::gpio::{Output};

pub fn setup_display(cs: Output<'static>, sck: Output<'static>, data: Output<'static>) -> max7219::MAX7219<max7219::connectors::PinConnector<Output<'static>, Output<'static>, Output<'static>>> {
    /*
    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);
    let spi = Spi::new_blocking(p.SPI2, p.PB13, p.PB15, p.PB14, spi_config);
    */

    //let mut display = max7219::MAX7219::from_spi(4, spi).unwrap();
    let mut display = max7219::MAX7219::from_pins(4, data, cs, sck).unwrap();

    display.power_on().unwrap();
    display.set_intensity(0, 0x0).unwrap();

    display
}