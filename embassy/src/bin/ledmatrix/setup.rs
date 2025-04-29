use embassy_stm32::gpio::Output;
use max7219::DataError;

type Max7219 = max7219::MAX7219<max7219::connectors::PinConnector<Output<'static>, Output<'static>, Output<'static>>>;

pub fn setup_display(cs: Output<'static>, sck: Output<'static>, data: Output<'static>) -> Result<Max7219, DataError> {
    /*
    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);
    let spi = Spi::new_blocking(p.SPI2, p.PB13, p.PB15, p.PB14, spi_config);
    */

    //let mut display = max7219::MAX7219::from_spi(4, spi).unwrap();
    let mut display = max7219::MAX7219::from_pins(4, data, cs, sck)?;

    display.power_on()?;

    let intensity = 0x0F; // 0x00 - 0x0F
    display.set_intensity(0, intensity)?;
    display.set_intensity(1, intensity)?;
    display.set_intensity(2, intensity)?;
    display.set_intensity(3, intensity)?;

    Ok(display)
}
