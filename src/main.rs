mod config;
mod discover;
mod player;

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

// Global flag that any thread can read/write safely
pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

fn main() {
    let cfg = config::load();

    // Register the Ctrl+C / SIGTERM handler
    ctrlc::set_handler(move || {
        eprintln!("\nShutdown signal received. Cleaning up...");
        SHUTDOWN.store(true, Ordering::SeqCst);
    }).expect("Error setting signal handler");

    let mut delay_secs = 2;

    loop {
        // Check if we should exit before starting a new scan
        if SHUTDOWN.load(Ordering::SeqCst) {
            break;
        }

        match discover::find_ip_by_mac(&cfg.arp_scan_path, &cfg.camera.mac, &cfg.subnet, &cfg.iface) {
            Some(ip) => {
                delay_secs = 2;
                if let Err(e) = player::run(&cfg.ffplay_path, &cfg.camera, &ip) {
                    eprintln!("Failed to spawn ffplay: {}", e);
                }
            }
            None => {
                eprintln!("Camera not found. Retrying in {}s...", delay_secs);

                // Sleep in small chunks so we can react to Ctrl+C quickly
                for _ in 0..delay_secs {
                    if SHUTDOWN.load(Ordering::SeqCst) { break; }
                    std::thread::sleep(Duration::from_secs(1));
                }
                delay_secs = (delay_secs * 2).min(30);
            }
        }
    }

    eprintln!("Daemon stopped cleanly.");
}
