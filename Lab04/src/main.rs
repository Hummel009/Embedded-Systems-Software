#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use panic_halt as _;
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::serial::{Config, Serial};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiod = dp.GPIOD.split();

    let mut afio = dp.AFIO.constrain();
    let channels = dp.DMA1.split();

    //pd5?
    let rx = gpiod.pd5.into_alternate_push_pull(&mut gpiod.crl);
    let tx = gpiod.pd6;

    //9600?
    let serial = Serial::new(
        dp.USART2,
        (rx, tx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &clocks,
    );

    let mut tx = serial.tx.with_dma(channels.7);

    let mut led_r = gpiod.pd9.into_push_pull_output(&mut gpiod.crh);
    let mut led_g = gpiod.pd10.into_push_pull_output(&mut gpiod.crh);
    let mut led_b = gpiod.pd11.into_push_pull_output(&mut gpiod.crh);

    let mut stage = 0;

    loop {
        unsafe {
            //откуда читать?
            //let received = rx.read().unwrap();
           // tx = loop_tx;
        }

        /*if rx.is_low() {
            if stage == 0 {
                stage = 1;
            } else if stage == 1 {
                stage = 2;
            } else if stage == 2 {
                stage = 3;
            } else if stage == 3 {
                stage = 1;
            }

            delay(2_000_000);
        }

        if stage == 1 {
            led_r.set_high();
            led_g.set_low();
            led_b.set_low();
        } else if stage == 2 {
            led_r.set_low();
            led_g.set_high();
            led_b.set_low();
        } else if stage == 3 {
            led_r.set_low();
            led_g.set_low();
            led_b.set_high();
        }*/
    }
}
