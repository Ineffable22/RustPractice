use rayon::prelude::*;
use std::process::exit;
use std::{
    env::args,
    // sync::{Arc, Mutex},
    thread,
};
mod task_managment;
use task_managment::task::{num_threads, Task, TaskStatus};

mod prime_factors;
use prime_factors::prime_factors;

fn print_factors(task: &Task) {
    print!(
        "{}: {}{}",
        task.params,
        task.result[0],
        task.result
            .iter()
            .skip(1)
            .map(|&f| format!(" * {}", f))
            .collect::<String>()
    );
    println!();
}

fn main() {
    let ac: Vec<String> = args().collect();

    if ac.len() < 2 {
        eprintln!("Usage: {:?} <numbers>", ac[0]);
        exit(1);
    }
    let mut tasks: Vec<Task> = Vec::new();
    for i in 1..ac.len() {
        match ac[i].parse::<u64>() {
            Ok(n) => tasks.push(Task::create_task(prime_factors, n)),
            Err(err) => {
                eprintln!("{}: {}", ac[i], err);
                exit(1);
            }
        }
    }

    println!(
        "Executing {} tasks on {} threads",
        tasks.len(),
        num_threads()
    );
    tasks.par_iter_mut().enumerate().for_each(|(i, task)| {
        if task.status == TaskStatus::Pending {
            task.status = TaskStatus::Started;
            println!("[{:?}] [{}] Started", thread::current().id(), i);
            task.result = (task.task)(task.params);
            if task.result.len() == 0 {
                task.status = TaskStatus::Failure;
                println!("[{:?}] [{}] Failure", thread::current().id(), i);
            } else {
                task.status = TaskStatus::Success;
                println!("[{:?}] [{}] Success", thread::current().id(), i);
            }
        } else {
            println!("[{:?}] [{}] Skipped", thread::current().id(), i);
        }
    });
    // let tasks: Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(tasks));
    // let mut join_handle: Vec<thread::JoinHandle<()>> = Vec::new();
    // for _ in 0..num_threads() {
    //     let tasks: Arc<Mutex<Vec<Task>>> = Arc::clone(&tasks);
    //     let new_thread: thread::JoinHandle<()> = thread::spawn(move || {
    //         Task::exec_tasks(tasks);
    //     });
    //     join_handle.push(new_thread);
    // }

    // for handle in join_handle {
    //     handle.join().unwrap();
    // }

    // let tasks = tasks.lock().unwrap();
    for task in tasks.iter() {
        print_factors(task);
    }
}
