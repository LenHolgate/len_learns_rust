extern crate idmanager;

use idmanager::thread_safe_id_manager::ThreadSafeIdManager;

pub fn main() {
    let manager = ThreadSafeIdManager::new();

    assert_eq!(manager.dump(), "[0,255]");

    {
        let id = manager.allocate_smart_id();

        let expected_id : u8 = 0;

        assert_eq!(id.value(), &expected_id);

        assert_eq!(manager.dump(), "[1,255]");
    }

    assert_eq!(manager.dump(), "[0,255]");
}
