#[allow(dead_code)]
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread::sleep;
    use std::time::Duration;

    // #[test]
    // fn test_channel_thread_with_shared_data() {
    //
    //     let mut data = HashSet::<String>::new();
    //
    //     let mut thread = ChannelThread::new( |message| {
    //         println!("got message {}", message);
    //
    //         data.insert(message);
    //
    //         return true;
    //     });
    // }

    // #[test]
    // fn test_channel_thread_with_shared_data_with_arc() {
    //
    //     let mut data = Arc::new(HashSet::<String>::new());
    //
    //     let mut shared_data = Arc::clone(&data);
    //
    //     let mut thread = ChannelThread::new( |message| {
    //         println!("got message {}", message);
    //
    //         data.insert(message);
    //
    //         return true;
    //     });
    // }

    fn dump_data(data: &Arc<Mutex<HashSet<String>>>) {
        let data = data.lock().expect("failed to lock");

        println!("data contains: {} ", data.len());

        for x in data.iter() {
            println!("data: {}", x);
        }
    }

    #[test]
    fn test_channel_thread_with_shared_data_locked() {
        let data = Arc::new(Mutex::new(HashSet::<String>::new()));

        let shared_data = Arc::clone(&data);

        let mut thread = ChannelThread::new(move |message| {
            println!("got message {}", message);

            shared_data.lock().expect("failed to lock").insert(message);
            return true;
        });

        for i in 1..15 {
            println!("sending {} to thread", i);
            thread.send(i.to_string());

            let data = data.lock().expect("failed to lock");

            println!("data contains: {} ", data.len());

            sleep(Duration::from_millis(100));
        }

        dump_data(&data);

        println!("close channel, signal thread we're done");

        thread.shutdown();

        println!("wait for thread to end");

        thread.join();

        dump_data(&data);

        println!("all done...");
    }
}
