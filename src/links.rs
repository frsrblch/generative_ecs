use code_gen::{CamelCase, TraitImplementation, Generics, TypeName, TraitFunction, CodeLine};
use crate::{World, get_link_trait, Arena};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LinkType {
    Required,
    Optional,
//    Many,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Link {
    pub from: CamelCase,
    pub to: CamelCase,
}

impl Link {
    // function only returns some under certain conditions to prevent duplication
    // if from owns to, return the implementation
    // if to owns from, return nothing
    // if they reference each other, return some if from > to, and none if to > from
    pub fn get_implementation(&self, world: &World, link_type: &LinkType) -> Option<TraitImplementation> {
        let from = world.get_arena(&self.from);
        let to = world.get_arena(&self.to);

        if from.owns(to) {
            Some(self.get_implementation_unchecked(world, link_type))
        } else {
            if to.owns(from) {
                return None;
            } else {
                if both_reference_each_other(from, to) {
                    if from.name > to.name {
                        Some(self.get_implementation_unchecked(world, link_type))
                    } else {
                        None
                    }
                } else {
                    Some(self.get_implementation_unchecked(world, link_type))
                }
            }
        }
    }

    fn get_implementation_unchecked(&self, world: &World, link_type: &LinkType) -> TraitImplementation {
        let from = world.get_arena(&self.from);
        let to = world.get_arena(&self.to);
        let link_trait = get_link_trait();

        let mut f = TraitFunction::new("link")
            .with_parameters(&link_trait.functions[0].parameters)
            .add_line(CodeLine::new(0, &format!(
                "self.{}.{}.insert(a, {});",
                from.get_state_field().name,
                to.get_state_field().name,
                match link_type {
                    LinkType::Required => "b.id()",
                    LinkType::Optional => "Some(b.id())",
                }
            )));

        if let Some(link_type) = to.references.get(&from.name) {
            f = f.add_line(CodeLine::new(0, &format!(
                "self.{}.{}.insert(a, {});",
                to.get_state_field().name,
                from.get_state_field().name,
                match link_type {
                    LinkType::Required => "b.id()",
                    LinkType::Optional => "Some(b.id())",
                }
            )));
        }

        let i = get_link_trait()
            .impl_for(&world.get_state())
            .with_generics(Generics::two(from.name.as_str(), to.name.as_str()))
            .add_associated_type(TypeName::new("IdA"), from.get_valid_id_type())
            .add_associated_type(TypeName::new("IdB"), to.get_valid_id_type())
            .add_function(f);
        i
    }
}

fn both_reference_each_other(a: &Arena, b: &Arena) -> bool {
    a.references(b) && b.references(a)
}

impl Link {
    pub fn new(from: &CamelCase, to: &CamelCase) -> Self {
        Link {
            from: from.clone(),
            to: to.clone(),
        }
    }

    pub fn overlaps(&self, rhs: &Link) -> bool {
        (self.from == rhs.from && self.to == rhs.to)
        || (self.from == rhs.to && self.to == rhs.from)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn overlap() {
        let a = CamelCase::from_str("A").unwrap();
        let b = CamelCase::from_str("B").unwrap();
        let c = CamelCase::from_str("C").unwrap();

        let a_b = Link::new(&a, &b);
        let a_c = Link::new(&a, &c);

        let b_a = Link::new(&b, &a);
        let b_c = Link::new(&b, &c);

        let c_a = Link::new(&c, &a);
        let c_b = Link::new(&c, &b);

        assert!(a_b.overlaps(&a_b));
        assert!(a_b.overlaps(&b_a));

        assert!(!a_b.overlaps(&a_c));
        assert!(!a_b.overlaps(&c_a));

        assert!(!a_b.overlaps(&b_c));
        assert!(!a_b.overlaps(&c_b));
    }
}