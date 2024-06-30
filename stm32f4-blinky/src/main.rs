#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*, delay::Delay};
#[allow(unused_imports)]


#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    // Get handles to the hardware objects. These functions can only be called
    // once, so that the borrowchecker can ensure you don't reconfigure
    // something by accident.
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();
    let gpiod = dp.GPIOD.split();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze();

    let mut delay = Delay::new(cp.SYST, &clocks);

    // Configure the LED pin as a push-pull output
    let mut orange_led = gpiod.pd13.into_push_pull_output();
    let mut green_led = gpiod.pd12.into_push_pull_output();
    let mut red_led = gpiod.pd14.into_push_pull_output();
    let mut blue_led = gpiod.pd15.into_push_pull_output();

    // Blink the LED
    loop {
        orange_led.set_high();
        green_led.set_high();
        red_led.set_high();
        blue_led.set_high();
        delay.delay_ms(500_u16);
        orange_led.set_low();
        green_led.set_low();
        red_led.set_low();
        blue_led.set_low();
        delay.delay_ms(500_u16);
    }
}
