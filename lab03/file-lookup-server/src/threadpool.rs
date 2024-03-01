use std::{
    sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: i32,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: i32, common_receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            handle: thread::spawn(move || {
                loop {
                    let job = common_receiver.lock().unwrap().recv().unwrap();
                    println!("Worker id={} finally got a job!", {id});
                    job();
                }
            }),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>
}

impl ThreadPool {
    pub fn new(size: i32) -> ThreadPool {
        let (tx, rx) = mpsc::channel::<Job>();
        let common_receiver = Arc::new(Mutex::new(rx));
        ThreadPool {
            workers: (0..size).map(|id| Worker::new(id, Arc::clone(&common_receiver))).collect(),
            sender: tx
        }
    }

    pub fn execute<F>(&self, fun: F) -> ()
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(fun));
    }
}
