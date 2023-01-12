extern crate idmanager;

use idmanager::IdManager;

pub fn main() {
    let manager = IdManager::<u8>::new();

    assert_eq!(manager.dump(), "[0,255]");

    {
        let id = manager.allocate_id();

        let expected_id : u8 = 0;

        assert_eq!(id.value(), &expected_id);

        assert_eq!(manager.dump(), "[1,255]");
    }

    assert_eq!(manager.dump(), "[0,255]");
}
