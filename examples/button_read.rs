#![no_main]
#![no_std]

use panic_halt as _;

use stm32f429i_disc as board;

use cortex_m_rt::entry;

use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;

use cortex_m::peripheral::Peripherals;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiog = p.GPIOG.split();
        let gpioa = p.GPIOA.split();

        // (Re-)configure PG13 and PG14 (green LED & red LED) as outputs
        let mut green_led = gpiog.pg13.into_push_pull_output();
        let mut red_led = gpiog.pg14.into_push_pull_output();

        // Configure PA0 as input (user button)
        let button = gpioa.pa0.into_pull_down_input();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Toggle green LED constantly
            green_led.toggle().unwrap();

            // Toggle red LED only if user button is pressed
            if button.is_high().unwrap() == true {
                red_led.toggle().unwrap();
            }

            // Delay a second
            delay.delay_ms(1000_u16);
        }
    }

    loop {
        continue;
    }
}
