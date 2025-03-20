use crate::ledmatrix::digits::{output_character, output_digit};
use crate::ledmatrix::compositor::{Compositor, WriteMode};

fn chr(c: u8) -> u8 {
    return c + b'0';
}

// just to test
pub fn write_fullscreen_float(value: i16, display: &mut Compositor) {
    // fixed point in 0.1
    let v = (value / 10) as u8;
    let frac = (value % 10) as u8;

    if value < 0 {
        write_char(b'-', 0, 0, display);
    }

    write_char(chr(v / 10), 4, 0, display);
    write_char(chr(v % 10), 8, 0, display);
    display.write_bit(11, 5, true);
    write_char(chr(frac), 12, 0, display);
}

pub fn write_num(number: u8, x: usize, y: usize, display: &mut Compositor) {
    display.blit(0+x, 0+y, 3, 6, &output_digit(number / 10));
    display.blit(4+x, 0+y, 3, 6, &output_digit(number % 10));
}
pub fn write_num_4_digits(number: i16, x: usize, y: usize, display: &mut Compositor) {
    // TODO: negative numbers
    display.blit(0+x, 0+y, 3, 6,  &output_digit(((number % 10000) / 1000) as u8));
    display.blit(4+x, 0+y, 3, 6,  &output_digit(((number % 1000) / 100) as u8));
    display.blit(8+x, 0+y, 3, 6,  &output_digit(((number % 100) / 10) as u8));
    display.blit(12+x, 0+y, 3, 6, &output_digit((number % 10) as u8));
}

pub fn write_char(char: u8, x: usize, y: usize, display: &mut Compositor) {
    display.blit(0+x, 0+y, 3, 6, &output_character(char))
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

pub fn write_battery_bar(voltage: i16, display: &mut Compositor) {
    // assuming Vmax = 4.2V, Vmin = 2.75V
    // 20s means 55.0-84.0V swing.

    let v_max = 840;
    let v_min = 550;

    // we need a value from 1 to 32
    // multiply first otherwise it would go to 0-1

    let number_of_leds = ((voltage - v_min) * 32) / (v_max-v_min);
    let bitmask: u32 = (1 << (number_of_leds+1)) - 1;

    // write to last row of bits on each display
    let a = flip_byte(((bitmask & 0x000000FF) >> 0) as u8);
    let b = flip_byte(((bitmask & 0x0000FF00) >> 8) as u8);
    let c = flip_byte(((bitmask & 0x00FF0000) >> 16) as u8);
    let d = flip_byte(((bitmask & 0xFF000000) >> 24) as u8);

    display.write_raw(0, &[0,0,0,0,0,0,0, a], WriteMode::BLEND);
    display.write_raw(1, &[0,0,0,0,0,0,0, b], WriteMode::BLEND);
    display.write_raw(2, &[0,0,0,0,0,0,0, c], WriteMode::BLEND);
    display.write_raw(3, &[0,0,0,0,0,0,0, d], WriteMode::BLEND);
}
