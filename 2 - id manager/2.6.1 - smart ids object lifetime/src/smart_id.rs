use crate::id_manager::IdManager;

pub struct SmartId<'a> {
    manager: &'a mut IdManager,
    id: u8,
    we_own_id: bool,
}

impl<'a> SmartId<'a> {
    pub fn new(manager: &'a mut IdManager) -> Self {

        if !manager.can_allocate()
        {
            panic!("No Ids available")
        }

        let id = manager.allocate();

        SmartId { manager, id, we_own_id: true }
    }
    pub fn release(&mut self) -> u8 {

        self.we_own_id = false;

        self.id
    }

    pub fn value(&self) -> &u8 {
        &self.id
    }
}

impl<'a> Drop for SmartId<'a> {
    fn drop(&mut self) {

        if self.we_own_id
        {
            self.manager.free(self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_one_smart_id() {
        let mut manager = IdManager::new();

        assert_eq!(manager.dump(), "[0,255]");

        {
            let id1 = SmartId::new(&mut manager);

            let expected_id: u8 = 0;

            assert_eq!(id1.value(), &expected_id);

            //assert_eq!(manager.dump(), "[1,255]");
        }

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    fn test_create_multiple_smart_ids() {
        let mut manager = IdManager::new();

        assert_eq!(manager.dump(), "[0,255]");

        {
            let id1 = SmartId::new(&mut manager);

            let expected_id1: u8 = 0;

            assert_eq!(id1.value(), &expected_id1);

            //assert_eq!(manager.dump(), "[1,255]");

            {
                let mut id2 = SmartId::new(&mut manager);

                let expected_id2: u8 = 1;

                assert_eq!(id2.value(), &expected_id2);

                //assert_eq!(manager.dump(), "[2,255]");

                id2.release();

                {
                    let id3 = SmartId::new(&mut manager);

                    let expected_id: u8 = 2;

                    assert_eq!(id3.value(), &expected_id);

                    //assert_eq!(manager.dump(), "[3,255]");
                }
            }

            //assert_eq!(manager.dump(), "[2,255]");
        }

        //assert_eq!(manager.dump(), "[0], [2,255]");
    }

    #[test]
    fn test_release() {
        let mut manager = IdManager::new();

        assert_eq!(manager.dump(), "[0,255]");

        {
            let mut id1 = SmartId::new(&mut manager);

            //assert_eq!(manager.dump(), "[1,255]");

            id1.release();

            //assert_eq!(manager.dump(), "[1,255]");
        }

        assert_eq!(manager.dump(), "[1,255]");
    }
}
