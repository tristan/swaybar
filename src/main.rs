mod swaybar;
mod idle;
mod datetime;
mod power;

use crossbeam_channel::unbounded;

use crate::swaybar::Block;

fn main() {

    let (s, r) = unbounded();

    let threads = vec![
        crate::idle::start(
            0, "pauza", s.clone()
        ),
        crate::power::start(
            1, "power", s.clone()
        ),
        crate::datetime::start(
            2, "datetime", s.clone()
        )
    ];

    let mut blocks: Vec<Option<Block>> = vec![None; threads.len()];

    println!("{{\"version\":1}}\n[");
    loop {
        let (offset, newblock) = r.recv().unwrap();

        if let Some(block) = blocks.iter_mut().nth(offset as _) {
            *block = Some(newblock);
        }

        while let Ok((offset, newblock)) = r.try_recv() {
            if let Some(block) = blocks.iter_mut().nth(offset as _) {
                *block = Some(newblock);
            }
        }

        let blocks: Vec<Block> = blocks.iter()
            .cloned().flatten().collect();
        println!("{},", serde_json::to_string(&blocks).unwrap());
    }
}
