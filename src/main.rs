mod datetime;
mod idle;
mod power;
mod swaybar;
mod wifi;

use crossbeam_channel::unbounded;

use crate::swaybar::Block;

fn main() {
    let (s, r) = unbounded();

    let threads = vec![
        crate::idle::start(0, "pauza", s.clone()),
        crate::wifi::start(1, "wifi", s.clone()),
        crate::power::start(2, "power", s.clone()),
        crate::datetime::start(3, "datetime", s),
    ];

    let mut blocks: Vec<Option<Block>> = vec![None; threads.len()];

    println!("{{\"version\":1}}\n[");
    loop {
        let (offset, newblock) = r.recv().unwrap();

        if let Some(block) = blocks.get_mut(offset as usize) {
            *block = Some(newblock);
        }

        while let Ok((offset, newblock)) = r.try_recv() {
            if let Some(block) = blocks.get_mut(offset as usize) {
                *block = Some(newblock);
            }
        }

        let blocks: Vec<Block> = blocks.iter().flatten().cloned().collect();
        println!("{},", serde_json::to_string(&blocks).unwrap());
    }
}
