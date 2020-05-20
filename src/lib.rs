pub mod threadpool;

pub mod util;

mod unit_thread;




#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn init_threadpool() {}
}
