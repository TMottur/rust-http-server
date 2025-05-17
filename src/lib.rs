use std::{
    sync::{Arc, mpsc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
/// Create a new ThreadPool.
        /// 
        /// The size is the number of threads in the pool.
        /// 
        /// # Panics
        /// 
        /// The 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F> (&self, f: F)
    where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Some(sender) = &self.sender {
            if let Err(e) = sender.send(job) {
                eprintln!("Failed to send job to worker thread: {e}");
            }
        } else {
            eprintln!("ThreadPool sender is already dropped");
        }
    }
}

impl Drop for ThreadPool {
    fn drop (&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
          let thread = thread::spawn(move || loop {
            let message = match receiver.lock() {
                Ok(guard) => guard.recv(),
                Err(e) => {
                    eprintln!("Worker {id} failed to acquire lock: {e}");
                    break;
                }
            };

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");

                    job()
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
          });
          Worker {
            id,
            thread: Some(thread),
        }
    }
}
