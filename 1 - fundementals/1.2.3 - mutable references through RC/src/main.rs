// Using std::rc::Rc rather than explicitly annotating the required lifetime

use std::rc::Rc;

struct Log {
    log_lines: std::sync::Mutex<Vec<String>>,
}

impl Log {
    fn new() -> Self {
        Log {
            log_lines: std::sync::Mutex::new(Vec::new()),
        }
    }

    fn log(&self, message: &str) {
        self.log_lines
            .lock()
            .expect("failed to lock")
            .push(message.to_string());

        println!("{}", message);
    }

    fn dump(&self) {
        println!("log contains...");

        let log_lines = self.log_lines.lock().expect("failed to lock");

        for line in log_lines.iter() {
            println!("{}", line);
        }
    }
}

struct ThingThatLogs {
    log: Rc<Log>,
}

impl ThingThatLogs {
    fn new(log: &Rc<Log>) -> Self {
        log.log("created");
        ThingThatLogs { log: log.clone() }
    }

    fn do_thing(&self) {
        self.log.log("doing thing");
    }

    fn do_other_thing(&mut self) {}
}

impl Drop for ThingThatLogs {
    fn drop(&mut self) {
        self.log.log("destroyed");
    }
}

fn main() {
    let log = Rc::new(Log::new());

    {
        let mut thing1 = ThingThatLogs::new(&log);

        thing1.do_thing();

        {
            let thing2 = ThingThatLogs::new(&log);

            thing2.do_thing();
        }

        thing1.do_other_thing();
    }

    println!("done");

    log.dump();
}
