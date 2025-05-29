use crate::ledmatrix::api::{write_fullscreen_float, write_battery_bar, write_num, write_num_4_digits, write_char, write_string};
use crate::ledmatrix::compositor::Compositor;
use crate::can::can_frame::{clamp_temp_to_0, get_battery_current, get_battery_voltage, get_controller_temp, get_controller_error, get_external_temp, get_rpm, ThreeVotolFrames, ControllerError};

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

pub fn ride_screen(frames: &ThreeVotolFrames, value_to_show: DisplayValue, compositor: &mut Compositor, _time_ms: u32) {
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

/// Converts a ControllerError to a descriptive string
pub fn error_to_string(error: &ControllerError) -> &'static str {
    match error {
        ControllerError::EBrakeOn => "Emergency Brake On",
        ControllerError::OverCurrent => "Hardware Overcurrent",
        ControllerError::UnderVoltage => "Low Battery Voltage",
        ControllerError::ThrottleHallError => "Throttle Hall Error",
        ControllerError::OverVoltage => "High Battery Voltage",
        ControllerError::McuError => "Mcu Error",
        ControllerError::MotorBlock => "Motor Blocked",
        ControllerError::FootplateErr => "Throttle Error",
        ControllerError::SpeedControl => "Runaway Error",
        ControllerError::WritingEeprom => "Program Upload Error",
        ControllerError::StartUpFailure => "Startup Failure",
        ControllerError::Overheat => "Controller Overheat",
        ControllerError::OverCurrent1 => "Software Overcurrent",
        ControllerError::AcceleratePadalErr => "Throttle Failure",
        ControllerError::Ics1Err => "Current Sensor 1 Error",
        ControllerError::Ics2Err => "Current Sensor 2 Error",
        ControllerError::BreakErr => "Brake Failure",
        ControllerError::HallSelError => "Hall Sensor Error",
        ControllerError::MosfetDriverFault => "Driver Failure",
        ControllerError::MosfetHighShort => "Mosfet High Short",
        ControllerError::PhaseOpen => "Phase Wire Open",
        ControllerError::PhaseShort => "Phase Wire Short",
        ControllerError::McuChipError => "Controller Failure",
        ControllerError::PreChargeError => "Pre-charge Failure",
        ControllerError::MotorOverheat => "Motor Overheat",
        ControllerError::SocZeroError => "SOC Zero Error",
    }
}

/// Helper function to write a scrolling string on the display
/// If the string is longer than the display width, it will scroll
/// If not, it will be displayed statically
pub fn write_string_scrolling(message: &str, x: usize, y: usize, time_ms: u32, display_width: usize, compositor: &mut Compositor) {
    // Only scroll if the message is longer than what fits on screen
    if message.len() > display_width {
        // Change scroll position every 500ms
        let time_step = 500;
        let scroll_position = ((time_ms / time_step) as usize) % (message.len() + 2);
        let offset_position = ((time_ms / (time_step / 4)) as usize) % 4;

        // Handle the case where we're at the end of the message and need to wrap
        if scroll_position < message.len() {
            write_string(&message, x, y, scroll_position, display_width, compositor);
            for _ in 0..offset_position {
                compositor.shift_left();
            }
        } else {
            // Just show the beginning when we're in the wrap-around transition
            write_string(message, x, y, 0, display_width, compositor);
        }
    } else {
        // Message fits, no need to scroll
        write_string(message, x, y, 0, display_width, compositor);
    }
}

pub fn fault_screen(frames: &ThreeVotolFrames, compositor: &mut Compositor, time_ms: u32) {
    if let Some(error) = get_controller_error(frames) {
        let error_message = error_to_string(&error);
        write_string_scrolling(error_message, 0, 0, time_ms, 8, compositor);
    } else {
        // This is a weird case as we are in error state but the error field is empty.
        write_string("Error", 0, 0, 0, 8, compositor);
    }
}

pub fn display_catastrophe_screen(_frames: &ThreeVotolFrames, compositor: &mut Compositor, time_ms: u32) {
    let message = "display error";
    write_string_scrolling(message, 0, 0, time_ms, 8, compositor);
}
