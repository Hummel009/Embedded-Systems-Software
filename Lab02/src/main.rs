#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    let mut led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut led2 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let mut led3 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut led4 = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);

    let btn1 = gpioa.pa1.into_pull_up_input(&mut gpioa.crl);
    let btn2 = gpioa.pa4.into_pull_up_input(&mut gpioa.crl);
    let btn3 = gpiob.pb0.into_pull_up_input(&mut gpiob.crl);

    led1.set_low();
    led2.set_low();
    led3.set_low();
    led4.set_low();

    let mut stage = 1;
    
    loop {
        if stage == 1 {
            if btn1.is_low() {
                led4.set_high();
            }
        } else if stage == 2 {

        } else if stage == 3 {

        } else if stage == 4 {

        }
    }
}