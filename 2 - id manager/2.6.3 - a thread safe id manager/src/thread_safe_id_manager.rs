use std::sync::{Mutex, MutexGuard};

use crate::id_manager::IdManager;
use crate::smart_id::SmartId;

pub struct ThreadSafeIdManager {
    manager : Mutex<IdManager>
}

impl ThreadSafeIdManager {
    pub fn new() -> Self {
        let manager = Mutex::new(IdManager::new());

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

    pub fn allocate(&self) -> u8 {
        let mut locked = self.lock();

        locked.allocate()
    }

    pub fn allocate_smart_id(&self) -> SmartId {
        SmartId::new(&self.manager)
    }

    pub fn free(&self, id: u8) {
        let mut locked = self.lock();

        locked.free(id)
    }

    fn lock(&self) -> MutexGuard<IdManager> {
        self.manager.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _manager = ThreadSafeIdManager::new();
    }

    #[test]
    fn test_can_allocate() {
        let manager = ThreadSafeIdManager::new();

        assert_eq!(manager.can_allocate(), true);
    }

    #[test]
    fn test_allocate() {
        let manager = ThreadSafeIdManager::new();

        assert_eq!(manager.allocate(), 0);

        assert_eq!(manager.dump(), "[1,255]");
    }

    #[test]
    fn test_allocate_all_ids_and_wrap() {
        let manager = ThreadSafeIdManager::new();

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
        let manager = ThreadSafeIdManager::new();

        assert_eq!(manager.allocate(), 0);

        assert_eq!(manager.dump(), "[1,255]");

        manager.free(0);

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    #[should_panic(expected = "id is not currently allocated")]
    fn test_free_id_not_allocated() {
        let manager = ThreadSafeIdManager::new();

        assert_eq!(manager.dump(), "[0,255]");

        manager.free(0);

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    fn test_create_one_smart_id() {
        let manager = ThreadSafeIdManager::new();

        {
            let _ids = manager.allocate_smart_id();

            assert_eq!(manager.dump(), "[1,255]");
        }

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    fn test_create_multiple_smart_ids() {
        let manager = ThreadSafeIdManager::new();

        assert_eq!(manager.dump(), "[0,255]");

        {
            let id1 = manager.allocate_smart_id();

            let expected_id1: u8 = 0;

            assert_eq!(id1.value(), &expected_id1);

            assert_eq!(manager.dump(), "[1,255]");

            {
                let mut id2 = manager.allocate_smart_id();

                let expected_id2: u8 = 1;

                assert_eq!(id2.value(), &expected_id2);

                assert_eq!(manager.dump(), "[2,255]");

                id2.release();

                {
                    let id3 = manager.allocate_smart_id();

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
