#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use core::mem::MaybeUninit;
use cortex_m::asm;
use cortex_m_rt::entry;
use pac::interrupt;
use panic_halt as _;
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::{pac, prelude::*};

static mut LED: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA5<Output<PushPull>>> =
    MaybeUninit::uninit();
static mut BUTTON: MaybeUninit<stm32f1xx_hal::gpio::gpioc::PC13<Input<Floating>>> =
    MaybeUninit::uninit();

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    let mut afio = dp.AFIO.constrain();

    // Инициализация светодиода на PA5
    let led = unsafe { &mut *LED.as_mut_ptr() };
    *led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    // Инициализация кнопки на PC13
    let button = unsafe { &mut *BUTTON.as_mut_ptr() };
    *button = gpioc.pc13.into_floating_input(&mut gpioc.crh);

    // Настройка кнопки как источника прерывания
    button.make_interrupt_source(&mut afio);
    button.trigger_on_edge(&mut dp.EXTI, Edge::Falling);
    button.enable_interrupt(&mut dp.EXTI);

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }

    loop {}
}

#[interrupt]
fn EXTI15_10() {
    let led = unsafe { &mut *LED.as_mut_ptr() };
    let button = unsafe { &mut *BUTTON.as_mut_ptr() };

    if button.check_interrupt() {
        for _ in 0..1_000_000 {
            asm::nop();
        }

        led.toggle();

        button.clear_interrupt_pending_bit();
    }
}
