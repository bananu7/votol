type ThreeVotolFrames = [[u8; 8]; 3];

pub fn get_battery_voltage(frames: &ThreeVotolFrames) -> u16 {
    ((frames[0][7] as u16) << 8u16) + (frames[1][0] as u16)
}


pub fn get_controller_temp(frames: &ThreeVotolFrames) -> u8 {
    frames[2][2] - 50 // 50C offset
}