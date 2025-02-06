#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::can::frame::Envelope;
use embassy_stm32::can::{
    filter, Can, Fifo, Frame, Id, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, StandardId,
    TxInterruptHandler, CanTx,
};
use embassy_stm32::peripherals::CAN;
use embassy_stm32::{bind_interrupts, Config};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::gpio::{Input, Pull};

use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    USB_LP_CAN1_RX0 => Rx0InterruptHandler<CAN>;
    CAN1_RX1 => Rx1InterruptHandler<CAN>;
    CAN1_SCE => SceInterruptHandler<CAN>;
    USB_HP_CAN1_TX => TxInterruptHandler<CAN>;
});

// This example is configured to work with real CAN transceivers on B8/B9.
// See other examples for loopback.

fn handle_frame(env: Envelope, read_mode: &str) {
    match env.frame.id() {
        Id::Extended(id) => {
            /*defmt::println!(
                "{} Extended Frame id={:x} {:02x}",
                read_mode,
                id.as_raw(),
                env.frame.data()
            );*/
        }
        Id::Standard(id) => {
            if (*id == StandardId::new(1022).unwrap()) {
                defmt::println!(
                    "{} Standard Frame id={:x} {:02x}",
                    read_mode,
                    id.as_raw(),
                    env.frame.data()
                );
            }
        }
    }
}

#[embassy_executor::task]
async fn send_votol_msg(mut tx: CanTx<'static>) {
    let votol_msg: [u8; 24] = [0xc9, 0x14, 0x02, 0x53, 0x48, 0x4f, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0x00, 0x00, 0x00, 0x18, 0xaa, 0x00, 0x00, 0x00, 0x00, 0xc4, 0x0d];

    // from ES https://endless-sphere.com/sphere/threads/votol-em100-canbus-protocols.114159/
    // id = 1023
    let votol_can_msg1: [u8; 8] = [9, 85, 170, 170, 0, 170, 0, 0];
    let votol_can_msg2: [u8; 8] = [0, 24, 170, 5, 210, 0, 32, 51];

    // from my captures
    //let votol_can_msg2: [u8; 8] = [0, 24, 170, 5, 220, 0, 26, 7];

    loop {
        let tx_frame = Frame::new_data(unwrap!(StandardId::new(1023)), &votol_can_msg1).unwrap();
        info!("writing votol message1");
        tx.write(&tx_frame).await;

        let tx_frame2 = Frame::new_data(unwrap!(StandardId::new(1023)), &votol_can_msg2).unwrap();
        info!("writing votol message2");
        tx.write(&tx_frame2).await;

        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut p = embassy_stm32::init(Config::default());

    // Set alternate pin mapping to B8/B9
    //embassy_stm32::pac::AFIO.mapr().modify(|w| w.set_can1_remap(2));

    static RX_BUF: StaticCell<embassy_stm32::can::RxBuf<10>> = StaticCell::new();
    static TX_BUF: StaticCell<embassy_stm32::can::TxBuf<10>> = StaticCell::new();

    // The next two lines are a workaround for testing without transceiver.
    // To synchronise to the bus the RX input needs to see a high level.
    // Use `mem::forget()` to release the borrow on the pin but keep the
    // pull-up resistor enabled.
    //let rx_pin = Input::new(&mut p.PA11, Pull::Up);
    //core::mem::forget(rx_pin);

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
    let mut i: u8 = 0;

    let (mut tx, mut rx) = can.split();

    spawner.spawn(send_votol_msg(tx)).unwrap();

    // This example shows using the wait_not_empty API before try read
    loop {
        //info!("waiting for not empty");
        rx.wait_not_empty().await;

        let env = rx.try_read().unwrap();
        //info!("read succesful");
        handle_frame(env, "Wait");
    }
}
