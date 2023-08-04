pub mod task {
    use std::thread;

    pub fn num_threads() -> usize {
        match thread::available_parallelism() {
            Ok(threads) => threads.get(),
            Err(err) => panic!("{}", err),
        }
    }

    #[derive(PartialEq)]
    pub enum TaskStatus {
        Pending,
        Started,
        Success,
        Failure,
    }

    pub struct Task {
        pub task: fn(u64) -> Vec<u64>,
        pub params: u64,
        pub status: TaskStatus,
        pub result: Vec<u64>,
    }

    impl Task {
        pub fn create_task(task: fn(u64) -> Vec<u64>, params: u64) -> Task {
            Task {
                task: task,
                params: params,
                status: TaskStatus::Pending,
                result: vec![],
            }
        }
    }
}
