//! Stub audio backend.
//! Will implement ruffle_core::backend::audio::AudioBackend using NDSP.

pub struct NdspAudioBackend {
    pub volume: f32,
}

impl NdspAudioBackend {
    pub fn new() -> Self {
        Self { volume: 1.0 }
    }
}
