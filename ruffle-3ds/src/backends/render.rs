//! Stub render backend.
//! Will implement ruffle_render::backend::RenderBackend using citro3d.

pub struct Ctr3dRenderBackend {
    pub width: u32,
    pub height: u32,
}

impl Ctr3dRenderBackend {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
