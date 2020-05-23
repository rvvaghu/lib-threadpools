use super::{unit_thread::UnitThread, util::job, util::job::Work};
use crossbeam::{
    channel::{unbounded, Sender},
    sync::ShardedLock,
};
use std::sync::Arc;

pub struct ThreadPool {
    threads: Vec<UnitThread>,
    s_work: Sender<job::Work>,
    terminate: Arc<ShardedLock<bool>>,
}

impl ThreadPool {
    pub fn new(no_of_threads: usize) -> Self {
        let mut threads: Vec<UnitThread> = Vec::new();
        let (s_work, r_work) = unbounded();
        let terminate = Arc::new(ShardedLock::new(false));

        (0..no_of_threads.max(1)).for_each(|id| {
            let thread_internal = UnitThread::new(id, r_work.clone(), terminate.clone());
            threads.push(thread_internal);
        });

        Self {
            threads,
            s_work,
            terminate,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let j = Box::new(f);
        self.s_work.send(Work::NextWork(j)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        {
            *(self.terminate.write().unwrap()) = true;
        }
        for _ in &self.threads {
            self.s_work.send(Work::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for thread in &mut self.threads {
            println!("Shutting down worker {}", thread.id);

            if let Some(thread) = thread.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}
