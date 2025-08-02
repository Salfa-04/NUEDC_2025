use crate::{T, hal, init_ticker, tasks};

mod laser;
mod vision;
mod yaw;

use hal::peripherals::{PC6, PD2};

use hal::gpio::{Input, Pull};
use laser::Laser;
use tasks::{servo, step, vision as V};
use vision::calculate;

#[embassy_executor::task]
pub async fn main(p: (PD2, PC6)) {
    // Wait for system stabilization
    crate::T::after_millis(100).await;
    let mut t = init_ticker!(10);

    let mut yawc = yaw::init(10).await;

    let input = Input::new(p.1, Pull::Up);

    // laser open in default state
    let laser_do = if input.is_low() {
        while input.is_low() {
            T::after_millis(10).await;
        }

        while input.is_high() {
            T::after_millis(10).await;
        }

        true
    } else {
        false
    };

    let mut visc = vision::init().await;
    let mut laser = Laser::new(p.0);

    let mut finished = false;
    let mut reverse = false;
    let mut count = 0u8;
    let mut keep_enable = false;

    loop {
        if !finished {
            let measurement = V::get_vision();

            if measurement != (V::OFFSET_X, V::OFFSET_Y) || keep_enable {
                let (ox, oy, err) = calculate(&mut visc, measurement);

                // defmt::info!("{}: {}", err, (ox, oy));

                servo::set_servo(Some(oy)).await;
                yaw::update(&mut yawc, Some(ox)).await;

                if err.0 < 2.5 && err.1 < 3. {
                    count += 1;
                } else {
                    count = 0;
                }

                if count > 10 {
                    laser.set(true);

                    if !laser_do {
                        // 未按下 PC6
                        finished = true;
                    } else {
                        // 按下 PC6
                        count = 200;
                        keep_enable = true;
                    }
                }

                // for test
                // servo::set_servo(Some(0.0)).await;
                // step::set_speed(None);
            } else {
                if reverse {
                    yawc.setpoint -= 1.5;
                } else {
                    yawc.setpoint += 1.5;
                }

                if yawc.setpoint.abs() > 180. {
                    reverse = !reverse;
                }

                yaw::update(&mut yawc, None).await;
            }
        }

        if finished {
            // Disable stepper, servo and reset yaw data
            yaw::reset(&mut yawc);
            vision::reset(&mut visc);
            step::set_speed(None);

            T::after_millis(2000).await;

            laser.set(false);
            servo::set_servo(None).await;

            t.reset();
        }

        t.next().await
    }
}
