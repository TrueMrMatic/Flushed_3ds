//! SWF file loader for 3DS SD card.

use std::fs;
use std::path::Path;

const SWF_DIR: &str = "sdmc:/ruffle/";

/// List all .swf files in the ruffle directory on SD card.
pub fn list_swf_files() -> Result<Vec<String>, String> {
    let path = Path::new(SWF_DIR);

    if !path.exists() {
        // Try to create the directory
        fs::create_dir_all(path).map_err(|e| format!("Cannot create {}: {}", SWF_DIR, e))?;
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(path)
        .map_err(|e| format!("Cannot read {}: {}", SWF_DIR, e))?;

    let mut swf_files = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.to_lowercase().ends_with(".swf") {
            swf_files.push(name);
        }
    }

    swf_files.sort();
    Ok(swf_files)
}

/// Load an SWF file from the SD card into memory.
pub fn load_swf(filename: &str) -> Result<Vec<u8>, String> {
    let path = format!("{}{}", SWF_DIR, filename);
    fs::read(&path).map_err(|e| format!("Cannot read {}: {}", path, e))
}
