#![no_std]
#![no_main]

use can::can_frame::{get_controller_state, ControllerState};
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
use crate::ledmatrix::compositor::{Compositor, write_out};

pub mod can;
use crate::can::can_communication::{send_votol_msg, handle_frame, create_fake_votol_response};
use crate::ledmatrix::screens::{ride_screen, fault_screen, display_catastrophe_screen, ControllerValue, next};

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

    let mut central_value = ControllerValue::Voltage;
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

        compositor.clear();

        // handle value change
        if button_b.is_low() {
            if !pressed {
                central_value = next(central_value);
                pressed = true;
            }
        } else {
            pressed = false;
        }

        match get_controller_state(&frames) {
            Some(ControllerState::FAULT) => fault_screen(&frames, &mut compositor),
            Some(_) => ride_screen(&frames, central_value, &mut compositor),
            None => display_catastrophe_screen(&frames, &mut compositor),
        }

        write_out(&compositor, &mut display);
    }
}
