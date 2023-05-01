use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use tracing::{info, trace};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Task>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        info!(size = size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        self.sender.as_ref().unwrap().send(task).unwrap();
        trace!("task send");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        trace!("sending terminate message to all workers");
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            // trace!(message = ?message);
            let thread_id = thread::current().id();
            trace!(thread_id = ?thread_id, "task received");
            match message {
                Ok(task) => {
                    task();
                }
                Err(_) => {
                    break;
                }
            }
        });
        Worker {
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn long_living_queue_execution() {
        let pool = ThreadPool::new(2);
        let (sender, receiver) = mpsc::channel();
        for i in 0..16 {
            let sender = sender.clone();
            pool.execute(move || {
                thread::sleep(Duration::from_secs(3) + Duration::from_millis(i as u64 * 100));
                sender.send(i).unwrap();
            });
        }
        let results: Vec<i32> = receiver.iter().take(16).collect();
        assert_eq!(results.len(), 16);
    }

    #[test]
    fn execute_sends_task_when_complete() {
        let pool = ThreadPool::new(4);
        let (sender, receiver) = mpsc::channel();
        for i in 0..4 {
            let sender = sender.clone();
            pool.execute(move || {
                thread::sleep(Duration::from_secs(i as u64 + 1));
                sender.send(i).unwrap();
            });
        }
        let results: Vec<i32> = receiver.iter().take(4).collect();
        assert_eq!(results, vec![0, 1, 2, 3]);
    }

    #[test]
    fn execute_sends_task_when_complete_reverse() {
        let pool = ThreadPool::new(4);
        let (sender, receiver) = mpsc::channel();
        for i in 0..4 {
            let sender = sender.clone();
            pool.execute(move || {
                thread::sleep(Duration::from_secs(5 - i as u64));
                sender.send(i).unwrap();
            });
        }
        let results: Vec<i32> = receiver.iter().take(4).collect();
        assert_eq!(results, vec![3, 2, 1, 0]);
    }

    #[test]
    fn task_queued_on_available_worker() {
        let pool = ThreadPool::new(3);
        let (sender, receiver) = mpsc::channel();
        for i in 0..4 {
            let sender = sender.clone();
            match i {
                0 | 3 => {
                    pool.execute(move || {
                        thread::sleep(Duration::from_secs(1));
                        sender.send(i).unwrap();
                    });
                }
                _ => {
                    pool.execute(move || {
                        thread::sleep(Duration::from_secs(4 + i as u64));
                        sender.send(i).unwrap();
                    });
                }
            }
        }
        let results: Vec<i32> = receiver.iter().take(4).collect();
        assert_eq!(results, vec![0, 3, 1, 2]);
    }

    #[test]
    fn execute_runs_tasks_in_parallel() {
        let pool = ThreadPool::new(4);
        let start_time = Instant::now();
        for _ in 0..4 {
            pool.execute(|| {
                thread::sleep(Duration::from_secs(1));
            });
        }
        let elapsed_time = start_time.elapsed();
        assert!(elapsed_time < Duration::from_secs(2));
    }

    #[test]
    fn execute_handles_large_tasks() {
        let pool = ThreadPool::new(1);
        let (sender, receiver) = mpsc::channel();

        let large_data: Vec<u8> = vec![0; 100_000];

        pool.execute(move || {
            sender.send(large_data).unwrap();
        });

        let result = receiver.recv().unwrap();
        assert_eq!(result.len(), 100_000);
    }
}
