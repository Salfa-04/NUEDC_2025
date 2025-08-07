use crate::{sync, tasks};
use core::sync::atomic::Ordering::Relaxed;
use core::sync::atomic::{AtomicI16, AtomicU32};

use pid::Pid;
use sync::once_lock::OnceLock;
use tasks::imu::get_yaw_data;
use tasks::step::set_speed;

mod init;

pub use init::init;

static CYCLE: OnceLock<u8> = OnceLock::new();
static OFFSET: AtomicU32 = AtomicU32::new(0);

fn get_offset() -> f32 {
    f32::from_bits(OFFSET.load(Relaxed))
}

pub fn reset(pid: &mut Pid<f32>) {
    // reset yaw data
    set_offset(get_yaw_data());

    // Reset yaw controller
    pid.setpoint(0.);
    pid.reset_integral_term();
}

pub(super) fn set_offset(value: f32) {
    OFFSET.store(value.to_bits(), Relaxed);
}

/// setpoint is in degrees(°)
pub async fn update(pid: &mut Pid<f32>, setpoint: Option<f32>) {
    if let Some(setpoint) = setpoint {
        pid.setpoint(setpoint);
    }

    // defmt::debug!("Yaw setpoint: {}", pid.setpoint);

    let yaw = get_yaw_data() - get_offset();
    let op: _ = pid.next_control_output(yaw);

    // defmt::info!("OP: {}", defmt::Debug2Format(&op));
    // let debug = (op.p, op.i, op.output);
    // defmt::info!("{} => {}", yaw, debug);

    // Safety: Yaw pid is limited to 1200(u16).
    let speed = op.output as i16;
    let s_last = option_output(Some(speed));

    // Safety: CYCLE is initialized already.
    let cycle = *CYCLE.get().await as f32; // in ms
    let delta_v = (speed - s_last) as f32; // in rpm

    let acc = (256. - 20. * cycle / delta_v) as u8;
    let acc = if acc < 1 { u8::MAX } else { acc };

    // defmt::debug!("Yaw: {}, Set: {}", yaw, (speed, acc));

    set_speed(Some((speed, acc)))
}

fn option_output(value: Option<i16>) -> i16 {
    static OUTPUT_LAST: AtomicI16 = AtomicI16::new(0);
    let last = OUTPUT_LAST.load(Relaxed);
    if let Some(val) = value {
        if val != last {
            OUTPUT_LAST.store(val, Relaxed);
        }
    }

    last
}
