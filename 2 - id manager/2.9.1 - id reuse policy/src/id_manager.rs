use crate::id_type::IdType;
use crate::intervals::Intervals;
use crate::reuse_policy::ReusePolicy;

pub struct IdManager<T: IdType> {
    free_ids: Intervals<T>,
    reuse_policy: ReusePolicy,
    next_to_allocate: T,
}

impl<T: IdType> IdManager<T> {
    pub fn new(reuse_policy: ReusePolicy) -> Self {
        let mut manager = IdManager::<T> { free_ids: Intervals::<T>::new(), reuse_policy, next_to_allocate : T::MIN };

        manager.free_ids.insert_interval(T::MIN, T::MAX);

        manager
    }

    pub fn dump(&self) -> String {
        self.free_ids.dump()
    }

    pub fn can_allocate(&self) -> bool {
        !self.free_ids.is_empty()
    }

    pub fn allocate(&mut self) -> T {
        if self.free_ids.is_empty()
        {
            panic!("No Ids available")
        }

        if self.reuse_policy == ReusePolicy::ReuseFast
        {
            return self.free_ids.remove_first_value();
        }

        let id: T;

        loop {
            if self.free_ids.remove_value(self.next_to_allocate)
            {
                id = self.next_to_allocate;

                self.next_to_allocate = self.increment_id(self.next_to_allocate);

                break;
            }

            self.next_to_allocate = self.increment_id(self.next_to_allocate);
        }

        id
    }

    fn increment_id(&self, mut id: T) -> T {
        if id == T::MAX
        {
            id = T::MIN;
        } else {
            id = id + T::one();
            //id += T::one();           // needs additional bounds
        }

        id
    }

    pub fn free(&mut self, id: T) {
        if !self.free_ids.insert_value(id)
        {
            panic!("id is not currently allocated");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reuse_policy::ReusePolicy::ReuseFast;
    use crate::reuse_policy::ReusePolicy::ReuseSlow;

    use super::*;

    #[test]
    fn test_new() {
        let manager = IdManager::<u8>::new(ReuseFast);

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    fn test_new_for_all_supported_types() {
        {
            let manager = IdManager::<u8>::new(ReuseFast);

            assert_eq!(manager.dump(), "[0,255]");
        }
        {
            let manager = IdManager::<u16>::new(ReuseFast);

            assert_eq!(manager.dump(), "[0,65535]");
        }
        {
            let manager = IdManager::<u32>::new(ReuseFast);

            assert_eq!(manager.dump(), "[0,4294967295]");
        }
        {
            let manager = IdManager::<u64>::new(ReuseFast);

            assert_eq!(manager.dump(), "[0,18446744073709551615]");
        }
        {
            let manager = IdManager::<u128>::new(ReuseFast);

            assert_eq!(manager.dump(), "[0,340282366920938463463374607431768211455]");
        }
        {
            let manager = IdManager::<usize>::new(ReuseFast);

            assert_eq!(manager.dump(), "[0,18446744073709551615]");
        }
    }

    #[test]
    fn test_can_allocate() {
        let manager = IdManager::<u8>::new(ReuseFast);

        assert_eq!(manager.can_allocate(), true);
    }

    #[test]
    fn test_allocate() {
        let mut manager = IdManager::<u8>::new(ReuseFast);

        assert_eq!(manager.allocate(), 0);

        assert_eq!(manager.dump(), "[1,255]");
    }

    #[test]
    fn test_allocate_all_ids_and_wrap() {
        let mut manager = IdManager::<u8>::new(ReuseSlow);

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
        let mut manager = IdManager::<u8>::new(ReuseFast);

        assert_eq!(manager.allocate(), 0);

        assert_eq!(manager.dump(), "[1,255]");

        manager.free(0);

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    #[should_panic(expected = "id is not currently allocated")]
    fn test_free_id_not_allocated() {
        let mut manager = IdManager::<u8>::new(ReuseFast);

        assert_eq!(manager.dump(), "[0,255]");

        manager.free(0);

        assert_eq!(manager.dump(), "[0,255]");
    }

    #[test]
    fn test_reuse_fast() {
        let mut manager = IdManager::<u8>::new(ReuseFast);

        for i in 0..10 {
            assert_eq!(manager.allocate(), i);
        }

        assert_eq!(manager.dump(), "[10,255]");

        manager.free(2);
        manager.free(6);
        manager.free(7);
        manager.free(4);

        assert_eq!(manager.dump(), "[2], [4], [6,7], [10,255]");

        assert_eq!(manager.allocate(), 2);
        assert_eq!(manager.allocate(), 4);
        assert_eq!(manager.allocate(), 6);
        assert_eq!(manager.allocate(), 7);
        assert_eq!(manager.allocate(), 10);

        assert_eq!(manager.dump(), "[11,255]");
    }

    #[test]
    fn test_reuse_slow() {
        let mut manager = IdManager::<u8>::new(ReuseSlow);

        for i in 0..10 {
            assert_eq!(manager.allocate(), i);
        }

        assert_eq!(manager.dump(), "[10,255]");

        manager.free(2);
        manager.free(6);
        manager.free(7);
        manager.free(4);

        assert_eq!(manager.dump(), "[2], [4], [6,7], [10,255]");

        for i in 10..255 {
            assert_eq!(manager.allocate(), i);
        }

        assert_eq!(manager.dump(), "[2], [4], [6,7], [255]");

        assert_eq!(manager.allocate(), 255);

        assert_eq!(manager.allocate(), 2);
        assert_eq!(manager.allocate(), 4);
        assert_eq!(manager.allocate(), 6);
        assert_eq!(manager.allocate(), 7);

        assert_eq!(manager.dump(), "");
    }
}
