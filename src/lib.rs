// Authored 2026 Jelly Terra <jellyterra@proton.me>

use std::sync::{Condvar, Mutex};

pub struct SyncBarrier {
    count: Mutex<usize>,
    cvar: Condvar,
}

impl SyncBarrier {
    pub fn new() -> Self {
        Self {
            count: Mutex::new(0),
            cvar: Condvar::new(),
        }
    }

    pub fn add(&self, delta: usize) {
        let mut count = self.count.lock().unwrap();
        *count += delta;
    }

    pub fn done(&self) {
        let mut count = self.count.lock().unwrap();

        assert!(*count != 0, "counter broke zero");

        *count -= 1;

        if *count == 0 {
            // Wake up.
            self.cvar.notify_all();
        }
    }

    pub fn wait(&self) {
        let mut count = self.count.lock().unwrap();
        while *count != 0 {
            // Free lock and block execution flow.
            count = self.cvar.wait(count).unwrap();
        }
        // Be woken up.
    }
}
