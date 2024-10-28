#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::serial::{Config, Serial};

extern crate libm;

static mut VALUE_BITS: [u8; 32] = [48; 32];

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

    let mut i = 0;
    loop {
        i += 1;

        let value = generate_sine_wave(10, 5, i, 44100, 0);

        let bits: u32 = value.to_bits();

        unsafe {
            for i in 0..32 {
                VALUE_BITS[i] = if (bits >> (31 - i)) & 1 == 1 { 49 } else { 48 };
            }

            let (_, loop_tx) = tx.write(&VALUE_BITS).wait();
            tx = loop_tx;
        }

        delay(2_000_000);
    }
}

fn generate_sine_wave(a: i32, f: i32, i: i32, n: i32, phi0: i32) -> f32 {
    return a as f32 * libm::sin(2.0 * 3.14 * f as f64 * i as f64 / n as f64 + phi0 as f64) as f32;
}