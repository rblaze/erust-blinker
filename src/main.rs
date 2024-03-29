#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_probe as _;
// use panic_halt as _;

use cortex_m_rt::entry;
use rtt_target::rprintln;
use rtt_target::rtt_init_print;
use stm32g0xx_hal::pac;
use stm32g0xx_hal::prelude::*;
use stm32g0xx_hal::rcc::{Config, Prescaler};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi(Prescaler::Div16));

    // Acquire the GPIO peripherals. This also enables the clock for GPIO in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA5 as output.
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        rprintln!("loop");
        cortex_m::asm::delay(200_000);
        led.set_high().unwrap();
        cortex_m::asm::delay(200_000);
        led.set_low().unwrap();
    }
}
