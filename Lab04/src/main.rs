//! Testing PWM output for pre-defined pin combination: all pins for default mapping

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m::asm::delay;
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    timer::{Channel, Tim3NoRemap, Tim4NoRemap, Timer},
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
    let mut gpioc = dp.GPIOC.split();
    
    let ir = gpiob.pb10.into_floating_input(&mut gpiob.crh);

    let mut led_r = gpioc.pc7.into_push_pull_output(&mut gpioc.crl);

    let g1 = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    let g2 = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let b1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let b2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);

    let pins_g = (g1, g2);
    let pins_b = (b1, b2);

    let mut pwm_g = Timer::new(dp.TIM3, &clocks).pwm_hz::<Tim3NoRemap, _, _>(pins_g, &mut afio.mapr, 1.kHz());
    let mut pwm_b = Timer::new(dp.TIM4, &clocks).pwm_hz::<Tim4NoRemap, _, _>(pins_b, &mut afio.mapr, 1.kHz());

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
            pwm_g.disable(Channel::C2);
            pwm_b.disable(Channel::C1);

            led_r.set_high();
        } else if stage == 2 {
            pwm_b.disable(Channel::C1);
            led_r.set_low();

            pwm_g.enable(Channel::C2);
            pwm_g.set_duty(Channel::C2, pwm_g.get_max_duty() / 10);
        } else if stage == 3 {
            pwm_g.disable(Channel::C2);
            led_r.set_low();
            
            pwm_b.enable(Channel::C1);
            pwm_b.set_duty(Channel::C1, pwm_b.get_max_duty() / 30);
        }
    }
}