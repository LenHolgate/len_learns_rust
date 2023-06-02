// Using encapsulation and the PImpl idiom as a logical extension of 1.1.5
// Note that this probably isn't a good thing to do...

use std::rc::Rc;

struct LogImpl {
    log_lines: Vec<String>,
    counter: u32,
}

impl LogImpl {
    fn new() -> Self {
        LogImpl {
            log_lines: Vec::new(),
            counter: 0,
        }
    }
}

#[derive(Clone)]
struct Log {
    log_impl: Rc<LogImpl>,
}

impl Log {
    fn new() -> Self {
        Log {
            log_impl: Rc::new(LogImpl::new()),
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
