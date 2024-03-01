use std::process::Command;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use crossbeam_channel::Sender;
use crate::swaybar::Block;

#[derive(Debug)]
enum WifiStatus {
    Error,
    Down,
    Up(String),
}

fn iw() -> WifiStatus {
    let res = Command::new("iw").args(["wlan0", "info"]).output();
    let Ok(output) = res else { return WifiStatus::Error };
    let Ok(stdout) = std::str::from_utf8(&output.stdout) else { return WifiStatus::Error };
    stdout.lines().find_map(|line: &str| {
        let line = line.trim();
        if let Some((key, val)) = line.split_once(' ') {
            if key == "ssid" {
                return Some(WifiStatus::Up(val.to_string()));
            }
        }
        None
    }).unwrap_or(WifiStatus::Down)
}

pub fn start(
    offset: u8, name: &str, sender: Sender<(u8, Block)>
) -> JoinHandle<()> {

    let name = name.to_string();
    let instance = "main".to_string();

    spawn(move || {
        loop {
            let mut full_text = String::new();
            full_text.push(' ');
            match iw() {
                WifiStatus::Error => full_text.push_str("📡‼️"),
                WifiStatus::Down => full_text.push_str("📡🔻"),
                WifiStatus::Up(wifi) => {
                    full_text.push('📡');
                    full_text.push_str(&wifi);
                },
            }
            full_text.push(' ');
            sender.send((offset, Block {
                full_text,
                background: None,
                name: Some(name.clone()),
                instance: Some(instance.clone()),
                .. Default::default()
            })).unwrap();
            sleep(Duration::from_secs(1));
        }
    })
}

#[cfg(test)]
mod test {
    use super::iw;

    #[test]
    fn test_iw() {
        dbg!(iw());
        panic!();
    }
}
