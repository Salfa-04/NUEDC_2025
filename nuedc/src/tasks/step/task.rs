//!
//! # Stepper Task
//!

use super::stepper::Stepper;
use crate::{T, hal, init_ticker, sync};

use peripherals::{DMA1_CH6 as DMA_RX, DMA1_CH7 as DMA_TX, PA2 as TX, PA3 as RX, USART2 as PERI};

use hal::{gpio::Pull, peripherals, usart};
use raw::CriticalSectionRawMutex as RM;
use sync::blocking_mutex::raw;
use sync::signal::Signal;
use usart::{Config, Uart};

static SPEED_SET: Signal<RM, Option<(i16, u8)>> = Signal::new();
static ZP_SET: Signal<RM, ()> = Signal::new();

/// Sets the stepper speed.
/// speed: 0~±32767(i16)
/// accel: 0~255(u8)
pub fn set_speed(state: Option<(i16, u8)>) {
    #[cfg(false)]
    {
        // 2rps = 1200
        let (speed, accel) = state.unwrap_or((0, 0));
        let speed = speed as f32 / const { 10. * 60. }; // Convert to rps
        defmt::info!("Setting {} rps, accel: {}", speed, accel);
    }

    // T (ms) = Δv (rpm) × (256 - acc) × 50 × 10e-3
    SPEED_SET.signal(state)
}

pub fn set_zero_point() {
    ZP_SET.signal(())
}

#[embassy_executor::task]
pub async fn task(p: (PERI, RX, TX, DMA_TX, DMA_RX)) -> ! {
    let mut config = Config::default();
    config.baudrate = 921_600;
    config.rx_pull = Pull::Up;

    // Safety: Config is valid, so Unwrap is safe.
    let (step, mut reader) = Uart::new(p.0, p.1, p.2, utils::IRQ, p.3, p.4, config)
        .unwrap()
        .split();

    let mut step = Stepper::new(step, 1);

    while !step.wait_ok(&mut reader).await {
        T::after_millis(30).await;
        // defmt::warn!("Stepper not ready, retrying...");
    }

    let mut t = init_ticker!(3);
    let _ = step.set_speed_scale_down().await;
    let _ = t.next().await;
    let _ = step.disable(false).await;

    defmt::debug!("{}: Stepper initialized!", file!());

    // true: enabled; false: disabled
    let mut step_state = false;
    let mut step_state_last = false;
    let mut state = (0i16, 0u8);

    loop {
        if ZP_SET.signaled() {
            // defmt::info!("ZeroPoint: {}", s);
            if let Err(e) = step.set_zero_point().await {
                defmt::error!("Failed to set zero point: {}", e);
            }

            // Reset the signal to avoid re-triggering
            ZP_SET.reset();
        }

        if SPEED_SET.signaled() {
            if let Some(s) = SPEED_SET.try_take() {
                // defmt::info!("StepSpeed: {}", s);
                if let Some((speed, accel)) = s {
                    step_state = true;
                    state = (speed, accel);
                } else {
                    // Stepper is disabled
                    step_state = false;
                }
            }
        }

        // true: enabled; false: disabled
        if step_state_last != step_state && step_state {
            if let Err(e) = step.enable().await {
                defmt::error!("Failed to enable stepper: {}", e);
            }
        }

        if step_state {
            let (speed, accel) = state;
            let _ = step.set_speed(speed, accel, false).await;
            // defmt::info!("SpeedSet: {}", state);
        } else {
            if let Err(e) = step.disable(false).await {
                defmt::error!("Failed to disable stepper: {}", e);
            }

            T::after_millis(100).await;
            t.reset();
        }

        step_state_last = step_state;

        t.next().await
    }
}
