#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_halt as _;
use panic_halt as _;
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    let ir = gpiob.pb10.into_floating_input(&mut gpiob.crh);

    let mut led_r = gpioc.pc7.into_push_pull_output(&mut gpioc.crl);
    let mut led_g = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut led_b = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);

    let mut stage = 0;

    loop {
        if ir.is_low() {
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
        }
    }
}
