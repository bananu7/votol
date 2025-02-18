#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::can::frame::{Envelope, Timestamp};
use embassy_stm32::can::{
    filter, Can, Fifo, Frame, Id, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, StandardId,
    TxInterruptHandler, CanTx,
};
use embassy_stm32::peripherals::CAN;
use embassy_stm32::{bind_interrupts, Config};
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::gpio::{Speed, Level, Output};

use embassy_time::Timer;

pub mod ledmatrix;
use crate::ledmatrix::setup::setup_display;
use crate::ledmatrix::api::{write_fullscreen_float, write_battery_bar, write_num};
use crate::ledmatrix::compositor::{Compositor, write_out};

bind_interrupts!(struct Irqs {
    USB_LP_CAN1_RX0 => Rx0InterruptHandler<CAN>;
    CAN1_RX1 => Rx1InterruptHandler<CAN>;
    CAN1_SCE => SceInterruptHandler<CAN>;
    USB_HP_CAN1_TX => TxInterruptHandler<CAN>;
});

async fn handle_frame(env: Envelope, read_mode: &str, counter: &mut usize, frames: &mut [[u8; 8]; 3]) {
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


#[embassy_executor::task]
async fn send_votol_msg(mut tx: CanTx<'static>) {
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

fn create_fake_votol_response(id: usize) -> Envelope {
    let votol_can_responses: [[u8; 8]; 3] = [
        [0x09, 0x55, 0xaa, 0xaa, 0x00, 0x00, 0x00, 0x01],
        [0x27, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x84],
        [0x00, 0x00, 0x4a, 0xf0, 0x00, 0x00, 0x01, 0x07]
    ];

    return Envelope {
        ts: Timestamp::now(),
        frame: Frame::new_standard(1022, &votol_can_responses[id]).unwrap(),
    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    // DISPLAY -----------------------
    let cs = Output::new(p.PB12, Level::High, Speed::VeryHigh);
    let sck = Output::new(p.PB13, Level::High, Speed::VeryHigh);
    let data = Output::new(p.PB15, Level::High, Speed::VeryHigh);
    let mut display = setup_display(cs, sck, data);
    // END DISPLAY -------------------


    // CAN -----------------------
    // Set alternate pin mapping to B8/B9
    //embassy_stm32::pac::AFIO.mapr().modify(|w| w.set_can1_remap(2));
    //let mut can = Can::new(p.CAN, p.PB8, p.PB9, Irqs);

    let mut can = Can::new(p.CAN, p.PA11, p.PA12, Irqs);

    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, filter::Mask32::accept_all());

    can.modify_config()
        .set_loopback(false)
        .set_silent(false)
        .set_bitrate(250_000);

    info!("enabling can");
    can.enable().await;

    let (tx, mut rx) = can.split();
    // END CAN -----------------------

    // VOTOL --------------------------------------------
    let mut frames: [[u8; 8]; 3] = [
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0]
    ];
    let mut frame_counter: usize = 0;
    let mut compositor = Compositor::new();

    spawner.spawn(send_votol_msg(tx)).unwrap();
    // END VOTOL --------------------------------------------

    // This example shows using the wait_not_empty API before try read
    loop {
        let env = if false {
            //info!("waiting for not empty");
            rx.wait_not_empty().await;
            rx.try_read().unwrap()
        } else {
            create_fake_votol_response(frame_counter)
        };

        handle_frame(env, "Wait", &mut frame_counter, &mut frames).await;

        let battery_voltage: u16 = ((frames[0][7] as u16) << 8u16) + (frames[1][0] as u16);
        compositor.clear();
        write_fullscreen_float(battery_voltage, &mut compositor);
        write_battery_bar(battery_voltage, &mut compositor);

        let controller_temp = frames[2][2] - 50; // 50C offset
        write_num(controller_temp, 14, 0, &mut compositor);

        write_out(&compositor, &mut display);
    }
}
