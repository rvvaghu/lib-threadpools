use super::util::job::{
    Work,
    Work::{NextWork, Terminate},
};
//use std::sync::{mpsc::Receiver, Arc, Mutex};
use crossbeam::channel::Receiver;
use std::thread::{spawn, JoinHandle};

pub(crate) struct UnitThread {
    pub(crate) id: usize,
    pub(crate) handle: Option<JoinHandle<()>>,
}

impl UnitThread {
    pub(crate) fn new(id: usize, r_work: Receiver<Work>) -> Self {
        let handle: JoinHandle<()> = spawn(move || loop {
            let work = r_work.recv().unwrap();

            match work {
                Terminate => break,
                NextWork(w) => w.call_box(),
            }
        });

        Self {
            id,
            handle: Some(handle),
        }
    }
}
