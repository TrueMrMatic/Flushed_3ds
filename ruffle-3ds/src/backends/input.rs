//! 3DS input handling.
//! Maps HID buttons and touch screen to Flash input events.

pub struct InputState {
    pub touch_x: u16,
    pub touch_y: u16,
    pub keys_down: u32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            touch_x: 0,
            touch_y: 0,
            keys_down: 0,
        }
    }
}
