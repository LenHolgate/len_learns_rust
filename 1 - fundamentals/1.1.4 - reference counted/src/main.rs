// Using std::rc::Rc rather than explicitly annotating the required lifetime

use std::rc::Rc;

struct Log {
    log_lines: Vec<String>,
}

impl Log {
    fn new() -> Self {
        Log {
            log_lines: Vec::new(),
        }
    }
}

struct ThingThatLogs {
    log: Rc<Log>,
}

impl ThingThatLogs {
    fn new(log: &Rc<Log>) -> Self {
        ThingThatLogs { log: log.clone() }
    }
}

fn main() {
    let log = Rc::new(Log::new());

    {
        let thing1 = ThingThatLogs::new(&log);

        {
            let thing2 = ThingThatLogs::new(&log);
        }
    }
}
