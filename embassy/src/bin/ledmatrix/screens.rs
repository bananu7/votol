 use crate::ledmatrix::api::{write_fullscreen_float, write_battery_bar, write_num, write_char};
 use crate::ledmatrix::compositor::Compositor;
 use crate::can::can_frame::{get_battery_voltage, get_controller_temp, get_external_temp, clamp_temp_to_0, ThreeVotolFrames};

 pub fn ride_screen(frames: &ThreeVotolFrames, compositor: &mut Compositor) {
    let battery_voltage = get_battery_voltage(&frames);
    write_battery_bar(battery_voltage, compositor);
    write_fullscreen_float(battery_voltage, compositor);

    let controller_temp = clamp_temp_to_0(get_controller_temp(&frames));
    let external_temp = clamp_temp_to_0(get_external_temp(&frames));

    // todo handle input
    //if button_a.is_high() {
        write_num(controller_temp, 14, 0, compositor);
        //} else {
        //write_num(external_temp, 14, 0, &mut display);
        //}

    /*/
    if button_b.is_low() {
        if !pressed {
            c += 1;
            if c > b'z' {
                c = b'a';
            }
            pressed = true;
        }
    } else {
        pressed = false;
    }
    */

    //write_char(c, 24, 0, &mut display);
}

pub fn fault_screen(_frames: &ThreeVotolFrames, compositor: &mut Compositor) {
    write_char(b'e', 0, 0, compositor);
    write_char(b'r', 4, 0, compositor);
    write_char(b'r', 8, 0, compositor);
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
