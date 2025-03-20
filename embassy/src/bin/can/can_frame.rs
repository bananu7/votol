pub type ThreeVotolFrames = [[u8; 8]; 3];
type FixedPointOneTenth = u16;

pub enum ControllerState {
    IDLE, // 0
    INIT, // 1
    START, // 2
    RUN, // 3
    STOP, // 4
    BRAKE, // 5
    WAIT, // 6
    FAULT, // 7
}

pub fn get_battery_voltage(frames: &ThreeVotolFrames) -> FixedPointOneTenth {
    ((frames[0][7] as u16) << 8u16) + (frames[1][0] as u16)
}

pub fn get_battery_current(frames: &ThreeVotolFrames) -> FixedPointOneTenth {
    ((frames[0][7] as u16) << 8u16) + (frames[1][0] as u16)
}

pub fn get_rpm(frames: &ThreeVotolFrames) -> i16 {
    ((frames[2][0] as i16) << 8u16) + (frames[2][1] as i16)
}

// The two temperature values have +50C offset; this means
// that e.g. temperature of 80C is stored as 130, and temperature
// of -10C is stored as 40.
pub fn get_controller_temp(frames: &ThreeVotolFrames) -> i16 {
    (frames[2][2] - 50).into()
}

pub fn get_external_temp(frames: &ThreeVotolFrames) -> i16 {
    (frames[2][3] - 50).into()
}

pub fn clamp_temp_to_0(temp: i16) -> u8 {
    if temp < 0 {
        0
    } else {
        temp as u8
    }
}

pub fn get_controller_state(frames: &ThreeVotolFrames) -> Option<ControllerState> {
    return match frames[2][7] {
        0 => Some(ControllerState::IDLE),
        1 => Some(ControllerState::INIT),
        2 => Some(ControllerState::START),
        3 => Some(ControllerState::RUN),
        4 => Some(ControllerState::STOP),
        5 => Some(ControllerState::BRAKE),
        6 => Some(ControllerState::WAIT),
        7 => Some(ControllerState::FAULT),
        _ => None,
    }
}
