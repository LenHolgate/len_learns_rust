// Using encapsulation to put the Rc inside the struct that we want
// to reference count.
// Note that this probably isn't a good thing to do...

use std::rc::Rc;

#[derive(Clone)]
struct Log {
    log_lines: Rc::<Vec<String>>,
}

impl Log {
    fn new() -> Self {
        Log {
            log_lines: Rc::new(Vec::new()),
        }
    }
}

struct ThingThatLogs {
    log: Log,
}

impl ThingThatLogs {
    fn new(log: &Log) -> Self {
        ThingThatLogs { log: log.clone() }
    }
}

fn main() {
    let log = Log::new();

    {
        let thing1 = ThingThatLogs::new(&log);

        {
            let thing2 = ThingThatLogs::new(&log);
        }
    }
}
