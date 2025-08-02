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
        let p = (p.USART2, p.PA3, p.PA2, p.DMA1_CH7, p.DMA1_CH6);
        s.must_spawn(tasks::step::task(p));
    }

    {
        let p = (p.TIM8, p.PC9);
        s.must_spawn(tasks::servo::task(p));
    }

    {
        let p = (p.UART4, p.PC11, p.DMA2_CH3);
        s.must_spawn(tasks::imu::task(p));
    }

    {
        let p = (p.USART3, p.PB11, p.DMA1_CH3);
        s.must_spawn(tasks::vision::task(p));
    }

    {
        let p = (p.PD2, p.PC6);
        s.must_spawn(controller::main(p));
    }
}
