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
    let mut best_level = 0;
    let mut current_level = 1;

    led1.set_high();
    led2.set_high();
    led3.set_high();
    led4.set_high();

    let levels = [
        // Уровень 1
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        // Уровень 2
        [2, 3, 0, 0, 0, 0, 0, 0, 0],
        // Уровень 3
        [1, 4, 2, 0, 0, 0, 0, 0, 0],
        // Уровень 4
        [3, 1, 2, 4, 0, 0, 0, 0, 0],
        // Уровень 5
        [2, 4, 1, 3, 2, 0, 0, 0, 0],
        // Уровень 6
        [1, 3, 4, 2, 1, 3, 0, 0, 0],
        // Уровень 7
        [4, 1, 2, 3, 4, 2, 1, 0, 0],
        // Уровень 8
        [2, 3, 1, 4, 2, 1, 4, 3, 0],
        // Уровень 9
        [1, 4, 2, 3, 1, 2, 3, 4, 2],
    ];

    loop {
        delay(500_000);

        if stage == 1 {
            led1.set_high();
            led2.set_high();
            led3.set_high();
            led4.set_low();

            display_number(&mut ds, &mut sh_cp, &mut st_cp, best_level);

            if btn1.is_low() {
                stage = 2;
            }
        } else if stage == 2 {
            led1.set_high();
            led2.set_high();
            led3.set_high();
            led4.set_high();
            
            display_number(&mut ds, &mut sh_cp, &mut st_cp, current_level);

            let sequence = levels[current_level as usize];

            light_on(sequence, &mut led1, &mut led2, &mut led3, &mut led4);

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
    let segments = [
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

fn light_on(
    sequence: [i32; 9],
    led1: &mut Pin<'A', 5, Output>,
    led2: &mut Pin<'A', 6, Output>,
    led3: &mut Pin<'A', 7, Output>,
    led4: &mut Pin<'B', 6, Output>

){

}