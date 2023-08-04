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
        // pub fn exec_tasks(tasks: Arc<Mutex<Vec<Task>>>) {
        //     let mut tasks: std::sync::MutexGuard<'_, Vec<Task>> = tasks.lock().unwrap();

        //     for (i, task) in tasks.iter_mut().enumerate() {
        //         if task.status == TaskStatus::Pending {
        //             task.status = TaskStatus::Started;
        //             println!("[{:?}] [{}] Started", thread::current().id(), i);
        //             task.result = (task.task)(task.params);
        //             if task.result.len() == 0 {
        //                 task.status = TaskStatus::Failure;
        //                 println!("[{:?}] [{}] Failure", thread::current().id(), i);
        //             } else {
        //                 task.status = TaskStatus::Success;
        //                 println!("[{:?}] [{}] Success", thread::current().id(), i);
        //             }
        //         } else {
        //             println!("[{:?}] [{}] Skipped", thread::current().id(), i);
        //         }
        //     }
        // }
    }
}
