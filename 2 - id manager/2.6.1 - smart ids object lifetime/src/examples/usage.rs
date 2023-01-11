extern crate idmanager;

use idmanager::intervals::Intervals;
use idmanager::id_manager::IdManager;

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

    let mut id_manager = IdManager::new();

    if "[0,255]" != id_manager.dump()
    {
        panic!("Unexpected initial content")
    }

    if !id_manager.can_allocate() {
        panic!("Unexpected, cannot allocate id")
    }

    let id = id_manager.allocate();

    id_manager.free(id);
}
