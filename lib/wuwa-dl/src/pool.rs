use std::{ops::Deref, sync::Arc, time::Duration};

use tokio::{sync::Mutex, time};

#[derive(Clone)]
pub struct Pool {
    count: Arc<Mutex<usize>>,
}

impl Deref for Pool {
    type Target = Arc<Mutex<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.count
    }
}

impl Pool {
    pub fn new(count: usize) -> Self {
        let count = Arc::new(Mutex::new(count));
        Self { count }
    }

    pub async fn attach(&self) -> Self {
        let pool = self.clone();

        while_none! {{
            time::sleep(Duration::from_millis(20)).await;

            let mut count = pool.lock().await;
            let status = count.checked_sub(1);

            match status {
                Some(c) => *count = c,
                None => (),
            }

            status
        }}

        pool
    }

    pub async fn detattch(&self) {
        *self.lock().await += 1;
    }
}
