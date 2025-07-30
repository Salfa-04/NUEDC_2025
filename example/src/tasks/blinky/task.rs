//!
//! # LED Task
//!

use crate::{hal, init_ticker};

use gpio::{AnyPin, Level, Output as OP, Speed};
use hal::{gpio, peripherals, wdg};
use {peripherals::IWDG, wdg::IndependentWatchdog as Dog};

#[embassy_executor::task]
pub async fn task(p: (IWDG, AnyPin)) -> ! {
    let mut t = init_ticker!(150); // 150ms

    let mut led = OP::new(p.1, Level::Low, Speed::Low);
    let mut dog = Dog::new(p.0, 200_000); // 200ms

    // dog.unleash(); // Start the WatchDog

    loop {
        (led.toggle(), dog.pet());
        t.next().await
    }
}
