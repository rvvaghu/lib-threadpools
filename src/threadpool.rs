use super::{unit_thread::UnitThread, util::job, util::job::Work};
use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

pub struct ThreadPool {
    threads: Vec<UnitThread>,
    s_work: Sender<job::Work>,
}
impl ThreadPool {
    /// Initializes new threadpool
    /// ```
    /// #use super::ThreadPool;
    /// #let thpool = new(5);
    /// #assert_eq!(thpool, ThreadPool{});
    /// ```
    pub fn new(no_of_threads: usize) -> Self {
        let mut threads: Vec<UnitThread> = Vec::new();

        let (s_work, r_work) = channel();

        let r_work = Arc::new(Mutex::new(r_work));

        for id in 0..=no_of_threads {
            let thread_internal = UnitThread::new(id, r_work.clone());
            threads.push(thread_internal);
        }

        Self { threads, s_work }
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

        for _ in &mut self.threads {
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
