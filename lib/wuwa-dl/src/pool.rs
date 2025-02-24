use crate::prelude::*;

pub struct Pool {
    pub sender: Sender<PoolOp>,
    pub watcher: Receiver<usize>,
}

pub enum PoolOp {
    Attach = -1,
    Dettach = 1,
}

impl Pool {
    pub fn new() -> DynResult<Self> {
        let (sender, mut rx) = mpsc::channel::<PoolOp>(1);
        let (tx, watcher) = watch::channel(0);

        tx.send(Handle::current().metrics().num_workers() - 1)?;

        tokio::spawn(async move {
            while let Some(op) = rx.recv().await {
                tx.send_if_modified(|counter| {
                    let result = counter.checked_add_signed(op as isize);

                    match result {
                        Some(c) => *counter = c,
                        None => (),
                    }

                    result.is_some()
                });
            }

            DynResult::Ok(())
        });

        Ok(Self { sender, watcher })
    }
}
