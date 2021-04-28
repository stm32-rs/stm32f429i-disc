#![no_main]
#![no_std]

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

        // (Re-)configure PG13 (green LED) as output
        let mut led = gpiog.pg13.into_push_pull_output();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Toggle LED
            led.toggle().ok();

            // Delay a second
            delay.delay_ms(1000_u16);
        }
    }

    loop {
        continue;
    }
}
