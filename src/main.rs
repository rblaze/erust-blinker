#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;
use stm32l0xx_hal::pac;
use stm32l0xx_hal::prelude::*;
use stm32l0xx_hal::rcc::Config;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIO peripherals. This also enables the clock for GPIO in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);

    // Configure PA5 as output.
    let mut led = gpioa.pa5.into_push_pull_output();

    // Configure PC13 as input.
    let button = gpioc.pc13.into_floating_input();

    loop {
        let state = match button.is_high() {
            Ok(true) => PinState::High,
            Ok(false) => PinState::Low,
            _ => unreachable!(),
        };

        led.set_state(state).unwrap();
    }
}
