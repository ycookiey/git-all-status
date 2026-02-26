use crate::types::RepoStatus;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
    RepoUpdated(Box<RepoStatus>),
    ScanComplete,
}

pub fn spawn_event_reader(tx: mpsc::UnboundedSender<Event>, tick_rate_ms: u64) {
    std::thread::spawn(move || loop {
        if event::poll(Duration::from_millis(tick_rate_ms)).unwrap_or(false) {
            if let Ok(evt) = event::read() {
                let send_result = match evt {
                    CrosstermEvent::Key(key) => tx.send(Event::Key(key)),
                    CrosstermEvent::Mouse(mouse) => tx.send(Event::Mouse(mouse)),
                    _ => Ok(()),
                };
                if send_result.is_err() {
                    return;
                }
            }
        } else if tx.send(Event::Tick).is_err() {
            return;
        }
    });
}
