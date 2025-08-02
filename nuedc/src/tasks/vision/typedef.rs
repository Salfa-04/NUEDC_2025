use core::str::from_utf8;
use core::sync::atomic::AtomicI16;
use core::sync::atomic::Ordering::Relaxed;

static VISION: (AtomicI16, AtomicI16) = (
    AtomicI16::new(super::task::OFFSET_X),
    AtomicI16::new(super::task::OFFSET_Y),
);

pub fn get_vision() -> (i16, i16) {
    (VISION.0.load(Relaxed), VISION.1.load(Relaxed))
}

pub fn set_vision(p: (i16, i16)) {
    let last_op = get_vision();
    let raw = if p == (0, 0) { last_op } else { p };

    // EMA filter
    let ema = |op: i16, op_last: i16| {
        let alpha = 0.28; // Smoothing factor:  0 < alpha <1
        (op as f32 * alpha + op_last as f32 * (1.0 - alpha)) as i16
    };

    let op = (ema(raw.0, last_op.0), ema(raw.1, last_op.1));

    // defmt::debug!("Vision: {} -> {}", raw, op);

    VISION.0.store(op.0, Relaxed);
    VISION.1.store(op.1, Relaxed);

    // VISION.0.store(p.0, Relaxed);
    // VISION.1.store(p.1, Relaxed);
}

pub fn vision_parse(data: &[u8]) -> Option<(i16, i16)> {
    for data in data.split(|&x| x == b'\n') {
        if let Ok(s) = from_utf8(data.trim_ascii()) {
            if !s.starts_with('[') || !s.ends_with(']') || !s.contains(',') {
                continue;
            }

            if let Some((x, y)) = s[1..s.len() - 1].split_once(',') {
                match (x.trim().parse::<i16>(), y.trim().parse::<i16>()) {
                    (Ok(x), Ok(y)) => {
                        return Some((x, y));
                    }

                    _ => defmt::error!("Vision MV: [{:?}]", (x, y)),
                }
            }
        }
    }

    None
}
