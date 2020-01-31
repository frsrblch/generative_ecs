pub trait Link<A, B> {
    type IdA;
    type IdB;

    fn link(&mut self, a: &Self::IdA, b: &Self::IdB);
}