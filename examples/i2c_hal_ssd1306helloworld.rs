#![no_main]
#![no_std]

use stm32f429i_disc as board;

use ssd1306::displayrotation::DisplayRotation;
use ssd1306::mode::TerminalMode;
use ssd1306::{Builder, I2CDIBuilder};

use board::hal::i2c::*;
use board::hal::prelude::*;
use board::hal::stm32;

use core::fmt::Write;

#[cortex_m_rt::entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        let gpiob = p.GPIOB.split();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Set up the clocks, going to fast exhibits some problem so let's take it slow for now
        let clocks = rcc.cfgr.sysclk(40.mhz()).freeze();

        // Set up the SCL pin of the I2C bus at PB6
        let scl = gpiob
            .pb6
            .into_alternate_af4()
            .internal_pull_up(true)
            .set_open_drain();

        // Set up the SDA pin of the I2C bus at PB7
        let sda = gpiob
            .pb7
            .into_alternate_af4()
            .internal_pull_up(true)
            .set_open_drain();

        // Setup I2C1 using the above defined pins at 400kHz bitrate (fast mode)
        let i2c = I2c::new(p.I2C1, (scl, sda), 400.khz(), clocks);

        // Set up the SSD1306 display at I2C address 0x3c
        let interface = I2CDIBuilder::new().init(i2c);
        let mut disp: TerminalMode<_,_> = Builder::new().connect(interface).into();

        // Set display rotation to 180 degrees
        let _ = disp.set_rotation(DisplayRotation::Rotate180);

        // Init and clear the display
        let _ = disp.init().unwrap();
        let _ = disp.clear();

        // Output "Hello world!" to the screen
        let _ = write!(disp, "Hello world!");
    }

    loop {
        continue;
    }
}
