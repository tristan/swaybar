use std::time::{Instant, Duration};
use std::thread::{sleep, spawn, JoinHandle};
use crossbeam_channel::{Sender, unbounded};

use wayland_client::{Display, GlobalManager};
use wayland_client::protocol::wl_seat;

use crate::swaybar::Block;
use self::generated::client::org_kde_kwin_idle::OrgKdeKwinIdle;
use self::generated::client::org_kde_kwin_idle_timeout::Event;

const IDLE_PAUSE_TIME: Duration = Duration::from_secs(30);
const IDLE_RESET_TIME: Duration = Duration::from_secs(300);
const SHORT_BREAK_TIME: Duration = Duration::from_secs(1500);
const SHORT_BREAK_DURATION: Duration = Duration::from_secs(60);
const BREAK_TIME: Duration = Duration::from_secs(2700);

#[derive(Debug)]
pub enum PauzaEvent {
    UpdateTime,
    PauseTimer,
    ResumeTimer,
    ShortBreakTaken,
    ResetTimer,
}

pub fn start(
    offset: u8, name: &str, sender: Sender<(u8, Block)>
) -> JoinHandle<()> {
    let name = name.to_string();
    let instance = "main".to_string();

    let (s, r) = unbounded();

    // update timer
    {
        let s = s.clone();
        spawn(move || {
            loop {
                sleep(Duration::from_secs(1));
                s.send(PauzaEvent::UpdateTime).unwrap();
            }
        });
    }

    // wayland/sway idle detection
    {
        let s = s.clone();
        spawn(move || {
            let display = Display::connect_to_env().unwrap();
            let mut event_queue = display.create_event_queue();
            let attached_display = display.attach(event_queue.token());
            let globals = GlobalManager::new(&attached_display);

            event_queue.sync_roundtrip(&mut (), |_, _, _| unreachable!()).unwrap();

            let seat = globals.instantiate_exact::<wl_seat::WlSeat>(1).unwrap();
            let idle = globals.instantiate_exact::<OrgKdeKwinIdle>(1).unwrap();

            let pause = idle.get_idle_timeout(&seat, IDLE_PAUSE_TIME.as_millis() as _);
            {
                let s = s.clone();
                pause.quick_assign(move |_timeout, event, _| {
                    match event {
                        Event::Idle => {
                            s.send(PauzaEvent::PauseTimer).unwrap();
                        },
                        Event::Resumed => {
                            s.send(PauzaEvent::ResumeTimer).unwrap();
                        }
                    }
                })
            }
            let short_break = idle.get_idle_timeout(&seat, SHORT_BREAK_DURATION.as_millis() as _);
            {
                let s = s.clone();
                short_break.quick_assign(move |_timeout, event, _| {
                    match event {
                        Event::Idle => {
                            s.send(PauzaEvent::ShortBreakTaken).unwrap();
                        },
                        Event::Resumed => {}
                    }
                })
            }
            let reset = idle.get_idle_timeout(&seat, IDLE_RESET_TIME.as_millis() as _);
            {
                let s = s.clone();
                reset.quick_assign(move |_timeout, event, _| {
                    match event {
                        Event::Idle => {
                            s.send(PauzaEvent::ResetTimer).unwrap();
                        },
                        Event::Resumed => {}
                    }
                })
            }

            loop {
                event_queue.dispatch(&mut (), |_, _, _| {
                    /* we ignore unfiltered messages */
                }).unwrap();
            }
        });
    }

    spawn(move || {
        let mut start = Instant::now();
        let mut has_reset = false;
        let mut short_break_taken = false;
        let mut pause_start: Option<Instant> = None;
        loop {
            let block = match r.recv().unwrap() {
                PauzaEvent::UpdateTime => {
                    if pause_start.is_some() {
                        continue;
                    }
                    let elapsed = start.elapsed();
                    let full_text = {
                        let total_secs = elapsed.as_secs();
                        let minutes = total_secs / 60;
                        let seconds = total_secs - (minutes * 60);
                        format!(" {:02}:{:02} ", minutes, seconds)
                    };
                    let background = if elapsed > BREAK_TIME {
                        Some("FF0000".to_string())
                    } else if !short_break_taken && elapsed > SHORT_BREAK_TIME {
                        Some("FF4500".to_string())
                    } else {
                        None
                    };
                    Some(Block {
                        full_text,
                        background,
                        .. Default::default()
                    })
                },
                PauzaEvent::PauseTimer => {
                    if pause_start.is_none() {
                        pause_start = Some(Instant::now());
                    }
                    None
                },
                PauzaEvent::ResumeTimer => {
                    if has_reset {
                        has_reset = false;
                        short_break_taken = false;
                        start = Instant::now();
                        pause_start = None;
                    } else if let Some(pause_start) = pause_start.take() {
                        let pause_duration = pause_start.elapsed();
                        start += pause_duration;
                    }
                    None
                },
                PauzaEvent::ResetTimer => {
                    if !has_reset {
                        has_reset = true;
                        Some(Block {
                            full_text: " 00:00 ".to_string(),
                            .. Default::default()
                        })
                    } else {
                        None
                    }
                },
                PauzaEvent::ShortBreakTaken => {
                    if let Some(ps) = pause_start.take() {
                        let pause_duration = ps.elapsed();
                        start += pause_duration;
                        pause_start = Some(Instant::now());
                    }
                    let elapsed = start.elapsed();
                    if !short_break_taken && elapsed > SHORT_BREAK_TIME {
                        short_break_taken = true;
                        // clear the color of the bar
                        let full_text = {
                            let total_secs = elapsed.as_secs();
                            let minutes = total_secs / 60;
                            let seconds = total_secs - (minutes * 60);
                            format!(" {:02}:{:02} ", minutes, seconds)
                        };
                        Some(Block {
                            full_text,
                            .. Default::default()
                        })
                    } else {
                        None
                    }
                }
            };
            if let Some(mut block) = block {
                block.name = Some(name.clone());
                block.instance = Some(instance.clone());
                block.border_left = Some(3);
                block.border_right = Some(3);
                sender.send((offset, block)).unwrap();
            }
        }
    })
}

mod generated {
    #![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
    #![allow(non_upper_case_globals,non_snake_case,unused_imports)]
    #![allow(missing_docs, clippy::all)]

    pub mod client {
        //! Client-side API of this protocol
        pub(crate) use wayland_client::{Main, Attached, Proxy, ProxyMap, AnonymousObject};
        pub(crate) use wayland_commons::map::{Object, ObjectMetadata};
        pub(crate) use wayland_commons::{Interface, MessageGroup};
        pub(crate) use wayland_commons::wire::{Argument, MessageDesc, ArgumentType, Message};
        pub(crate) use wayland_commons::smallvec;
        pub(crate) use wayland_client::protocol::wl_seat;
        pub(crate) use wayland_client::sys;

        include!(concat!(env!("OUT_DIR"), "/idle_client_api.rs"));
    }
}
