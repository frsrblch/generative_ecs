pub trait Link<A, B> {
    type IdA;
    type IdB;

    fn link(&mut self, a: &Self::IdA, b: &Self::IdB);
}

pub trait Insert<ID, T> {
    fn insert(&mut self, id: &ID, value: T);
}