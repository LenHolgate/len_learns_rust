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
    log: &Log,
}

impl ThingThatLogs {
    fn new(log: &Log) -> Self {
        log.log("created");
        ThingThatLogs { log }
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
