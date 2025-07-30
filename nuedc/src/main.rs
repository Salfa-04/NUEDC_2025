#![no_std]
#![no_main]

use utils::init_ticker;
use utils::prelude::*;

mod controller;
mod tasks;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    {
        use hal::gpio::Pin;
        let p = (p.IWDG, p.PC7.degrade());
        s.must_spawn(tasks::blinky::task(p));
    }

    {
        let p = ();
        s.must_spawn(controller::main(p));
    }
}
