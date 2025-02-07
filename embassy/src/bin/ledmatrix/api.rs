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

// flips the bit in a byte the other way around, e.g.
// 0b00000111 -> 0b11100000
fn flip_byte(b: u8) -> u8 {
    let lookup = [
        0x0, 0x8, 0x4, 0xc, 0x2, 0xa, 0x6, 0xe,
        0x1, 0x9, 0x5, 0xd, 0x3, 0xb, 0x7, 0xf, 
    ];

    (lookup[(b&0b1111) as usize] << 4) | lookup[(b>>4) as usize]
}

pub fn write_battery_bar<CONN: Connector>(voltage: u16, display: &mut max7219::MAX7219<CONN>) {
    // assuming Vmax = 4.2V, Vmin = 2.75V
    // 20s means 55.0-84.0V swing.

    let v_max = 840;
    let v_min = 550;

    // temporary
    let v = voltage * 3;

    // we need a value from 1 to 32
    // multiply first otherwise it would go to 0-1

    let number_of_leds = ((v - v_min) * 32) / (v_max-v_min);
    let bitmask: u32 = (1 << (number_of_leds+1)) - 1;

    // write to last row of bits on each display
    let a = flip_byte(((bitmask & 0x000000FF) >> 0) as u8);
    let b = flip_byte(((bitmask & 0x0000FF00) >> 8) as u8);
    let c = flip_byte(((bitmask & 0x00FF0000) >> 16) as u8);
    let d = flip_byte(((bitmask & 0xFF000000) >> 24) as u8);

    display.write_raw(0, &[0,0,0,0,0,0,0, a]);
    display.write_raw(1, &[0,0,0,0,0,0,0, b]);
    display.write_raw(2, &[0,0,0,0,0,0,0, c]);
    display.write_raw(3, &[0,0,0,0,0,0,0, d]);
}