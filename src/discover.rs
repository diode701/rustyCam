use std::process::Command;

pub(crate) fn find_ip_by_mac(bin_path: &str, mac: &str, subnet: &str, iface: &str) -> Option<String> {
    let out = Command::new(bin_path)
        .arg("--plain")
        .arg(format!("--interface={iface}"))
        .arg("--format=${ip} ${mac}")
        .arg(subnet)
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&out.stdout);

    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let ip = match parts.next() { Some(x) => x, None => continue };
        let mac_on_line = match parts.next() { Some(x) => x, None => continue };

        if mac_on_line.eq_ignore_ascii_case(mac) {
            return Some(ip.to_string());
        }
    }
    None
}
