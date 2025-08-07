use super::numext::NumExt;
use crate::sync::lazy_lock::LazyLock;
use core::f32::consts::TAU;

pub static CIRCLE: LazyLock<[(i16, i16); 800]> = LazyLock::new(|| {
    const A: u16 = 60; // in mm
    const T: u16 = 800; // in ms
    const O: f32 = const { TAU / T as f32 };

    let mut t = 0.0;

    unsafe { core::mem::zeroed::<[(); 800]>() }.map(|_| -> (i16, i16) {
        let x = (A as f32 * (O * t).sin()) as i16;
        let y = (A as f32 * (O * t).cos()) as i16;
        t += 1.;
        (x, y)
    })
});

pub static SINE_WAVE: LazyLock<[(i16, i16); 240]> = LazyLock::new(|| {
    const X_SIZE: u16 = 120;
    const Y_SIZE: u16 = 60;

    const T: u16 = 60;
    const O: f32 = const { TAU / T as f32 };

    let mut x = -(X_SIZE as i16);

    unsafe { core::mem::zeroed::<[(); 240]>() }.map(|_| -> (i16, i16) {
        let point = (x, (Y_SIZE as f32 * (O * x as f32).sin()) as i16);
        x += 1;

        point
    })
});
