use super::util::job::{Work, Work::NextWork};
//use std::sync::{mpsc::Receiver, Arc, Mutex};
use crossbeam::{channel::Receiver, sync::ShardedLock};
use std::{
    sync::Arc,
    thread::{spawn, JoinHandle},
};

pub(crate) struct UnitThread {
    pub(crate) id: usize,
    pub(crate) handle: Option<JoinHandle<()>>,
}

impl UnitThread {
    pub(crate) fn new(
        id: usize,
        r_work: Receiver<Work>,
        terminate: Arc<ShardedLock<bool>>,
    ) -> Self {
        let handle: JoinHandle<()> = spawn(move || loop {
            {
                let t = terminate.read().unwrap();
                println!("thread {}, reading {}", id, *t);
                if *t == true {
                    break;
                };
            }

            let work = r_work.recv().unwrap();

            match work {
                NextWork(w) => w.call_box(),
                Work::Terminate => break,
            }
        });

        Self {
            id,
            handle: Some(handle),
        }
    }
}
