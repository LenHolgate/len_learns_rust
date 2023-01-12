use crate::intervals::Intervals;

pub struct IdManager {
    free_ids: Intervals,
}

impl IdManager {
    pub fn new() -> Self {
        let mut manager = IdManager { free_ids: Intervals::new() };

        manager.free_ids.insert_interval(u8::MIN, u8::MAX);

        manager
    }

    pub fn dump(&self) -> String {
        self.free_ids.dump()
    }

    pub fn can_allocate(&self) -> bool {
        !self.free_ids.is_empty()
    }

    pub fn allocate(&mut self) -> u8 {
        if self.free_ids.is_empty()
        {
            panic!("No Ids available")
        }

        self.free_ids.remove_first_value()
    }

    pub fn free(&mut self, id: u8) {
        if !self.free_ids.insert_value(id)
        {
            panic!("id is not currently allocated");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let manager = IdManager::new();

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    fn test_can_allocate() {
        let manager = IdManager::new();

        assert_eq!(manager.can_allocate(), true);
    }

    #[test]
    fn test_allocate() {
        let mut manager = IdManager::new();

        assert_eq!(manager.allocate(), 0);

        assert_eq!(manager.dump(), "[1,255]");
    }

    #[test]
    fn test_allocate_all_ids_and_wrap() {
        let mut manager = IdManager::new();

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
        let mut manager = IdManager::new();

        assert_eq!(manager.allocate(), 0);

        assert_eq!(manager.dump(), "[1,255]");

        manager.free(0);

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    #[should_panic(expected = "id is not currently allocated")]
    fn test_free_id_not_allocated() {
        let mut manager = IdManager::new();

        assert_eq!(manager.dump(), "[0,255]");

        manager.free(0);

        assert_eq!(manager.dump(), "[0,255]");
    }
}
