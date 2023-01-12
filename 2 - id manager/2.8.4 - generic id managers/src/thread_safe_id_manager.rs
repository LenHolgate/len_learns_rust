use std::sync::{Mutex, MutexGuard};

use crate::id_manager::IdManager;
use crate::id_type::IdType;
use crate::smart_id::SmartId;

pub struct ThreadSafeIdManager<T: IdType> {
    manager: Mutex<IdManager<T>>,
}

impl<T: IdType> ThreadSafeIdManager<T> {
    pub fn new() -> Self {
        let manager = Mutex::new(IdManager::<T>::new());

        ThreadSafeIdManager { manager }
    }

    pub fn dump(&self) -> String {
        let locked = self.lock();

        locked.dump()
    }

    pub fn can_allocate(&self) -> bool {
        let locked = self.lock();

        locked.can_allocate()
    }

    fn allocate(&self) -> T {
        let mut locked = self.lock();

        locked.allocate()
    }

    pub fn allocate_id(&self) -> SmartId<T> {
        SmartId::new(&self.manager)
    }

    fn free(&self, id: T) {
        let mut locked = self.lock();

        locked.free(id)
    }

    fn lock(&self) -> MutexGuard<IdManager<T>> {
        self.manager.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _manager = ThreadSafeIdManager::<u8>::new();
    }

    #[test]
    fn test_new_for_all_supported_types() {
        {
            let manager = ThreadSafeIdManager::<u8>::new();

            assert_eq!(manager.dump(), "[0,255]");
        }
        {
            let manager = ThreadSafeIdManager::<u16>::new();

            assert_eq!(manager.dump(), "[0,65535]");
        }
        {
            let manager = ThreadSafeIdManager::<u32>::new();

            assert_eq!(manager.dump(), "[0,4294967295]");
        }
        {
            let manager = ThreadSafeIdManager::<u64>::new();

            assert_eq!(manager.dump(), "[0,18446744073709551615]");
        }
        {
            let manager = ThreadSafeIdManager::<u128>::new();

            assert_eq!(manager.dump(), "[0,340282366920938463463374607431768211455]");
        }
        {
            let manager = ThreadSafeIdManager::<usize>::new();

            assert_eq!(manager.dump(), "[0,18446744073709551615]");
        }
    }

    #[test]
    fn test_can_allocate() {
        let manager = ThreadSafeIdManager::<u8>::new();

        assert_eq!(manager.can_allocate(), true);
    }

    #[test]
    fn test_allocate() {
        let manager = ThreadSafeIdManager::<u8>::new();

        assert_eq!(manager.allocate(), 0);

        assert_eq!(manager.dump(), "[1,255]");
    }

    #[test]
    fn test_allocate_all_ids_and_wrap() {
        let manager = ThreadSafeIdManager::<u8>::new();

        for i in 0..u8::MAX {
            assert_eq!(manager.allocate(), i);
        }
        assert_eq!(manager.dump(), "[255]");

        assert_eq!(manager.allocate(), 255);

        assert_eq!(manager.can_allocate(), false);

        assert_eq!(manager.dump(), "");

        for i in 0..10 {
            manager.free(i);
        }

        assert_eq!(manager.dump(), "[0,9]");

        for i in 0..10 {
            assert_eq!(manager.allocate(), i);
        }
        assert_eq!(manager.dump(), "");
    }

    #[test]
    fn test_free() {
        let manager = ThreadSafeIdManager::<u8>::new();

        assert_eq!(manager.allocate(), 0);

        assert_eq!(manager.dump(), "[1,255]");

        manager.free(0);

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    #[should_panic(expected = "id is not currently allocated")]
    fn test_free_id_not_allocated() {
        let manager = ThreadSafeIdManager::<u8>::new();

        assert_eq!(manager.dump(), "[0,255]");

        manager.free(0);

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    fn test_create_one_smart_id() {
        let manager = ThreadSafeIdManager::<u8>::new();

        {
            let _id = manager.allocate_id();

            assert_eq!(manager.dump(), "[1,255]");
        }

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    fn test_create_multiple_smart_ids() {
        let manager = ThreadSafeIdManager::<u8>::new();

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
}
