use ctru::prelude::*;

mod backends;
mod swf_loader;

fn main() {
    let apt = Apt::new().expect("Failed to init APT");
    let mut hid = Hid::new().expect("Failed to init HID");
    let gfx = Gfx::new().expect("Failed to init GFX");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("\x1b[1;1H\x1b[36m=============================");
    println!("   Ruffle 3DS v0.1.0");
    println!("   Flash Player for 3DS");
    println!("=============================\x1b[0m\n");
    println!("Status: Skeleton Build");
    println!("Render: Stub (no GPU yet)");
    println!("Audio:  Stub (no NDSP yet)");
    println!("Input:  Basic HID polling\n");

    // Test SD card access
    match swf_loader::list_swf_files() {
        Ok(files) => {
            if files.is_empty() {
                println!("No .swf files found on SD card.");
                println!("Place SWF files in sdmc:/ruffle/");
            } else {
                println!("Found {} SWF file(s):", files.len());
                for (i, name) in files.iter().enumerate().take(10) {
                    println!("  {}. {}", i + 1, name);
                }
            }
        }
        Err(e) => {
            println!("SD card access: {}", e);
            println!("(This is normal on emulator)");
        }
    }

    println!("\n\x1b[32mPress START to exit.\x1b[0m");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }
}
