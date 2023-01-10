extern crate idmanager;
use idmanager::intervals::Intervals;

pub fn main() {
    let mut intervals = Intervals::new();

    if !intervals.insert_interval(10, 20) {
        panic!("Unexpected, failed to insert")
    }

    if !intervals.insert_value(8) {
        panic!("Unexpected, failed to insert")
    }

    if intervals.insert_value(11) {
        panic!("Unexpected insert allowed")
    }
}
