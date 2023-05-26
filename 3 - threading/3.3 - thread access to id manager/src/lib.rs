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
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::Mutex;

    use idmanager::Id;
    use idmanager::IdManager;
    use idmanager::ReusePolicy::ReuseSlow;

    #[test]
    fn test_channel_thread_with_id_manager() {
        let id_manager = IdManager::<u8>::new(ReuseSlow);
        let shared_manager = id_manager.clone();

        let data = Arc::new(Mutex::new(HashMap::<String, Id<u8>>::new()));
        let shared_data = Arc::clone(&data);

        let mut thread = ChannelThread::new(move |message| {
            let id = shared_manager.allocate_id();

            println!("got message {} - {}", message, id.value());

            shared_data
                .lock()
                .expect("failed to lock data")
                .insert(message, id);

            return true;
        });

        for i in 1..15 {
            println!("sending {} to thread", i);
            thread.send(i.to_string());
        }

        println!("ids: {}", id_manager.dump());

        println!("close channel, signal thread we're done");

        thread.shutdown();

        println!("wait for thread to end");

        thread.join();

        println!("ids: {}", id_manager.dump());

        {
            let data = data.lock().expect("failed to lock data");

            for named_id in data.iter() {
                println!("id: {} - {}", named_id.0, named_id.1.value());
            }
        }

        {
            let mut data = data.lock().expect("failed to lock data");

            let keys: Vec<String> = data.keys().cloned().collect();

            for key in keys {
                if let Some(id) = data.remove(&key) {
                    println!("id: {} - {}", key, id.value());

                    println!("ids: {}", id_manager.dump());
                }
            }
        }

        println!("ids: {}", id_manager.dump());

        println!("all done...");
    }
}
