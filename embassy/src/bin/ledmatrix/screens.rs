use crate::ledmatrix::api::{write_fullscreen_float, write_battery_bar, write_num, write_num_4_digits, write_char};
use crate::ledmatrix::compositor::Compositor;
use crate::can::can_frame::{clamp_temp_to_0, get_battery_current, get_battery_voltage, get_controller_temp, get_external_temp, get_rpm, ThreeVotolFrames};

#[derive(Copy, Clone)]
pub enum DisplayValue {
     Rpm,
     Speed,
     ControllerMotorTemp,
     Voltage,
     Current,
     Power
}

pub fn next(v: DisplayValue) -> DisplayValue {
    match v {
        DisplayValue::Rpm => DisplayValue::Speed,
        DisplayValue::Speed => DisplayValue::ControllerMotorTemp,
        DisplayValue::ControllerMotorTemp => DisplayValue::Voltage,
        DisplayValue::Voltage => DisplayValue::Current,
        DisplayValue::Current => DisplayValue::Power,
        DisplayValue::Power => DisplayValue::Rpm,
    }
}

pub fn ride_screen(frames: &ThreeVotolFrames, value_to_show: DisplayValue, compositor: &mut Compositor) {
    let battery_voltage = get_battery_voltage(&frames);

    let controller_temp = clamp_temp_to_0(get_controller_temp(&frames));
    let external_temp = clamp_temp_to_0(get_external_temp(&frames));
    let rpm = get_rpm(&frames);
    let current = get_battery_current(frames);

    // todo: state or prop?
    match value_to_show {
        DisplayValue::Rpm => {
            write_num_4_digits(get_rpm(&frames), 0, 0, compositor);
            write_char(b'%', 18, 0, compositor);
        }
        DisplayValue::Speed => {
            let mut speed = rpm_to_speed(rpm);
            // TODO speeds over 100
            if speed > 99 {
                speed = 99;
            }

            write_num(speed, 12, 0, compositor);
        }
        DisplayValue::Current => {
            write_num_4_digits(current as i16, 0, 0, compositor);
            write_char(b'a', 18, 0, compositor);
        }
        DisplayValue::ControllerMotorTemp => {
            write_num(controller_temp, 0, 0, compositor);
            write_char(b'*', 8, 0, compositor);
            write_num(external_temp, 14, 0, compositor);
            write_char(b'*', 22, 0, compositor);
        }
        DisplayValue::Voltage => {
            write_fullscreen_float(battery_voltage, compositor);
        }
        DisplayValue::Power => {
            write_num_4_digits(current * battery_voltage/10, 0, 0, compositor);
            write_char(b'w', 18, 0, compositor);
        }
    }

    write_battery_bar(battery_voltage, compositor);

    let mode_char = get_mode_char(frames);
    write_char(mode_char, 28, 0, compositor);
}

fn rpm_to_speed(rpm: i16) -> u8 {
    // todo: runtime config
    let front_sprocket = 14;
    let rear_sprocket = 60;
    let motor_reduction_a = 20;
    let motor_reduction_b = 47;
    let wheel_circumference = 2138; // mm
    let minutes_in_hour = 60;

    // try to get the number large first
    let speed =
        rpm as i32 // use u32 for more precise math
        * front_sprocket
        * motor_reduction_a
        * wheel_circumference
        / 1000
        * minutes_in_hour
        / (
            rear_sprocket
            * motor_reduction_b
            * 1000
        );

    return speed.abs() as u8;
}

fn get_mode_char(frames: &ThreeVotolFrames) -> u8 {
    let status_byte = frames[2][6];

    // todo bits 5,6,7 - antitheft, sidestand, regen
    return if status_byte & 0b10000 != 0 { // brake
        b'b'
    } else if status_byte & 0b1000 != 0 {
        b'p'
    } else if status_byte & 0b100 != 0 {
        b'r'
    } else if status_byte & 0b010 != 0 {
        if status_byte & 0b1 != 0 {
            b's'
        } else {
            b'3'
        }
    } else {
        if status_byte & 0b1 != 0 {
            b'2'
        } else {
            b'1'
        }
    };
}

pub fn fault_screen(_frames: &ThreeVotolFrames, compositor: &mut Compositor) {
    write_char(b'c', 0, 0, compositor);
    write_char(b't', 4, 0, compositor);
    write_char(b'r', 8, 0, compositor);
    write_char(b'l', 12, 0, compositor);
    write_char(b'e', 20, 0, compositor);
    write_char(b'r', 24, 0, compositor);
    write_char(b'r', 28, 0, compositor);
}

pub fn display_catastrophe_screen(_frames: &ThreeVotolFrames, compositor: &mut Compositor) {
    write_char(b'd', 0, 0, compositor);
    write_char(b'i', 4, 0, compositor);
    write_char(b's', 8, 0, compositor);
    write_char(b'p', 12, 0, compositor);
    write_char(b'e', 20, 0, compositor);
    write_char(b'r', 24, 0, compositor);
    write_char(b'r', 28, 0, compositor);
}
