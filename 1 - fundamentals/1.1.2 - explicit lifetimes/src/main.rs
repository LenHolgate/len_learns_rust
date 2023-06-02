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

struct ThingThatLogs<'a> {
    log: &'a Log,
}

impl<'a> ThingThatLogs<'a> {
    fn new(log: &'a Log) -> Self {
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
