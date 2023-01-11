extern crate idmanager;

use std::sync::Mutex;
use idmanager::intervals::Intervals;
use idmanager::id_manager::IdManager;
use idmanager::smart_id::SmartId;

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

    {
        let manager = Mutex::new(IdManager::new());

        assert_eq!(manager.lock().unwrap().dump(), "[0,255]");

        {
            let id1 = SmartId::new(&manager);

            let expected_id : u8 = 0;

            assert_eq!(id1.value(), &expected_id);

            assert_eq!(manager.lock().unwrap().dump(), "[1,255]");
        }

        assert_eq!(manager.lock().unwrap().dump(), "[0,255]");
    }

}
