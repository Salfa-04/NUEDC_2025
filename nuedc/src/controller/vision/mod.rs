use crate::tasks;

use pid::Pid;

use tasks::vision::{OFFSET_X, OFFSET_Y};

pub async fn init() -> (Pid<f32>, Pid<f32>) {
    let mut pid_x = Pid::new(0., 360.);
    let mut pid_y = Pid::new(0., 30.);

    {
        // 初版方案
        // pid_x.p(0.16, 10.);
        // pid_x.i(0.0028, 10.);

        // AI 方案 1
        // pid_x.p(0.14, 360.);
        // pid_x.i(0.009, 360.);

        // AI 方案 2
        pid_x.p(0.10, 360.);
        pid_x.i(0.01, 360.);
    }

    {
        // 初版方案
        // pid_y.p(0., 10.);
        // pid_y.i(0., 10.);

        // AI 方案 1
        pid_y.p(0.16, 30.);
        pid_y.i(0.005, 30.);

        // AI 方案 2
        // pid_y.p(0.1, 30.);
    }

    (pid_x, pid_y)
}

pub fn calculate(
    pid: &mut (Pid<f32>, Pid<f32>),
    measurement: (i16, i16),
) -> (f32, f32, (f32, f32)) {
    let (x, y) = pid;
    let (mx, my) = (
        measurement.0 as f32 - OFFSET_X as f32,
        measurement.1 as f32 - OFFSET_Y as f32,
    );

    let error = ((x.setpoint - mx).abs(), (y.setpoint - my).abs());
    let ox = x.next_control_output(mx as f32);
    let oy = y.next_control_output(my as f32);

    // defmt::info!("{}, {}", my, oy.output);

    (ox.output, oy.output, error)
}

pub fn reset(pid: &mut (Pid<f32>, Pid<f32>)) {
    let (x, y) = pid;

    (x.reset_integral_term(), x.setpoint(0.));
    (y.reset_integral_term(), y.setpoint(0.));
}
