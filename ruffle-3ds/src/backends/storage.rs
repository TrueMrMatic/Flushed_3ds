//! Stub storage backend.
//! Will implement SharedObject persistence to SD card.

pub struct SdCardStorageBackend {
    pub base_path: String,
}

impl SdCardStorageBackend {
    pub fn new() -> Self {
        Self {
            base_path: String::from("sdmc:/ruffle/save/"),
        }
    }
}
