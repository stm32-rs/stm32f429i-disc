#![no_main]
#![no_std]

use stm32f429i_disc as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("main");

    defmt::panic!()
}
