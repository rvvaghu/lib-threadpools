pub fn sleep_secs(t: u64) {
    let duration = std::time::Duration::from_secs(t);
    std::thread::sleep(duration);
}

pub fn sleep_millis(t: u64) {
    let duration = std::time::Duration::from_millis(t);
    std::thread::sleep(duration);
}

pub(crate) mod job {
    pub(crate) enum Work {
        NextWork(DoWork),
        Terminate,
    }

    pub trait FnBox {
        fn call_box(self: Box<Self>);
    }

    impl<F> FnBox for F
    where
        F: FnOnce(),
    {
        fn call_box(self: Box<F>) {
            (*self)()
        }
    }

    pub type DoWork = Box<dyn FnBox + Send + 'static>;
}
