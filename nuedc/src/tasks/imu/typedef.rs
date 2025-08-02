use super::crc::get_crc16;

#[derive(Default, defmt::Format, Clone, PartialEq)]
pub enum DaMiaoIMURegister {
    Acceleration = 0x01,
    AngularVelocity = 0x02,
    EulerAngles = 0x03,

    #[default]
    NotSupported = 0xFF,
}

#[derive(Default, defmt::Format, Clone)]
pub struct DaMiaoIMU {
    pub device_id: u8,
    pub register: DaMiaoIMURegister,

    pub x: f32, // roll
    pub y: f32, // pitch
    pub z: f32, // yaw

    pub verified: bool,
}

impl DaMiaoIMU {
    pub const fn new() -> Self {
        Self {
            device_id: 0,
            register: DaMiaoIMURegister::NotSupported,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            verified: false,
        }
    }
}

impl From<&[u8]> for DaMiaoIMU {
    fn from(raw: &[u8]) -> Self {
        if raw.len() != 19 || raw[0] != 0x55 || raw[1] != 0xAA || raw[18] != 0x0A {
            return Self::default();
        }

        let convertor = |data: &[u8]| {
            let bytes = [data[0], data[1], data[2], data[3]];
            // Convert 4 bytes to a f32
            f32::from_bits(u32::from_le_bytes(bytes))
        };

        let register = match raw[3] {
            0x01 => DaMiaoIMURegister::Acceleration,
            0x02 => DaMiaoIMURegister::AngularVelocity,
            0x03 => DaMiaoIMURegister::EulerAngles,

            _ => DaMiaoIMURegister::NotSupported,
        };

        let checksum = u16::from_le_bytes([raw[16], raw[17]]);

        Self {
            device_id: raw[2],
            register,
            x: convertor(&raw[4..8]),
            y: convertor(&raw[8..12]),
            z: convertor(&raw[12..16]),
            verified: get_crc16(&raw[..16]) == checksum,
        }
    }
}
