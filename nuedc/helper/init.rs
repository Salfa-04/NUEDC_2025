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
        config.enable_debug_during_sleep = true;

        let rcc = &mut config.rcc;

        rcc.hsi = false; // HSI = 8MHz
        rcc.hse = Some(rcc::Hse {
            freq: mhz(8),
            mode: rcc::HseMode::Oscillator,
        });

        rcc.pll = Some(rcc::Pll {
            src: rcc::PllSource::HSE,     //  8MHz
            prediv: rcc::PllPreDiv::DIV1, //  8MHz
            mul: rcc::PllMul::MUL9,       // 72MHz
        });

        rcc.sys = rcc::Sysclk::PLL1_P; // 72MHz
        rcc.ahb_pre = rcc::AHBPrescaler::DIV1; // 72MHz
        rcc.apb1_pre = rcc::APBPrescaler::DIV2; // 36MHz
        rcc.apb2_pre = rcc::APBPrescaler::DIV1; // 72MHz
        rcc.adc_pre = rcc::ADCPrescaler::DIV6; // 12MHz

        rcc.ls = rcc::LsConfig::default_lse(); // LSE = 32.768kHz

        init(config) // SysClock = 72MHz
    };

    (peripherals,)
}
