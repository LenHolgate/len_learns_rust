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
            .expect("too late to send")
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

    #[test]
    fn test_channel_thread_with_closure() {
        let mut thread = ChannelThread::new(|_message| {
            return true;
        });

        for i in 1..15 {
            println!("sending {} to thread", i);
            thread.send(i);
        }

        println!("close channel, signal thread we're done");

        thread.shutdown();

        println!("wait for thread to end");

        thread.join();

        println!("all done...");
    }

    fn process_message_u32(message : u32) -> bool {
        println!("got message {}", message);
        return true;
    }

    #[test]
    fn test_channel_thread_with_function() {
        let mut thread = ChannelThread::new(process_message_u32);

        for i in 1..15 {
            println!("sending {} to thread", i);
            thread.send(i);
        }

        println!("close channel, signal thread we're done");

        thread.shutdown();

        println!("wait for thread to end");

        thread.join();

        println!("all done...");
    }

    struct Message {
        name : String,
        value : i32

    }

    fn process_message(message : Message) -> bool {
        println!("got message {}", message.name);

        if message.value == 10 {
            return false;
        }

        return true;
    }

    #[test]
    fn test_channel_thread_with_custom_message() {
        let mut thread = ChannelThread::new(process_message);

        for i in 1..15 {
            println!("sending {} to thread", i);
            thread.send(Message {name : i.to_string(), value: i });
        }

        println!("close channel, signal thread we're done");

        thread.shutdown();

        println!("wait for thread to end");

        thread.join();

        println!("all done...");
    }
}
