use std::{sync::mpsc::{self, Receiver, RecvError, Sender}, thread, time::Duration};

use crossterm::event::{self, Event, KeyEvent};

pub enum FEvents {
    Input(KeyEvent),
    Tick
}

pub struct EventHandler {
    _tx: Sender<FEvents>,
    rx: Receiver<FEvents>
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (tx, rx) = mpsc::channel();

        let tx_clone = tx.clone();
        thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap() && let Event::Key(key) = event::read().unwrap() {
                  tx_clone.send(FEvents::Input(key)).unwrap();
            } 

            tx_clone.send(FEvents::Tick).unwrap();
        });

        Self {
            _tx: tx,
            rx
        }

    }

    pub fn next(&self) -> Result<FEvents, RecvError> {
        self.rx.recv()
    }
}
