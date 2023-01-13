extern crate idmanager;

use idmanager::IdManager;
use idmanager::ReusePolicy::ReuseSlow;

pub fn main() {
    let manager = IdManager::<u8>::new(ReuseSlow);

    assert_eq!(manager.dump(), "[0,255]");

    {
        let id1 = manager.allocate_id();

        let expected_id1: u8 = 0;

        assert_eq!(id1.value(), &expected_id1);

        assert_eq!(manager.dump(), "[1,255]");

        {
            let mut id2 = manager.allocate_id();

            let expected_id2: u8 = 1;

            assert_eq!(id2.value(), &expected_id2);

            assert_eq!(manager.dump(), "[2,255]");

            id2.release();

            {
                let id3 = manager.allocate_id();

                let expected_id: u8 = 2;

                assert_eq!(id3.value(), &expected_id);

                assert_eq!(manager.dump(), "[3,255]");
            }
        }

        assert_eq!(manager.dump(), "[2,255]");
    }

    assert_eq!(manager.dump(), "[0], [2,255]");
}
