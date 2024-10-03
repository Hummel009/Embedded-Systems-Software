#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::pac::interrupt;
use stm32f1xx_hal::prelude::*;

static mut BUTTON: MaybeUninit<gpioa::PA1<Input<PullUp>>> = MaybeUninit::uninit();
static FLAG: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();
    let mut afio = dp.AFIO.constrain();

    let mut led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut led2 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let mut led3 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut led4 = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);

    let btn1 = unsafe { &mut *BUTTON.as_mut_ptr() };
    *btn1 = gpioa.pa1.into_pull_up_input(&mut gpioa.crl);
    let btn2 = gpioa.pa4.into_pull_up_input(&mut gpioa.crl);
    let btn3 = gpiob.pb0.into_pull_up_input(&mut gpiob.crl);

    btn1.make_interrupt_source(&mut afio);
    btn1.trigger_on_edge(&mut dp.EXTI, Edge::Falling);
    btn1.enable_interrupt(&mut dp.EXTI);

    let mut ds = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let mut sh_cp = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let mut st_cp = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);

    led1.set_high();
    led2.set_high();
    led3.set_high();
    led4.set_high();

    let levels = [
        [2, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 3, 0, 0, 0, 0, 0, 0, 0],
        [1, 3, 2, 0, 0, 0, 0, 0, 0],
        [3, 1, 2, 3, 0, 0, 0, 0, 0],
        [2, 3, 1, 3, 2, 0, 0, 0, 0],
        [1, 3, 1, 2, 1, 3, 0, 0, 0],
        [3, 1, 2, 3, 1, 2, 1, 0, 0],
        [2, 3, 1, 3, 2, 1, 2, 3, 0],
        [1, 3, 2, 3, 1, 2, 3, 1, 2]
    ];

    let mut best_level = 0;
    let mut current_level = 1;

    let mut stage = 1;

    let mut stage1_init = false;
    let mut stage2_init = false;

    let mut step = 0;
    let mut error = false;

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI1);
    }

    loop {
        if stage == 1 {
            if !stage1_init {
                stage1_init = true;

                led1.set_high();
                led2.set_high();
                led3.set_high();
                led4.set_low();

                display_number(&mut ds, &mut sh_cp, &mut st_cp, best_level, [true, false, false, false]);
            }

            if FLAG.load(Ordering::SeqCst) {
                FLAG.store(false, Ordering::SeqCst);

                stage = 2;

                led4.set_high();
            }
        } else if stage == 2 {
            if !stage2_init {
                stage2_init = true;

                display_number(&mut ds, &mut sh_cp, &mut st_cp, current_level, [true, false, false, false]);

                delay(2_000_000);

                let sequence = levels[(current_level - 1) as usize];

                show_sequence(sequence, &mut led1, &mut led2, &mut led3, &mut led4);

                stage = 3;
            }
        } else if stage == 3 {
            let sequence = levels[(current_level - 1) as usize];
            let steps = count_non_zeros(sequence);

            if btn1.is_low() {
                if sequence[step] != 1 {
                    stage = 4;
                    error = true;
                } else {
                    led1.set_low();
                    delay(2_000_000);
                    led1.set_high();

                    step += 1;
                }
            } else if btn2.is_low() {
                if sequence[step] != 2 {
                    stage = 4;
                    error = true;
                } else {
                    led2.set_low();
                    delay(2_000_000);
                    led2.set_high();

                    step += 1;
                }
            } else if btn3.is_low() {
                if sequence[step] != 3 {
                    stage = 4;
                    error = true;
                } else {
                    led3.set_low();
                    delay(2_000_000);
                    led3.set_high();

                    step += 1;
                }
            }

            if step >= steps {
                stage = 4;
                error = false;
            }
        } else if stage == 4 {
            if error {
                current_level = 1;

                for _ in 0..100000 {
                    display_number(&mut ds, &mut sh_cp, &mut st_cp, 10, [false, false, true, false]);
                    display_number(&mut ds, &mut sh_cp, &mut st_cp, 11, [false, true, false, false]);
                    display_number(&mut ds, &mut sh_cp, &mut st_cp, 11, [true, false, false, false]);
                }
                
                delay(2_000_000);

                stage = 1;

                stage1_init = false;
                stage2_init = false;
                step = 0;
                error = false;
            } else {
                current_level += 1;

                if best_level < current_level {
                    best_level = current_level;
                }

                display_number(&mut ds, &mut sh_cp, &mut st_cp, best_level, [true, false, false, false]);

                delay(2_000_000);

                led1.set_low();
                delay(2_000_000);
                led2.set_low();
                delay(2_000_000);
                led3.set_low();
                delay(2_000_000);
                led1.set_high();
                led2.set_high();
                led3.set_high();

                stage = 2;

                stage1_init = false;
                stage2_init = false;
                step = 0;
                error = false;
            }
        }
    }
}

fn display_number(
    ds: &mut Pin<'A', 9, Output>,
    sh_cp: &mut Pin<'A', 8, Output>,
    st_cp: &mut Pin<'B', 5, Output>,
    number: u8,
    placement: [bool; 4]
) {
    let segments = [
        // централ, верх-лево, низ-лево, низ, низ-право, верх-право, верх
        [false, true, true, true, true, true, true], // 0
        [false, false, false, false, true, true, false], // 1
        [true, false, true, true, false, true, true], // 2
        [true, false, false, true, true, true, true], // 3
        [true, true, false, false, true, true, false], // 4
        [true, true, false, true, true, false, true], // 5
        [true, true, true, true, true, false, true], // 6
        [false, false, false, false, true, true, true], // 7
        [true, true, true, true, true, true, true],  // 8
        [true, true, false, true, true, true, true], // 9,
        [true, true, true, true, false, false, true], // E,
        [true, false, true, false, false, false, false], // r
    ];

    // НАЧАЛО
    st_cp.set_low();
    sh_cp.set_low();

    // Точка для дробей
    ds.set_high();
    sh_cp.set_high();
    sh_cp.set_low();

    // Палочки, из которых состоит число
    for pos in 0..7 {
        if segments[number as usize][pos] {
            ds.set_low();
        } else {
            ds.set_high();
        }
        sh_cp.set_high();
        sh_cp.set_low();
    }

    // Протолкнуть данные выше во второй регистр
    for _ in 0..4 {
        ds.set_high();
        sh_cp.set_high();
        sh_cp.set_low();
    }

    // Какая позиция на табло горит
    for &place in &placement {
        if place {
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

fn show_sequence(
    sequence: [i32; 9],
    led1: &mut Pin<'A', 5, Output>,
    led2: &mut Pin<'A', 6, Output>,
    led3: &mut Pin<'A', 7, Output>,
    led4: &mut Pin<'B', 6, Output>,
) {
    for &num in &sequence {
        if num > 0 {
            let index = num;
            if index == 1 {
                led1.set_low();
            } else if index == 2 {
                led2.set_low();
            } else if index == 3 {
                led3.set_low();
            } else if index == 4 {
                led4.set_low();
            }

            delay(2_000_000);
            
            if index == 1 {
                led1.set_high();
            } else if index == 2 {
                led2.set_high();
            } else if index == 3 {
                led3.set_high();
            } else if index == 4 {
                led4.set_high();
            }
        }
    }
}

fn count_non_zeros(sequence: [i32; 9]) -> usize {
    let mut count = 0;
    for &num in &sequence {
        if num != 0 {
            count += 1;
        }
    }
    count
}

#[interrupt]
fn EXTI1() {
    let button = unsafe { &mut *BUTTON.as_mut_ptr() };

    if button.check_interrupt() {
        FLAG.store(true, Ordering::SeqCst);
        button.clear_interrupt_pending_bit();
    }
}
