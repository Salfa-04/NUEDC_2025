//!
//! # Pwm Task
//!

use crate::{hal::peripherals, init_ticker};

pub(super) use peripherals::{PC9 as PWM_PIN, TIM8 as TIMER};

use super::typedef::{ServoPwm, pwm_init};
use embassy_sync::{self as sync, once_lock, signal};
use sync::blocking_mutex::raw::ThreadModeRawMutex as RM;
use {once_lock::OnceLock, signal::Signal};

static MAX_DUTY_CYCLE: OnceLock<u16> = OnceLock::new();
static DUTY_CYCLE: Signal<RM, Option<f32>> = Signal::new();

/// PWM Duty Cycle Set
/// x, y: from -135 to 135
pub async fn set_servo(angle: Option<f32>) {
    let Some(x) = angle else {
        DUTY_CYCLE.signal(None);
        return;
    };

    // defmt::debug!("SetServo: {}", x);

    let x = x.clamp(-30.0, 30.0);

    let max = *MAX_DUTY_CYCLE.get().await;
    // duty_cycle_percent = (x / 135° + 1.5) / 20ms
    //          x = -135° to 135°
    // set = duty_cycle_percent * duty_cycle_max
    DUTY_CYCLE.signal(Some((-x as f32 + 202.5) * max as f32 / 2700f32))
}

#[embassy_executor::task]
pub async fn task(p: (TIMER, PWM_PIN)) -> ! {
    let mut t = init_ticker!(20);

    let (mut pwm_ch, max_duty_cycle) = pwm_init(p).await;

    MAX_DUTY_CYCLE.init(max_duty_cycle).unwrap();

    // Duty Cycle Step Calc:
    // Servo Speed: 0.16s/60°
    // ~ => 160ms/60° => 20ms/7.5°
    // ~ => ∆7.5°  ~ ∆7.5/2700 * max ms
    // ~ => max / 360 ms
    let duty_step = max_duty_cycle as f32 / 600f32;
    let mut servo = ServoPwm::new(duty_step);
    pwm_ch.set_duty_cycle_fraction(3, 40);

    defmt::debug!("{}: Servo initialized!", file!());

    loop {
        if DUTY_CYCLE.signaled() {
            if let Some(x) = DUTY_CYCLE.try_take() {
                servo.set(x);
            }
        }

        if servo.finished() {
            let duty_cycle = DUTY_CYCLE.wait().await;
            servo.set(duty_cycle);
        }

        // Update Step Duty Cycle
        if let Some(o) = servo.calc() {
            // defmt::debug!("Servo: {}", o);
            pwm_ch.set_duty_cycle(o as u16);
        }

        t.next().await;
    }
}
