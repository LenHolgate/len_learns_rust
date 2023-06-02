struct Log {
    log_lines: Vec<String>,
}

impl Log {
    fn new() -> Self {
        Log {
            log_lines: Vec::new(),
        }
    }

    fn log(&mut self, message: &str) {
        self.log_lines.push(message.to_string());

        println!("{}", message);
    }

    fn dump(&self) {
        println!("log contains...");

        for line in self.log_lines.iter() {
            println!("{}", line);
        }
    }
}

struct ThingThatLogs<'a> {
    log: &'a mut Log,
}

impl<'a> ThingThatLogs<'a> {
    fn new(log: &'a mut Log) -> Self {
        log.log("created");
        ThingThatLogs { log }
    }

    fn do_thing(&mut self) {
        self.log.log("doing thing");
    }

    fn do_other_thing(&mut self) {

    }
}

// impl<'a> Drop for ThingThatLogs<'a> {
//     fn drop(&mut self) {
//         self.log.log("destroyed");
//     }
// }

fn main() {
    let mut log = Log::new();

    {
        let mut thing1 = ThingThatLogs::new(&mut log);

        thing1.do_thing();

        {
            let mut thing2 = ThingThatLogs::new(&mut log);

            thing2.do_thing();
        }

        thing1.do_other_thing();
    }

    println!("done");

    log.dump();
}
