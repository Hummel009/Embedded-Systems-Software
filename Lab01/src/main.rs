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

static mut BUTTON: MaybeUninit<gpioc::PC13<Input<Floating>>> = MaybeUninit::uninit();
static FLAG: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    rcc.cfgr.sysclk(8.MHz()).freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    let mut afio = dp.AFIO.constrain();

    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    let button = unsafe { &mut *BUTTON.as_mut_ptr() };
    *button = gpioc.pc13.into_floating_input(&mut gpioc.crh);

    button.make_interrupt_source(&mut afio);
    button.trigger_on_edge(&mut dp.EXTI, Edge::Falling);
    button.enable_interrupt(&mut dp.EXTI);

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }

    loop {
        if FLAG.load(Ordering::SeqCst) {
            FLAG.store(false, Ordering::SeqCst);

            delay(2 * 4_000_000);

            led.set_high();
        }
    }
}

#[interrupt]
fn EXTI15_10() {
    let button = unsafe { &mut *BUTTON.as_mut_ptr() };

    if button.check_interrupt() {
        FLAG.store(true, Ordering::SeqCst);
        button.clear_interrupt_pending_bit();
    }
}