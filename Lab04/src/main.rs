//! Testing PWM output for pre-defined pin combination: all pins for default mapping

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    time::ms,
    timer::{Channel, Tim3NoRemap},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    let c1 = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let c3 = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    let c4 = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);

    let pins = (c1, c2, c3, c4);

    let mut pwm = dp.TIM3.pwm_hz::<Tim3NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz(), &clocks);

    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);
    pwm.enable(Channel::C3);

    pwm.set_period(ms(500).into_rate());

    let max = pwm.get_max_duty();

    pwm.set_duty(Channel::C2, max / 4);

    loop {}
}