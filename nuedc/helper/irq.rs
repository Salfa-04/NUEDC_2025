use super::prelude::hal::{self, bind_interrupts, peripherals};

bind_interrupts! {
    pub struct IRQ {
        USART1 => hal::usart::InterruptHandler<peripherals::USART1>;
        USART2 => hal::usart::InterruptHandler<peripherals::USART2>;
        USART3 => hal::usart::InterruptHandler<peripherals::USART3>;
        UART4 => hal::usart::InterruptHandler<peripherals::UART4>;
    }
}
