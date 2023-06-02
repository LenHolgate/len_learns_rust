// Cloning the log removes the need to annotate the lifetime but doesn't do
// what we want as we now have two logs rather than one shared log

#[derive(Clone)]
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
