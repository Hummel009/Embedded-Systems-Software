#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m::asm::delay;
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

    let mut ds = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let mut sh_cp = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let mut st_cp = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);

    let mut stage = 1;
    let mut is_displayed = false;

    loop {
        if stage == 1 {
            led1.set_low();
            led2.set_low();
            led3.set_low();
            led4.set_high();

            if !is_displayed {
                is_displayed = true;

                for number in 0..10 {
                    delay(1 * 4_000_000);
                    display_number(&mut ds, &mut sh_cp, &mut st_cp, number);
                }
            }

            if btn1.is_low() {
                stage = 2;

                led4.set_low();
            }
        } else if stage == 2 {
        } else if stage == 3 {
        } else if stage == 4 {
        }
    }
}

fn display_number(
    ds: &mut Pin<'A', 9, Output>,
    sh_cp: &mut Pin<'A', 8, Output>,
    st_cp: &mut Pin<'B', 5, Output>,
    number: u8,
) {
    let segments: [[bool; 7]; 10] = [
        // централ, верх-лево, низ-лево, низ,    низ-право, верх-право, верх
        [false,     true,      true,     true,   true,      true,       true],      // 0
        [false,     false,     false,    false,  true,      true,       false],     // 1
        [true,      false,     true,     true,   false,     true,       true],      // 2
        [true,      false,     false,    true,   true,      true,       true],      // 3
        [true,      true,      false,    false,  true,      true,       false],     // 4
        [true,      true,      false,    true,   true,      false,      true],      // 5
        [true,      true,      true,     true,   true,      false,      true],      // 6
        [false,     false,     false,    false,  true,      true,       true],      // 7
        [true,      true,      true,     true,   true,      true,       true],      // 8
        [true,      true,      false,    true,   true,      true,       true],      // 9
    ];

    let selected = segments[number as usize];

    if number > 9 {
        return;
    }

    // НАЧАЛО
    st_cp.set_low();
    sh_cp.set_low();

    // Зачистка
    for _ in 0..2 {
        ds.set_high();
        sh_cp.set_high();
        sh_cp.set_low();
    }

    // Какие палочки из числа горят
    for pos in 0..7 {
        if selected[pos] { 
            ds.set_low(); 
        } else { 
            ds.set_high(); 
        }
        sh_cp.set_high();
        sh_cp.set_low();
    }

    // Зачистка
    for _ in 0..4 {
        ds.set_high();
        sh_cp.set_high();
        sh_cp.set_low();
    }

    // Какое число на табло горит

    let states = [true, false, false, false];

    for &state in &states {
        if state {
            ds.set_high();
        } else {
            ds.set_low();
        }
        sh_cp.set_high();
        sh_cp.set_low();
    }

    // КОНЕЦ
    st_cp.set_high();
    st_cp.set_low();
}
