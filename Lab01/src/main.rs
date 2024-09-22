#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use hal::{pac, prelude::*};
use stm32f1xx_hal as hal;

#[entry]
fn main() -> ! {
    // Получаем доступ к периферийным устройствам
    let dp = pac::Peripherals::take().unwrap();

    // Получаем доступ к периферийному устройству GPIOA
    let mut gpioa = dp.GPIOA.split();

    // Конфигурируем вывод A5 как выход с подъемом.
    // Регистры `crl` передаются функции для настройки порта.
    // Для пинов 8-15 следует передавать crh.
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let button = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);

    let mut once_lighted = false;
    let mut once_pressed = false;
    
    loop {
        if button.is_low() && !once_pressed {
            // Устранение гипотетического дребезга контактов 0.1 мс
            delay((0.1 * 4_000_000.0) as u32);
            if button.is_low() {
                once_pressed = true;
            }
        }

        if once_pressed && !once_lighted {
            once_lighted = true;
            
            // 4 МГЦ — тактовая чистота, поэтому 4 млн. циклов = 1 секунда
            delay(5 * 4_000_000);

            // Лампочка зажигается
            led.set_high();

            delay(5 * 4_000_000);

            // Лампочка гаснет
            led.set_low();
        }
    }
}