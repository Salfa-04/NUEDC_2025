use super::typedef::{set_vision, vision_parse};
use crate::hal::{gpio, peripherals, usart};
use usart::{Config, UartRx};

use peripherals::{DMA1_CH3 as U_DMA_R, PB11 as U_RX, USART3 as PERI};

pub const OFFSET_X: i16 = 400;
pub const OFFSET_Y: i16 = 240 + 10;

#[embassy_executor::task]
pub async fn task(p: (PERI, U_RX, U_DMA_R)) -> ! {
    let mut config = Config::default();
    config.baudrate = 115_200;
    config.rx_pull = gpio::Pull::Up;

    let mut rx = UartRx::new(p.0, utils::IRQ, p.1, p.2, config)
        .inspect_err(|e| defmt::error!("Vison: {:?}", e))
        .unwrap();

    defmt::debug!("{}: Vision Initialized!", file!());

    let mut buffer = [0u8; 100];

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => {
                // defmt::info!("{}", &buffer[..x]);
                if let Some(data) = vision_parse(&buffer[..x]) {
                    set_vision(data);
                }
            }

            Err(e) => defmt::error!("Vison: {:?}", e),
        }
    }
}
