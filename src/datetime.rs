use chrono::{DateTime, Local};
use crossbeam_channel::Sender;
use crate::swaybar::Block;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

pub fn start(
    offset: u8, name: &str, sender: Sender<(u8, Block)>
) -> JoinHandle<()> {
    let name = name.to_string();
    let instance = "main".to_string();

    spawn(move || {
        loop {
            let dt: DateTime<Local> = Local::now();
            let full_text = dt.format(" %a %d-%b-%Y %R ").to_string();
            let b = Block {
                full_text,
                name: Some(name.clone()),
                instance: Some(instance.clone()),
                .. Default::default()
            };
            sender.send((offset, b)).unwrap();
            sleep(Duration::from_secs(1));
        }
    })
}
