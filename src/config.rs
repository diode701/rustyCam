use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub arp_scan_path: String,
    pub ffplay_path: String,
    pub iface: String,
    pub subnet: String,
    pub camera: Camera,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub name: String,
    pub mac: String,
    pub user: String,
    pub password: String,
    pub channel: u8,
    pub subtype: u8,
}

pub fn config_path() -> PathBuf {
    ProjectDirs::from("com", "diode", "rusty-cam") 
        .expect("no home dir")
        .config_dir()
        .join("config.toml")
}

const TEMPLATE: &str = r#"arp_scan_path = "arp-scan"
ffplay_path = "ffplay"
iface = ""
subnet = "192.168.123.123/24"

[camera]
name     = ""
mac      = ""
user     = ""
password = ""
channel  = 1
subtype  = 1
"#;

pub fn load() -> Config {
    let path = config_path();

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("mkdir config dir");
        }
        fs::write(&path, TEMPLATE).expect("write template");
        eprintln!("wrote template config to {}", path.display());
        eprintln!("edit it and restart.");
        std::process::exit(1);
    }

    let text = fs::read_to_string(&path).expect("read config");
    toml::from_str(&text).expect("parse config")
}
