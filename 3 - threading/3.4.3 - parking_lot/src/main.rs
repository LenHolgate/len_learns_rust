use parking_lot::deadlock;
use parking_lot::Mutex;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct ChannelThread<T> {
    channel: Option<Sender<T>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl<T: Send + 'static> ChannelThread<T> {
    fn new<F>(mut f: F) -> Self
    where
        F: FnMut(T) -> bool + Send + 'static,
    {
        let (to_thread, from_controller) = mpsc::channel::<T>();

        let handle = Some(thread::spawn(move || {
            println!("spawned thread has started");

            println!("spawned is running");

            loop {
                match from_controller.recv() {
                    Ok(message) => {
                        if !f(message) {
                            break;
                        }
                    }
                    Err(reason) => {
                        println!("thread recv error {}", reason);
                        break;
                    }
                }
            }
            println!("spawned thread is done");
        }));

        ChannelThread {
            channel: Some(to_thread),
            handle,
        }
    }

    fn send(&self, message: T) {
        self.channel
            .as_ref()
            .expect("Too late to send")
            .send(message)
            .expect("failed to send");
    }

    fn shutdown(&mut self) {
        if let Some(sender) = self.channel.take() {
            self.channel = None;

            drop(sender);
        }
    }

    fn join(&mut self) {
        if let Some(handle) = self.handle.take() {
            self.handle = None;

            handle.join().expect("failed to join with thread");
        } else {
            panic!("already joined");
        }
    }
}

fn main() {
    // Create a background thread which checks for deadlocks every 1s
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());

                    std::process::abort();
                    //panic!("Deadlock detected");
                }
            }
        }
    });

    let data1 = Arc::new(Mutex::new("data1".to_string()));
    let data2 = Arc::new(Mutex::new("data2".to_string()));

    let shared_data1 = Arc::clone(&data1);
    let shared_data2 = Arc::clone(&data2);

    let mut thread = ChannelThread::new(move |message| {
        println!("got message {}", message);

        let _lock1 = shared_data1.lock(); //expect("failed to lock data1");

        sleep(Duration::from_millis(100));

        let _lock2 = shared_data2.lock(); //expect("failed to lock data2");

        return true;
    });

    for i in 1..15 {
        println!("sending {} to thread", i);
        thread.send(i.to_string());

        let _lock2 = data2.lock(); //expect("failed to data2");

        sleep(Duration::from_millis(100));

        let _lock1 = data1.lock(); //expect("failed to data1");
    }

    println!("close channel, signal thread we're done");

    thread.shutdown();

    println!("wait for thread to end");

    thread.join();

    println!("all done...");
}
