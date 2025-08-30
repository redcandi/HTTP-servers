use std::{
    thread,
    sync::{mpsc,Arc,Mutex},
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker{
    id : usize,
    handle : thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id : id, handle : handle}
    }
}

impl ThreadPool{
    pub fn new(size : usize) -> ThreadPool{
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers : Vec<Worker> = Vec::with_capacity(size);
        
        for id in 0..size{
           workers.push(Worker::new(id,Arc::clone(&receiver))); 
        }

        ThreadPool{workers : workers, sender : Some(sender)}
    }

    pub fn execute<F>(&self, f:F)
    where
        F : FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        drop(self.sender.take());

        for worker in self.workers.drain(..){
            println!("Shutting down worker {}", worker.id);
            worker.handle.join().unwrap();
        }
    }
}
