//! On-board user LEDs

use stm32f4xx_hal as hal; // memory layout

use hal::prelude::*;

use hal::gpio::gpiog::{self, PG, PG13, PG14};
use hal::gpio::{Output, PushPull};

///  Green LED
pub type LD3 = PG13<Output<PushPull>>;

/// Red LED
pub type LD4 = PG14<Output<PushPull>>;

/// Led Colors. Each one matches one of the user LEDs.
pub enum Color {
    /// Green / LD3
    Green,
    /// Red / LD4
    Red,
}

// Array of the on-board user LEDs
pub struct Leds {
    leds: [Led; 2],
}

impl Leds {
    pub fn new(gpiog: gpiog::Parts) -> Self {
        let green = gpiog.pg13.into_push_pull_output();
        let blue = gpiog.pg14.into_push_pull_output();
        Leds {
            leds: [green.into(), blue.into()],
        }
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::Index<Color> for Leds {
    type Output = Led;

    fn index(&self, c: Color) -> &Led {
        &self.leds[c as usize]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl core::ops::IndexMut<Color> for Leds {
    fn index_mut(&mut self, c: Color) -> &mut Led {
        &mut self.leds[c as usize]
    }
}

/// One of the on-board user LEDs
pub struct Led {
    pin: PG<Output<PushPull>>,
}

macro_rules! ctor {
	($($ldx:ident),+) => {
		$(
			impl Into<Led> for $ldx {
				fn into(self) -> Led {
					Led {
						pin: self.downgrade(),
					}
				}
			}
		)+
	}
}

ctor!(LD3, LD4);

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pin.set_low().ok();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pin.set_high().ok();
    }

    /// Toggles the LED
    pub fn toggle(&mut self) {
        if let Ok(true) = self.pin.is_low() {
            self.pin.set_high().ok();
        } else {
            self.pin.set_low().ok();
        }
    }
}
