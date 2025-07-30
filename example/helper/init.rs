//!
//! # Init
//!
//! ## Reserved
//!
//!  (for stm32g473cbt6)
//! - PF0, PF1   for OSC
//! - PC14, PC15 for OSC32
//! - PA13, PA14 for SWD
//! - PB8        for BOOT0
//! - PG10       for RST
//!

use super::prelude::hal::{Config, init, rcc, time::mhz};

pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    defmt::debug!("System Initialization...");

    if cortex_m::singleton!(:()=()).is_none() {
        panic!("{}: Can Be Called Only Once!!!", file!());
    }

    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;

        init(config) // SysClock = xMHz
    };

    (peripherals,)
}
