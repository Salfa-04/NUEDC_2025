pub trait NumExt {
    /// Arctangent of y/x (f32)
    /// Returns a value in radians, in the range of -pi to pi.
    fn atan2(self, other: f32) -> f32;

    /// The sine of `x` (f32).
    ///
    /// `x` is specified in radians.
    /// Computes the sine of a number (in radians).
    fn sin(self) -> f32;

    fn cos(self) -> f32;
}

impl NumExt for f32 {
    fn atan2(self, other: f32) -> f32 {
        let (y, x) = (self, other);

        // use libm
        if cfg!(false) {
            use libm::atan2f;
            atan2f(y, x)
        } else {
            use micromath::F32;
            F32(y).atan2(F32(x)).0
        }
    }

    fn sin(self) -> f32 {
        if cfg!(false) {
            use libm::sinf;
            sinf(self)
        } else {
            use micromath::F32;
            F32(self).sin().0
        }
    }

    fn cos(self) -> f32 {
        if cfg!(false) {
            use libm::cosf;
            cosf(self)
        } else {
            use micromath::F32;
            F32(self).cos().0
        }
    }
}
