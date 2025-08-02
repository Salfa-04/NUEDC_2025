use super::command::{Result, StepCommand};
use crate::hal::{mode::Async, usart::UartRx};

use embassy_time::{Duration, WithTimeout};
use embedded_io_async::Write;

#[derive(defmt::Format)]
pub struct Stepper<I: Write> {
    id: u8,
    interface: I,
}

impl<I: Write> Stepper<I> {
    pub const fn new(iface: I, id: u8) -> Self {
        Self {
            id,
            interface: iface,
        }
    }

    pub const fn get_id(&self) -> u8 {
        self.id
    }
}

impl<I: Write> Stepper<I> {
    pub async fn sync(&mut self) -> Result<I> {
        let iface = &mut self.interface;
        StepCommand::Sync.send(iface, self.id, false).await
    }

    pub async fn enable(&mut self) -> Result<I> {
        let iface = &mut self.interface;
        StepCommand::Enable.send(iface, self.id, false).await
    }

    pub async fn stop(&mut self, wait: bool) -> Result<I> {
        let iface = &mut self.interface;
        StepCommand::Stop.send(iface, self.id, wait).await
    }

    pub async fn disable(&mut self, wait: bool) -> Result<I> {
        let iface = &mut self.interface;
        StepCommand::Disable.send(iface, self.id, wait).await
    }
}

impl<I: Write> Stepper<I> {
    pub async fn set_speed_scale_down(&mut self) -> Result<I> {
        let iface = &mut self.interface;
        StepCommand::ScaleDownSpeed(true)
            .send(iface, self.id, false)
            .await
    }

    pub async fn set_zero_point(&mut self) -> Result<I> {
        let iface = &mut self.interface;
        StepCommand::SetZeroPoint.send(iface, self.id, false).await
    }

    pub async fn go_zero_point(&mut self, wait: bool) -> Result<I> {
        let iface = &mut self.interface;
        StepCommand::GoZeroPoint.send(iface, self.id, wait).await
    }
}

impl<I: Write> Stepper<I> {
    /// Speed is in RPM, u16, 0 ~ 5000
    pub async fn set_speed(&mut self, speed: i16, accel: u8, wait: bool) -> Result<I> {
        let pn = speed < 0;
        let s = if pn { -speed } else { speed } as u16;
        let iface = &mut self.interface;
        StepCommand::SetSpeed(pn, s, accel)
            .send(iface, self.id, wait)
            .await
    }
}

impl<I: Write> Stepper<I> {
    pub async fn wait_ok(&mut self, r: &mut UartRx<'_, Async>) -> bool {
        let mut buffer = [0u8; 10];
        let iface = &mut self.interface;

        let timeout = Duration::from_millis(10);

        let mut async_task = async || {
            if StepCommand::ReadOption
                .send(iface, self.id, false)
                .await
                .is_err()
            {
                return false;
            }

            let data = match r.read_until_idle(&mut buffer).await {
                Ok(size) if size >= 4 => &buffer[..size],
                _ => return false,
            };

            if data.iter().fold(0u8, |acc, &x| acc ^ x) != 0 {
                return false;
            }

            true
        };

        if let Err(_) = async_task().with_timeout(timeout).await {
            return false;
        }

        true
    }
}
