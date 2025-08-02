use super::task::{PWM_PIN, TIMER};
use crate::{T, hal};

use gpio::{Level, Output, OutputType, Speed};
use hal::timer::simple_pwm::SimplePwmChannel;
use hal::{gpio, time::hz, timer};
use timer::low_level::CountingMode::EdgeAlignedUp;
use timer::simple_pwm::{PwmPin, SimplePwm};

pub struct ServoPwm {
    duty_cycle: Option<f32>,
    duty_cycle_step: f32,
    duty_step: f32,
}

impl ServoPwm {
    pub const fn new(duty_step: f32) -> ServoPwm {
        let s = unsafe { core::mem::zeroed() };
        Self { duty_step, ..s }
    }

    pub fn set(&mut self, set: Option<f32>) {
        if self.duty_cycle.is_none() {
            if let Some(x) = set {
                self.duty_cycle_step = x;
            }
        }

        self.duty_cycle = set;
    }

    pub fn finished(&self) -> bool {
        if let Some(duty_cycle) = self.duty_cycle {
            duty_cycle == self.duty_cycle_step
        } else {
            true
        }
    }

    pub fn calc(&mut self) -> Option<f32> {
        if let Some(duty_cycle) = self.duty_cycle {
            if duty_cycle > self.duty_cycle_step {
                self.duty_cycle_step += self.duty_step;
                if duty_cycle <= self.duty_cycle_step {
                    self.duty_cycle_step = duty_cycle;
                }
            } else if duty_cycle < self.duty_cycle_step {
                self.duty_cycle_step -= self.duty_step;
                if duty_cycle >= self.duty_cycle_step {
                    self.duty_cycle_step = duty_cycle;
                }
            }

            Some(self.duty_cycle_step)
        } else {
            None
        }
    }
}

pub async fn pwm_init(p: (TIMER, PWM_PIN)) -> (SimplePwmChannel<'static, TIMER>, u16) {
    let pwm_pin = PwmPin::new_ch4(p.1, OutputType::PushPull);
    let pwm = SimplePwm::new(p.0, None, None, None, Some(pwm_pin), hz(50), EdgeAlignedUp);

    let max_duty_cycle = pwm.max_duty_cycle();
    let channel = pwm.split();
    let mut pwm_ch = channel.ch4;

    pwm_ch.enable();

    (pwm_ch, max_duty_cycle)
}
