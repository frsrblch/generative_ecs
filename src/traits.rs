use code_gen::{Trait, Generics, TraitFunction};

pub trait Insert<ID, T> {
    fn insert(&mut self, id: &ID, value: T);
}

pub trait Link<A, B> {
    type IdA;
    type IdB;

    fn link(&mut self, a: &Self::IdA, b: &Self::IdB);
}

//impl<A, B, L: Link<A, B>> Link<B, A> for L {
//    type IdA = L::IdB;
//    type IdB = L::IdA;
//
//    fn link(&mut self, a: &Self::IdA, b: &Self::IdB) {
//        Link::<A, B>::link(self, b, a);
//    }
//}

pub fn get_link_trait() -> Trait {
    Trait::new("Link")
        .with_generics(Generics::two("A", "B"))
        .add_associated_type("IdA")
        .add_associated_type("IdB")
        .add_function_definition(
            TraitFunction::new("link").with_parameters("&mut self, a: &Self::IdA, b: &Self::IdB"))
}