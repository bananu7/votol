pub type ThreeVotolFrames = [[u8; 8]; 3];
type FixedPointOneTenth = i16;

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

impl Into<u8> for ControllerState {
    fn into(self) -> u8 {
        match self {
            ControllerState::IDLE => 0,
            ControllerState::INIT => 1,
            ControllerState::START => 2,
            ControllerState::RUN => 3,
            ControllerState::STOP => 4,
            ControllerState::BRAKE => 5,
            ControllerState::WAIT => 6,
            ControllerState::FAULT => 7,
        }
    }
}

impl TryFrom<u8> for ControllerState {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ControllerState::IDLE),
            1 => Ok(ControllerState::INIT),
            2 => Ok(ControllerState::START),
            3 => Ok(ControllerState::RUN),
            4 => Ok(ControllerState::STOP),
            5 => Ok(ControllerState::BRAKE),
            6 => Ok(ControllerState::WAIT),
            7 => Ok(ControllerState::FAULT),
            _ => Err(()),
        }
    }
}

pub fn combine_two_bytes_into_i16(a: u8, b: u8) -> i16 {
    ((a as i16) << 8u16) + (b as i16)
}

pub fn get_battery_voltage(frames: &ThreeVotolFrames) -> FixedPointOneTenth {
    combine_two_bytes_into_i16(frames[0][7], frames[1][0])
}

pub fn get_battery_current(frames: &ThreeVotolFrames) -> FixedPointOneTenth {
    combine_two_bytes_into_i16(frames[1][1], frames[1][2])
}

pub fn get_rpm(frames: &ThreeVotolFrames) -> i16 {
    ((frames[2][0] as i16) << 8u16) + (frames[2][1] as i16)
}

// The two temperature values have +50C offset; this means
// that e.g. temperature of 80C is stored as 130, and temperature
// of -10C is stored as 40.
pub fn get_controller_temp(frames: &ThreeVotolFrames) -> i16 {
    (frames[2][2] as i16) - 50
}

pub fn get_external_temp(frames: &ThreeVotolFrames) -> i16 {
    (frames[2][3] as i16) - 50
}

pub fn clamp_temp_to_0(temp: i16) -> u8 {
    if temp < 0 {
        0
    } else {
        temp as u8
    }
}

pub fn get_controller_state(frames: &ThreeVotolFrames) -> Option<ControllerState> {
    return match ControllerState::try_from(frames[2][7]) {
        Ok(state) => Some(state),
        Err(_) => None,
    }
}

pub enum ControllerError {
    EBrakeOn,               // 0x001 Brake
    OverCurrent,            // 0x02 Hardware overcurrent
    UnderVoltage,           // 0x04 Under voltage
    ThrottleHallError,      // 0x08 Throttle Hall error (this is often labeled just "Hall Error")
    OverVoltage,            // 0x10 Over voltage
    McuError,               // 0x20 Controller error
    MotorBlock,             // 0x40 Motor block error
    FootplateErr,           // 0x80 Throttle error
    SpeedControl,           // 0x100 Run away
    WritingEeprom,          // 0x200 EEROM writing
    StartUpFailure,         // 0x800 Quality inspection failure
    Overheat,               // 0x1000 Controller overheat
    OverCurrent1,           // 0x2000 Software overcurrent
    AcceleratePadalErr,     // 0x4000 Throttle failure
    Ics1Err,                // 0x8000 Current sensor error 1
    Ics2Err,                // 0x10000 Current sensor error 2
    BreakErr,               // 0x20000 Brake failure
    HallSelError,           // 0x40000 Hall error
    MosfetDriverFault,      // 0x80000 Driver failure
    MosfetHighShort,        // 0x100000 MOS tube short circuit
    PhaseOpen,              // 0x200000 Phase wire connection failure
    PhaseShort,             // 0x400000 phase wire short circuit
    McuChipError,           // 0x800000 Controller failure
    PreChargeError,         // 0x1000000 Pre-charge failure
    MotorOverheat,          // 0x8000000 Motor overheat
    SocZeroError            // 0x80000000 SOC 0 error
}

pub fn get_controller_error(frames: &ThreeVotolFrames) -> Option<ControllerError> {
    // Error bits are stored in 4 bytes at indices 10-13 in the overall message
    // Frame 1, bytes 2-5 correspond to these indices
    let error_byte1 = frames[1][4];
    let error_byte2 = frames[1][5];
    let error_byte3 = frames[1][6];
    let error_byte4 = frames[1][7];

    // Combine the 4 bytes into a 32-bit error code
    let error_code: u32 = (error_byte1 as u32) << 24 |
                          (error_byte2 as u32) << 16 |
                          (error_byte3 as u32) << 8 |
                          (error_byte4 as u32);

    // No errors if error_code is 0
    if error_code == 0 {
        return None;
    }

    // Return the first error code basing on lowest-error first.
    if (error_code & 0x001) != 0 { return Some(ControllerError::EBrakeOn); }
    if (error_code & 0x002) != 0 { return Some(ControllerError::OverCurrent); }
    if (error_code & 0x004) != 0 { return Some(ControllerError::UnderVoltage); }
    if (error_code & 0x008) != 0 { return Some(ControllerError::ThrottleHallError); }
    if (error_code & 0x010) != 0 { return Some(ControllerError::OverVoltage); }
    if (error_code & 0x020) != 0 { return Some(ControllerError::McuError); }
    if (error_code & 0x040) != 0 { return Some(ControllerError::MotorBlock); }
    if (error_code & 0x080) != 0 { return Some(ControllerError::FootplateErr); }
    if (error_code & 0x100) != 0 { return Some(ControllerError::SpeedControl); }
    if (error_code & 0x200) != 0 { return Some(ControllerError::WritingEeprom); }
    if (error_code & 0x800) != 0 { return Some(ControllerError::StartUpFailure); }
    if (error_code & 0x1000) != 0 { return Some(ControllerError::Overheat); }
    if (error_code & 0x2000) != 0 { return Some(ControllerError::OverCurrent1); }
    if (error_code & 0x4000) != 0 { return Some(ControllerError::AcceleratePadalErr); }
    if (error_code & 0x8000) != 0 { return Some(ControllerError::Ics1Err); }
    if (error_code & 0x10000) != 0 { return Some(ControllerError::Ics2Err); }
    if (error_code & 0x20000) != 0 { return Some(ControllerError::BreakErr); }
    if (error_code & 0x40000) != 0 { return Some(ControllerError::HallSelError); }
    if (error_code & 0x80000) != 0 { return Some(ControllerError::MosfetDriverFault); }
    if (error_code & 0x100000) != 0 { return Some(ControllerError::MosfetHighShort); }
    if (error_code & 0x200000) != 0 { return Some(ControllerError::PhaseOpen); }
    if (error_code & 0x400000) != 0 { return Some(ControllerError::PhaseShort); }
    if (error_code & 0x800000) != 0 { return Some(ControllerError::McuChipError); }
    if (error_code & 0x1000000) != 0 { return Some(ControllerError::PreChargeError); }
    if (error_code & 0x8000000) != 0 { return Some(ControllerError::MotorOverheat); }
    if (error_code & 0x80000000) != 0 { return Some(ControllerError::SocZeroError); }

    None
}
