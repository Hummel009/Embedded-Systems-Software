#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::serial::{Config, Serial};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain();
    let channels = p.DMA1.split();

    let mut gpioa = p.GPIOA.split();

    let rx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let tx = gpioa.pa3;

    let serial = Serial::new(
        p.USART2,
        (rx, tx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &clocks,
    );

    let mut tx = serial.tx.with_dma(channels.7);

    loop {
        let (_, loop_tx) = tx.write(b"0").wait();
        tx = loop_tx;
        delay(2_000_000);

        let (_, loop_tx) = tx.write(b"1").wait();
        tx = loop_tx;
        delay(2_000_000);

        let (_, loop_tx) = tx.write(b"0").wait();
        tx = loop_tx;
        delay(2_000_000);
    }
}
