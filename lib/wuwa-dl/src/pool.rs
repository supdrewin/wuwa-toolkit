use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::utils::Volatile;

pub struct Pool {
    count: Arc<Mutex<usize>>,
}

pub struct PoolGuard {
    count: Arc<Mutex<usize>>,
}

impl Pool {
    pub fn new(count: usize) -> Self {
        let count = Arc::new(Mutex::new(count));
        Self { count }
    }

    pub fn attach(&self) -> PoolGuard {
        let count = self.count.clone();

        while_none! {{
            thread::sleep(Duration::from_millis(1));

            let mut count = count.lock().unwrap();
            let status = count.checked_sub(1);

            match status {
                Some(t) => *count = t,
                None => (),
            }

            status
        }}

        PoolGuard { count }
    }
}

impl Volatile for PoolGuard {}

impl Drop for PoolGuard {
    fn drop(&mut self) {
        *self.count.lock().unwrap() += 1;
    }
}
