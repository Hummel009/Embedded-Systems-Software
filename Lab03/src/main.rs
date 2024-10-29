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
use stm32f1xx_hal::serial::{Config, Serial};

extern crate libm;

static mut BTN_MODE_SELECT: MaybeUninit<gpioa::PA1<Input<PullUp>>> = MaybeUninit::uninit();
static FLAG: AtomicBool = AtomicBool::new(false);

static mut VALUE_BITS: [u8; 4] = [0; 4];

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    let mut afio = dp.AFIO.constrain();
    let channels = dp.DMA1.split();

    let rx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let tx = gpioa.pa3;

    let serial = Serial::new(
        dp.USART2,
        (rx, tx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &clocks,
    );

    let mut tx = serial.tx.with_dma(channels.7);

    let mut led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut led2 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let mut led3 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut led4 = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);

    let btn_mode_select = unsafe { &mut *BTN_MODE_SELECT.as_mut_ptr() };
    *btn_mode_select = gpioa.pa1.into_pull_up_input(&mut gpioa.crl);
    let btn_switch = gpiob.pb0.into_pull_up_input(&mut gpiob.crl);

    btn_mode_select.make_interrupt_source(&mut afio);
    btn_mode_select.trigger_on_edge(&mut dp.EXTI, Edge::Falling);
    btn_mode_select.enable_interrupt(&mut dp.EXTI);

    led1.set_high();
    led2.set_high();
    led3.set_high();
    led4.set_low();

    let mut ds = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let mut sh_cp = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let mut st_cp = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI1);
    }

    let mut mode = "none";

    let mut a: i32 = 1;
    let mut f: i32 = 2;
    let mut phi: i32 = 3;

    let mut i = 0;
    loop {
        i += 1;

        if FLAG.load(Ordering::SeqCst) {
            FLAG.store(false, Ordering::SeqCst);

            if mode == "none" {
                mode = "a";

                led1.set_low();
                led2.set_high();
                led3.set_high();
                led4.set_high();
            } else if mode == "a" {
                mode = "f";

                led1.set_high();
                led2.set_low();
                led3.set_high();
                led4.set_high();
            } else if mode == "f" {
                mode = "phi";

                led1.set_high();
                led2.set_high();
                led3.set_low();
                led4.set_high();
            } else if mode == "phi" {
                mode = "a";

                led1.set_low();
                led2.set_high();
                led3.set_high();
                led4.set_high();
            }
        }

        if mode == "a" {
            if btn_switch.is_low() {
                delay(2_000_000);
                if a >= 0 && a < 9 {
                    a += 1;
                } else {
                    a = 0;
                }
            }
            display_number(
                &mut ds,
                &mut sh_cp,
                &mut st_cp,
                a,
                [true, false, false, false],
            );
        } else if mode == "f" {
            if btn_switch.is_low() {
                delay(2_000_000);
                if f >= 0 && f < 9 {
                    f += 1;
                } else {
                    f = 0;
                }
            }
            display_number(
                &mut ds,
                &mut sh_cp,
                &mut st_cp,
                f,
                [true, false, false, false],
            );
        } else if mode == "phi" {
            if btn_switch.is_low() {
                delay(2_000_000);
                if phi >= 0 && phi < 9 {
                    phi += 1;
                } else {
                    phi = 0;
                }
            }
            display_number(
                &mut ds,
                &mut sh_cp,
                &mut st_cp,
                phi,
                [true, false, false, false],
            );
        }

        let value = generate_sine_wave(a * 10, f * 10, i, 44100, phi * 10);

        unsafe {
            VALUE_BITS = value.to_be_bytes();

            let (_, loop_tx) = tx.write(&VALUE_BITS).wait();
            tx = loop_tx;
        }

        delay(100_000);
    }
}

fn generate_sine_wave(a: i32, f: i32, i: i32, n: i32, phi0: i32) -> f32 {
    return a as f32 * libm::sinf(2.0 * 3.14 * f as f32 * i as f32 / n as f32 + phi0 as f32);
}

fn display_number(
    ds: &mut Pin<'A', 9, Output>,
    sh_cp: &mut Pin<'A', 8, Output>,
    st_cp: &mut Pin<'B', 5, Output>,
    number: i32,
    placement: [bool; 4],
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

#[interrupt]
fn EXTI1() {
    let btn_mode_select = unsafe { &mut *BTN_MODE_SELECT.as_mut_ptr() };

    if btn_mode_select.check_interrupt() {
        FLAG.store(true, Ordering::SeqCst);
        btn_mode_select.clear_interrupt_pending_bit();
    }
}