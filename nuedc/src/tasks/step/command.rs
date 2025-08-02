use core::result::Result as CoreResult;
use embedded_io_async::{ErrorType, Write};

pub type Result<I> = CoreResult<(), <I as ErrorType>::Error>;

pub enum StepCommand {
    Sync,
    Enable,
    Stop,
    Disable,

    /// bool: enable scale down speed
    /// After Enable: speed /= 10
    ScaleDownSpeed(bool),
    SetZeroPoint,
    GoZeroPoint,

    /// (Reverse, Speed, Acceleration)
    /// Reverse is false for forward, true for backward.
    SetSpeed(bool, u16, u8),

    ReadOption,
}

impl StepCommand {
    pub async fn send<I>(self, iface: &mut I, id: u8, wait: bool) -> Result<I>
    where
        I: Write,
    {
        let cmd = match self {
            StepCommand::Sync => &mut [id, 0xFF, 0x66, 0x00][..],
            StepCommand::Enable => &mut [id, 0xF3, 0xAB, 1, wait as u8, 0x00],
            StepCommand::Stop => &mut [id, 0xFE, 0x98, wait as u8, 0x00],
            StepCommand::Disable => &mut [id, 0xF3, 0xAB, 0, wait as u8, 0x00],

            StepCommand::ScaleDownSpeed(x) => &mut [id, 0x4F, 0x71, 1, x as u8, 0x00],
            StepCommand::SetZeroPoint => &mut [id, 0x93, 0x88, 1, 0x00],
            StepCommand::GoZeroPoint => &mut [id, 0x9A, 0x00, wait as u8, 0x00],

            StepCommand::SetSpeed(r, s, a) => {
                let [s1, s2] = s.to_be_bytes();
                &mut [id, 0xF6, r as u8, s1, s2, a, wait as u8, 0x00]
            }

            StepCommand::ReadOption => &mut [id, 0x1F, 0x00],
        };

        let checksum = cmd.iter().fold(0u8, |acc, &x| acc ^ x);
        cmd.last_mut().map(|last| *last = checksum);

        iface.write(cmd).await.and(iface.flush().await)
    }
}
