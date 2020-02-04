use code_gen::{Trait, Generics, TraitFunction};

pub trait Insert<ID, T> {
    fn insert(&mut self, id: &ID, value: T);
}

pub trait Link<A, B> {
    type IdA;
    type IdB;

    fn link(&mut self, a: &Self::IdA, b: &Self::IdB);
}

pub fn get_link_trait() -> Trait {
    Trait::new("Link")
        .with_generics(Generics::two("A", "B"))
        .add_associated_type("IdA")
        .add_associated_type("IdB")
        .add_function_definition(
            TraitFunction::new("link").with_parameters("&mut self, a: &Self::IdA, b: &Self::IdB"))
}

pub trait Create<T> {
    type Id;
    fn create(&mut self, value: T) -> Self::Id;
}