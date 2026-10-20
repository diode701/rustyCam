use std::os::unix::prelude::CommandExt;
use std::process::{Command, Stdio};
use std::time::Duration;
use crate::config::Camera;
use crate::SHUTDOWN;
use std::sync::atomic::Ordering;

pub fn run(ffplay_path: &str, camera: &Camera, ip: &str) -> std::io::Result<()> {
    let url = format!(
        "rtsp://{}:{}@{}:554/cam/realmonitor?channel={}&subtype={}",
        camera.user, camera.password, ip, camera.channel, camera.subtype,
    );

    eprintln!("[{}] starting ffplay at {}", camera.name, ip);

    let mut child = Command::new(ffplay_path)
        .args([
            "-fs", "-rtsp_transport", "tcp", "-buffer_size", "2048000",
            "-max_delay", "10000000", "-infbuf", "-analyzeduration", "10000000",
            "-probesize", "10000000", "-ec", "guess_mvs", "-reorder_queue_size", "1024",
        ])
        .arg(&url)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .process_group(0)
        .spawn()?;

    loop {
        if SHUTDOWN.load(Ordering::SeqCst) {
            eprintln!("[{}] killing ffplay...", camera.name);
            let _ = child.kill(); // Sends SIGKILL to ffplay
            let _ = child.wait(); // Reap the zombie process
            return Ok(());
        }

        match child.try_wait() {
            Ok(Some(_status)) => {
                eprintln!("[{}] ffplay exited", camera.name);
                return Ok(());
            }
            Ok(None) => {
                // Still running. Sleep for 100ms to avoid pegging the CPU
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(e); // Something went wrong checking the process
            }
        }
    }
}
