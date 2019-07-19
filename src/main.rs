use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct App {
    counter: usize,
}

impl App {
    pub fn new(until: usize) -> Self {
        Self { counter: until }
    }

    pub fn count_down(&mut self) -> Option<usize> {
        self.counter -= 1;

        if self.counter > 0 {
            Some(self.counter)
        } else {
            None
        }
    }
}

fn main() {
    let mut app = App::new(10);
    let terminated = Arc::new(AtomicBool::new(false));

    let term = Arc::clone(&terminated);
    let _ = ctrlc::set_handler(move || {
        term.store(true, Ordering::Relaxed);
    });

    loop {
        if terminated.load(Ordering::Relaxed) {
            println!("Terminated!");
            return;
        }

        if let Some(counter) = app.count_down() {
            println!("{}... ", counter);
        } else {
            println!("ALARM!");
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
