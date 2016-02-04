use super::{Manager, Seed};

impl Manager {
    pub fn add_machine(&self, seed: Seed) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(seed);
        if q.len() == 1 {
            self.notifier.wakeup().unwrap();
        }
    }
}
