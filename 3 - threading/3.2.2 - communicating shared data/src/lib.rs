#[allow(dead_code)]
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn dump_data(data: &HashSet<String>) {
        println!("data contains: {} ", data.len());

        for x in data.iter() {
            println!("data: {}", x);
        }
    }

    #[test]
    fn test_thread_share_data_by_communicating() {
        let mut data = HashSet::<String>::new();

        let (to_thread, from_controller) = mpsc::channel::<String>();
        let (to_controller, from_thread) = mpsc::channel::<HashSet<String>>();

        let thread = thread::spawn(move || {
            println!("spawned thread has started");

            println!("spawned is running");

            loop {
                match from_controller.recv() {
                    Ok(message) => {
                        data.insert(message);
                    }
                    Err(reason) => {
                        println!("thread recv error {}", reason);

                        to_controller.send(data).expect("failed to send data");
                        break;
                    }
                }
            }
            println!("spawned thread is done");
        });

        for i in 1..15 {
            println!("sending {} to thread", i);
            to_thread
                .send(i.to_string())
                .expect("failed to send to thread");
        }

        println!("close channel, signal thread we're done");

        drop(to_thread);

        println!("wait for thread to end");

        thread.join().expect("failed to join with thread");

        if let Ok(data) = from_thread.recv() {
            dump_data(&data);
        } else {
            println!("failed to get data from thread");
        }

        println!("all done...");
    }
}
