use std::{
    sync::{mpsc, mpsc::*},
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CEvent, KeyEvent};

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct SecdecimEvents {
    rx: Receiver<Event<KeyEvent>>,
}

impl SecdecimEvents {
    pub fn new() -> SecdecimEvents {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);
        thread::spawn(move || -> ! {
            let last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("Polling event has failed!") {
                    if let CEvent::Key(key) = event::read().expect("Unable to read the event!") {
                        tx.send(Event::Input(key)).expect("Unable to send the event!");
                    }
                }                
            }
        });
        SecdecimEvents { rx }
    }

    pub fn next(&self) -> Event<KeyEvent> {
        return self.rx.recv().unwrap_or(Event::Tick);
    }
}
