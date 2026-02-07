//! Stub navigator backend.
//! On 3DS, network navigation is mostly a no-op.

pub struct Ctr3dNavigatorBackend;

impl Ctr3dNavigatorBackend {
    pub fn new() -> Self {
        Self
    }
}
