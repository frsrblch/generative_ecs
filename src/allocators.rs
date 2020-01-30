use crate::ids::*;
use bit_set::BitSet;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct FixedAllocator<T> {
    next_index: usize,
    marker: PhantomData<T>,
}

impl<T> Default for FixedAllocator<T> {
    fn default() -> Self {
        Self {
            next_index: 0,
            marker: PhantomData,
        }
    }
}

impl<T> FixedAllocator<T> {
    pub fn create(&mut self) -> Id<T> {
        let id = Id::new(self.next_index);
        self.next_index += 1;
        id
    }
}

#[derive(Debug)]
pub struct GenAllocator<T> {
    ids: Vec<Valid<T>>,
    dead: Vec<usize>,
    living: BitSet,
}

impl<T> Default for GenAllocator<T> {
    fn default() -> Self {
        Self {
            ids: vec![],
            dead: vec![],
            living: BitSet::new(),
        }
    }
}

impl<T> GenAllocator<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create(&mut self) -> &Valid<T> {
        if let Some(index) = self.dead.pop() {
            let i = index as usize;

            let gen = self.ids.get(i).unwrap().id.gen;

            let id = GenId::new(index, gen);
            let id = Valid::new(id);

            self.ids[i] = id;
            self.living.insert(i);

            &self.ids[i]
        } else {
            let i = self.ids.len();
            let gen = Generation::default();

            let id = GenId::new(i, gen);
            let id = Valid::new(id);

            self.ids.push(id);
            self.living.insert(i);

            &self.ids[i]
        }
    }

    pub fn verify(&self, id: GenId<T>) -> Option<&Valid<T>> {
        let index = id.id.index;

        if let Some(current) = self.ids.get(index) {
            if id == current.id {
                return Some(current)
            }
        }

        None
    }

    pub fn is_alive(&self, id: GenId<T>) -> bool {
        let index = id.id.index;
        if let Some(current) = self.ids.get(index) {
            current.id == id
        } else {
            false
        }
    }

    pub fn kill(&mut self, id: GenId<T>) {
        if self.is_alive(id) {
            let id = &mut self.ids[id.id.index];
            id.id.gen = id.id.gen.next();

            self.dead.push(id.index());
            self.living.remove(id.index());
        }
    }
}

impl<T> Clone for GenAllocator<T> {
    fn clone(&self) -> Self {
        Self {
            ids: self.ids.iter().map(|id| Valid::new(id.id)).collect(),
            dead: self.dead.clone(),
            living: self.living.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Generation;

    #[derive(Debug)]
    struct Test;

    #[test]
    fn flex_allocator() {
        let mut allocator = GenAllocator::<Test>::default();

        let id_0_gen_1 = allocator.create().id;
        let id_1_gen_1 = allocator.create().id;

        assert_eq!(id_0_gen_1, GenId::<Test>::new(0, Generation::default()));
        assert_eq!(id_1_gen_1, GenId::<Test>::new(1, Generation::default()));
    }

    #[test]
    fn verify_when_id_is_alive_returns_some() {
        let mut allocator = GenAllocator::<Test>::default();

        let id_0_gen_1 = allocator.create().id;

        assert!(allocator.verify(id_0_gen_1).is_some());
    }

    #[test]
    fn verify_when_id_is_not_alive_returns_none() {
        let mut allocator = GenAllocator::<Test>::default();

        let _id_0_gen_1 = allocator.create().id;

        assert!(allocator.verify(GenId::new(1, Generation::default())).is_none()); //invalid index
        assert!(allocator.verify(GenId::new(0, Generation::default().next())).is_none()); // wrong generation
    }

    #[test]
    fn is_alive_when_id_is_alive_returns_true() {
        let mut allocator = GenAllocator::<Test>::default();

        let id_0_gen_1 = allocator.create().id;

        assert!(allocator.is_alive(id_0_gen_1));
    }

    #[test]
    fn is_alive_when_id_is_not_alive_returns_false() {
        let mut allocator = GenAllocator::<Test>::default();

        let _id_0_gen_1 = allocator.create().id;

        assert!(!allocator.is_alive(GenId::new(1, Generation::default()))); //invalid index
        assert!(!allocator.is_alive(GenId::new(0, Generation::default().next()))); // wrong generation
    }

    #[test]
    fn kill_given_live_entity_is_no_longer_alive() {
        let mut allocator = GenAllocator::<Test>::default();

        let id_0_gen_1 = allocator.create().id;

        allocator.kill(id_0_gen_1);

        assert!(!allocator.is_alive(id_0_gen_1))
    }

    #[test]
    fn create_when_dead_index_returns_reused_index() {
        let mut allocator = GenAllocator::<Test>::default();

        let id_0_gen_1 = allocator.create().id;

        allocator.kill(id_0_gen_1);

        let id_0_gen_2 = allocator.create().id;

        assert_eq!(id_0_gen_2, GenId::new(0, Generation::default().next()));
    }
}