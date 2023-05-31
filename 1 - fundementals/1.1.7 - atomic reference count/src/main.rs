// Where we replace the Rc with an Arc to allow the log to be used in threaded code

use std::sync::Arc;

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
    log: Arc<Log>,
}

impl ThingThatLogs {
    fn new(log: &Arc<Log>) -> Self {
        ThingThatLogs { log: log.clone() }
    }
}

fn main() {
    let log = Arc::new(Log::new());

    {
        let thing1 = ThingThatLogs::new(&log);

        {
            let thing2 = ThingThatLogs::new(&log);
        }
    }
}
