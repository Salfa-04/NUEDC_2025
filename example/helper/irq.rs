use super::prelude::hal::{self, bind_interrupts, peripherals};

bind_interrupts! {
    pub struct IRQ {
        // USART1 => hal::usart::InterruptHandler<peripherals::USART1>;
    }
}
