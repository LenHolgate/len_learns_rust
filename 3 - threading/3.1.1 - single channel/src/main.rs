use std::thread;
use std::sync::mpsc;

fn main() {
    let (to_thread, from_main) = mpsc::channel::<String>();

    let thread_handle = thread::spawn(move || {
        println!("spawned thread has started");

        let mut done = false;

        while !done {
            match from_main.recv() {
                Ok(message) => {
                    println!("got {} from main", message);
                }
                Err(reason) => {
                    println!("thread recv error {}", reason);
                    done = true;
                }
            }
        }
        println!("spawned thread is done");
    });

    for i in 1..15 {
        println!("sending {} to thread", i);
        to_thread.send(i.to_string()).expect("failed to send");
    }

    println!("close channel, signal thread we're done");

    drop(to_thread);

    println!("wait for thread to end");

    thread_handle.join().expect("failed to join with thread");

    println!("thread has ended");

    println!("all done...");
}
