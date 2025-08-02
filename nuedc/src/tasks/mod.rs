//!
//! # Tasks
//!

pub mod blinky {
    mod task;

    pub use task::task;
}

pub mod step {
    mod command;
    mod stepper;
    mod task;

    pub use task::task;

    pub use task::set_speed;
}

pub mod servo {
    mod task;
    mod typedef;

    pub use task::task;

    pub use task::set_servo;
}

pub mod imu {
    mod crc;
    mod task;
    mod typedef;

    pub use task::task;

    pub use task::get_imu_data;
    pub use task::get_yaw_data;
}

pub mod vision {
    mod task;
    mod typedef;

    pub use task::task;
    pub use task::{OFFSET_X, OFFSET_Y};
    pub use typedef::get_vision;
}
