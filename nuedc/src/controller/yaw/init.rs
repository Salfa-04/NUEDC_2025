use super::{CYCLE, set_offset};
use crate::{T, tasks};

use pid::Pid;
use tasks::imu::get_imu_data;
use tasks::imu::get_yaw_data;

/// cycle in ms
pub async fn init(cycle: u8) -> Pid<f32> {
    let _ = CYCLE.init(cycle); // init cycle

    while let imu = get_imu_data().await
        && imu.verified == false
    {
        T::after_millis(300).await
    }

    set_offset(get_yaw_data());

    // Todo: update setpoint to a transform
    let mut pid = Pid::new(0., 1200.);

    {
        pid.p(42.0, 1200.);
        pid.i(0.02, 360.);
        // pid.d(0.001, 12.);
    }

    pid
}
