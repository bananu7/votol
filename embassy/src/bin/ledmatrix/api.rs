use crate::ledmatrix::digits::output_digit;
use max7219::connectors::Connector;

// just to test
pub fn write_fullscreen_voltage<CONN: Connector>(voltage: u16, display: &mut max7219::MAX7219<CONN>) {
    // fixed point in 0.1
    let v = (voltage / 10) as u8;
    let frac = (voltage % 10) as u8;

    display.write_raw(0, &output_digit(v / 10));
    display.write_raw(1, &output_digit(v % 10));
    display.write_raw(3, &output_digit(frac));
}

