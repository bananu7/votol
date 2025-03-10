#![no_std]
#![no_main]

use core::ops::Not;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::can::{
    filter, Can, Fifo,  Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
    TxInterruptHandler
};
use embassy_stm32::peripherals::CAN;
use embassy_stm32::{bind_interrupts, Config};
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::gpio::{Speed, Level, Output, Input, Pull};

pub mod ledmatrix;
use crate::ledmatrix::setup::setup_display;
use crate::ledmatrix::api::{write_fullscreen_float, write_battery_bar, write_num, write_char};
use crate::ledmatrix::compositor::{Compositor, write_out};

pub mod can;
use crate::can::can_frame::{get_battery_voltage, get_controller_temp, get_external_temp, clamp_temp_to_0};
use crate::can::can_communication::{send_votol_msg, handle_frame, create_fake_votol_response};

bind_interrupts!(struct Irqs {
    USB_LP_CAN1_RX0 => Rx0InterruptHandler<CAN>;
    CAN1_RX1 => Rx1InterruptHandler<CAN>;
    CAN1_SCE => SceInterruptHandler<CAN>;
    USB_HP_CAN1_TX => TxInterruptHandler<CAN>;
});

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

    // BUTTONS
    let button_a = Input::new(p.PB10, Pull::Up);
    let button_b = Input::new(p.PB11, Pull::Up);
    // END BUTTONS

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

    let mut c: u8 = b'a';
    let mut pressed = false;

    // This example shows using the wait_not_empty API before try read
    loop {
        let env = if false {
            rx.wait_not_empty().await;
            rx.try_read().unwrap()
        } else {
            create_fake_votol_response(frame_counter)
        };

        handle_frame(env, "Wait", &mut frame_counter, &mut frames).await;

        let battery_voltage = get_battery_voltage(&frames);
        compositor.clear();
        write_fullscreen_float(battery_voltage, &mut compositor);
        write_battery_bar(battery_voltage, &mut compositor);

        let controller_temp = clamp_temp_to_0(get_controller_temp(&frames));
        let external_temp = clamp_temp_to_0(get_external_temp(&frames));
        if button_a.is_high() {
            write_num(controller_temp, 14, 0, &mut compositor);
        } else {
            write_num(external_temp, 14, 0, &mut compositor);
        }

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
        write_char(c, 24, 0, &mut compositor);

        write_out(&compositor, &mut display);
    }
}
