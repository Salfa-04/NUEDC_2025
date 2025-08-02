//!
//! # IMU Task
//!

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering::Relaxed;

use super::typedef::{DaMiaoIMU, DaMiaoIMURegister};
use crate::{hal, sync};

use peripherals::{DMA2_CH3 as DMA, PC11 as PIN, UART4 as PERI};

use hal::{gpio::Pull, peripherals, usart};
use raw::ThreadModeRawMutex as RM;
use sync::{blocking_mutex::raw, mutex::Mutex};
use usart::{Config, UartRx};

static IMU_DATA: Mutex<RM, DaMiaoIMU> = Mutex::new(DaMiaoIMU::new());
static YAW_GLOBAL: AtomicU32 = AtomicU32::new(0);

pub async fn get_imu_data() -> DaMiaoIMU {
    IMU_DATA.lock().await.clone()
}

pub fn get_yaw_data() -> f32 {
    f32::from_bits(YAW_GLOBAL.load(Relaxed))
}

fn set_yaw_data(yaw: f32) {
    // defmt::info!("Setting Yaw: {}", yaw);
    YAW_GLOBAL.store(yaw.to_bits(), Relaxed);
}

#[embassy_executor::task]
pub async fn task(p: (PERI, PIN, DMA)) -> ! {
    let mut config = Config::default();
    config.baudrate = 921_600;
    config.rx_pull = Pull::Up;

    // Safety: Config is valid, so Unwrap is safe.
    let mut imu = UartRx::new(p.0, utils::IRQ, p.1, p.2, config).unwrap();

    defmt::debug!("{}: IMU Initialized!", file!());

    let mut buffer = [0u8; 64];

    let (mut yaw, mut yaw_last) = (0., 0.);

    loop {
        match imu.read_until_idle(&mut buffer).await {
            Ok(len) if len > 18 => {
                let data = &buffer[..len];

                let imu: DaMiaoIMU = data.into();

                if imu.verified && imu.register == DaMiaoIMURegister::EulerAngles {
                    yaw = imu.z;
                }

                *IMU_DATA.lock().await = imu;
            }

            Err(e) => {
                defmt::error!("IMU Read Error: {:?}", e);
            }

            _ => continue,
        }

        {
            let yaw_g = get_yaw_data();
            let mut delta = yaw - yaw_last;
            yaw_last = yaw;

            if delta > 180f32 {
                delta -= 360.0;
            } else if delta < -180f32 {
                delta += 360.0;
            }

            set_yaw_data(yaw_g + delta);
        }
    }
}
