use defmt::*;
use embassy_time::Timer;
use embassy_stm32::can::{
    Frame, Id, StandardId, CanTx,
};
use embassy_stm32::can::frame::{
    Envelope, Timestamp
};

#[embassy_executor::task]
pub async fn send_votol_msg(mut tx: CanTx<'static>) {
    // from ES https://endless-sphere.com/sphere/threads/votol-em100-canbus-protocols.114159/
    let votol_can_msg1: [u8; 8] = [9, 85, 170, 170, 0, 170, 0, 0];
    let votol_can_msg2: [u8; 8] = [0, 24, 170, 5, 210, 0, 32, 51];
    let id = unwrap!(StandardId::new(1023));
    let tx_frame = Frame::new_data(id, &votol_can_msg1).unwrap();
    let tx_frame2 = Frame::new_data(id, &votol_can_msg2).unwrap();

    loop {
        info!("writing votol message1");
        tx.write(&tx_frame).await;

        info!("writing votol message2");
        tx.write(&tx_frame2).await;

        Timer::after_millis(300).await;
    }
}

pub async fn handle_frame(env: Envelope, read_mode: &str, counter: &mut usize, frames: &mut [[u8; 8]; 3]) {
    match env.frame.id() {
        Id::Extended(_id) => {
            /*defmt::println!(
                "{} Extended Frame id={:x} {:02x}",
                read_mode,
                id.as_raw(),
                env.frame.data()
            );*/
        }
        Id::Standard(id) => {
            if *id == StandardId::new(1022).unwrap() {
                defmt::println!(
                    "{} Standard Frame id={:x} {:02x}",
                    read_mode,
                    id.as_raw(),
                    env.frame.data()
                );

                for i in 0..8 {
                    frames[*counter][i] = env.frame.data()[i];
                }
                defmt::println!("{}", *frames);

                *counter += 1;
                if *counter == 3 {
                    *counter = 0;
                }
            }
        }
    }
}

pub fn create_fake_votol_response(
    id: usize,
    battery_voltage: u16,
    controller_temp: i8,
    motor_temp: i8,
    rpm: i16
) -> Envelope {
    let bv_h: u8 = (battery_voltage >> 8) as u8;
    let bv_l: u8 = (battery_voltage & 0xFF) as u8;

    let ct: u8 = (controller_temp + 50) as u8;
    let et: u8 = (motor_temp + 50) as u8;

    let rpm_h: u8 = (rpm >> 8) as u8;
    let rpm_l: u8 = (rpm & 0xFF) as u8;

    let votol_can_responses: [[u8; 8]; 3] = [
        [0x09,  0x55,  0xaa, 0xaa, 0x00, 0x00, 0x00, bv_h],
        [bv_l,  0x00,  0x01, 0x00, 0x00, 0x00, 0x00, 0x84],
        [rpm_h, rpm_l,   ct,   et, 0x00, 0x00, 0x01, 0x07]
    ];

    return Envelope {
        ts: Timestamp::now(),
        frame: Frame::new_standard(1022, &votol_can_responses[id]).unwrap(),
    }
}
