use std::thread::{sleep, spawn, JoinHandle};
use std::path::Path;
use std::time::Duration;
use std::io::{self, Read};
use std::fs::OpenOptions;
use std::str::FromStr;
use crossbeam_channel::Sender;
use crate::swaybar::Block;

fn read(path: &Path) -> Result<String, io::Error> {
    let mut f = OpenOptions::new().read(true).open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    // Removes trailing newline
    content.pop();
    Ok(content)
}

enum BatteryStatus {
    Unknown,
    Charging,
    Discharging,
    NotCharging,
    Full
}

impl FromStr for BatteryStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Charging" => BatteryStatus::Charging,
            "Discharging" => BatteryStatus::Discharging,
            "Not charging" => BatteryStatus::NotCharging,
            "Full" => BatteryStatus::Full,
            _ => BatteryStatus::Unknown,
        })
    }
}

impl BatteryStatus {
    fn emoji(&self) -> &'static str {
        match self {
            BatteryStatus::Charging => "⚡",
            BatteryStatus::Discharging => "🔋",
            BatteryStatus::NotCharging => "❌",
            BatteryStatus::Full => "",
            BatteryStatus::Unknown => "",
        }
    }
}

pub fn start(
    offset: u8, name: &str, sender: Sender<(u8, Block)>
) -> JoinHandle<()> {
    let name = name.to_string();
    let instance = "main".to_string();
    let path = Path::new("/sys/class/power_supply");
    let ac = path.join("AC");
    let bat0 = path.join("BAT0");

    let ac_online = ac.join("online");
    let is_plugged_in = move || -> bool {
        let ac_online = ac_online.as_path();
        if ac_online.exists() {
            if let Ok(val) = read(ac_online) {
                return val == "1"
            }
        }
        false
    };

    let bat0_status = bat0.join("status");
    let bat0_status = move || -> BatteryStatus {
        let bat0_status = bat0_status.as_path();
        if let Ok(val) = read(bat0_status) {
            FromStr::from_str(&val)
                .unwrap_or(BatteryStatus::Unknown)
        } else {
            BatteryStatus::Unknown
        }
    };

    let bat0_capacity = bat0.join("capacity");
    let bat0_capacity = move || -> Option<String> {
        let bat0_capacity = bat0_capacity.as_path();
        read(bat0_capacity).ok()
    };

    spawn(move || {
        loop {
            let mut full_text = String::new();
            full_text.push(' ');
            if is_plugged_in() {
                full_text.push('🔌');
            }
            let bat0_status = bat0_status();
            full_text.push_str(bat0_status.emoji());
            let mut background = None;
            match bat0_status {
                BatteryStatus::Charging | BatteryStatus::Discharging => {
                    if let Some(capacity) = bat0_capacity() {
                        full_text.push_str(&capacity);
                        full_text.push('%');
                        if let Ok(capacity) = capacity.parse::<i32>() {
                            if capacity <= 15 && matches!(bat0_status, BatteryStatus::Discharging) {
                                background = Some("FF4500".to_string())
                            }
                        }
                    }
                },
                _ => {}
            };
            full_text.push(' ');
            sender.send((offset, Block {
                full_text,
                background,
                name: Some(name.clone()),
                instance: Some(instance.clone()),
                .. Default::default()
            })).unwrap();
            sleep(Duration::from_secs(1));
        }
    })
}
