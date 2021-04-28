#![no_main]
#![no_std]

use stm32f429i_disc as board;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;
use board::led::{Color, Leds};

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        // Constrain clock registers
        let rcc = p.RCC.constrain();
        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();
        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);
        //get gpio G
        let gpiog = p.GPIOG.split();
        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiog);

        // Endlessly blink the 2 LEDs every 500 ms
        loop {
            delay.delay_ms(500_u16);
            leds[Color::Red].toggle();
            delay.delay_ms(500_u16);
            leds[Color::Green].toggle();
        }
    }

    loop {}
}
