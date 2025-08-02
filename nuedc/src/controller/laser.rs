use crate::hal;

use hal::gpio::{Level, Output as OP, Speed};
use hal::peripherals::PD2;

pub struct Laser<'t> {
    pin: OP<'t>,
}

impl Laser<'_> {
    pub fn new<'t>(pin: PD2) -> Laser<'t> {
        Laser {
            pin: OP::new(pin, Level::Low, Speed::Low),
        }
    }

    pub fn set(&mut self, open: bool) {
        match open {
            true => self.pin.set_high(),
            false => self.pin.set_low(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.pin.is_set_high()
    }
}
