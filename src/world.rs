use crate::*;
use code_gen::*;
use std::fmt::{Display, Formatter, Error};
use std::collections::HashMap;

// Think about a better way to program the World.
// Allocators and Arenas only make sense if one is already familiar with ECS.
// Instead, compose types and build a world from them, similar to a more object-oriented approach.

#[derive(Debug, Default)]
pub struct World {
    pub uses: Vec<String>,
    pub arenas: Vec<Arena>,
    pub components: Vec<StaticComponent>,
    pub links: HashMap<links::Link, LinkType>,
    pub compound_entities: Vec<CompoundEntity>,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.validate();

        for s in self.uses.iter() {
            writeln!(f, "use {};", s).ok();
        }
        writeln!(f).ok();

        writeln!(f, "{}", self.get_world()).ok();
        writeln!(f, "{}", self.impl_world()).ok();
        writeln!(f, "{}", self.get_allocators()).ok();
        writeln!(f, "{}", self.get_state()).ok();

        for arena in self.arenas.iter() {
            writeln!(f, "{}", arena.get_struct(&self)).ok();
            writeln!(f, "{}", arena.get_impl()).ok();
            writeln!(f, "{}", arena.get_data_row()).ok();
        }

        for link_impl in self.get_link_implementations() {
            writeln!(f, "{}", link_impl).ok();
        }

        for compound in self.compound_entities.iter() {

        }

        Ok(())
    }
}

impl World {
    pub fn new() -> Self {
        World {
            uses: vec!["generative_ecs::prelude::*".to_string()],
            arenas: vec![],
            components: vec![],
            links: Default::default(),
            compound_entities: vec![],
        }
    }

    pub fn add_use(mut self, use_ref: &str) -> Self {
        self.uses.push(use_ref.to_string());
        self
    }

    pub fn add_arena(mut self, arena: Arena) -> Self {
        self.arenas.push(arena);
        self.update_links();
        self
    }

    pub fn add_static_component(mut self, component: StaticComponent) -> Self {
        self.components.push(component);
        self
    }

    pub fn get_world(&self) -> Struct {
        Struct::new("World")
            .with_derives(Derives::with_debug_default_clone())
            .add_field(Field::from_type(Type::new("Allocators")))
            .add_field(Field::from_type(Type::new("State")))
    }

    pub fn impl_world(&self) -> Impl {
        Impl::from(&self.get_world().typ)
            .add_function(self.get_split())
    }

    fn get_split(&self) -> Function {
        let fields = self.get_world().fields;

        let return_type: Vec<String> = fields.iter().cloned().map(|f| f.field_type.to_string()).collect();
        let return_fields: Vec<SnakeCase> = fields.iter().cloned().map(|f| f.name).collect();

        let return_type = StrConcat {
            iter: return_type,
            left_bound: "(",
            right_bound: ")",
            item_prepend: "&mut ",
            item_append: "",
            join: ", "
        }.to_string();

        let code = StrConcat {
            iter: return_fields,
            left_bound: "(",
            right_bound: ")",
            item_prepend: "&mut self.",
            item_append: "",
            join: ", "
        }.to_string();

        Function::new("split")
            .with_parameters("&mut self")
            .with_return(return_type)
            .add_line(CodeLine::new(0, &code))
    }

    pub fn get_allocators(&self) -> Struct {
        let fields = self.arenas.iter()
            .map(Arena::get_allocator_field)
            .collect();

        Struct::new("Allocators")
            .with_derives(Derives::with_debug_default_clone())
            .with_fields(fields)
    }

    pub fn get_state(&self) -> Struct {
        let static_fields = self.components.iter().map(StaticComponent::get_field);
        let arena_fields = self.arenas.iter().map(Arena::get_state_field);

        let fields = static_fields
            .chain(arena_fields)
            .collect();

        Struct::new("State")
            .with_derives(Derives::with_debug_default_clone())
            .with_fields(fields)
    }

    fn validate(&self) {
        assert!(self.no_transient_owns_permanent());
        assert!(self.no_permanent_has_mandatory_link_to_transient());
        assert!(self.no_transient_has_mandatory_reference_to_non_owner_transient());
    }

    fn no_transient_owns_permanent(&self) -> bool {
        let owns_permanent = |arena: &Arena| {
            arena.ownership.keys()
                .map(|k| self.get_arena(k))
                .any(|owned| owned.allocator == Allocator::Fixed)
        };

        !self.transient_entities().any(owns_permanent)
    }

    fn no_permanent_has_mandatory_link_to_transient(&self) -> bool {
        let mandatory_link_to_transient = |arena: &Arena| {
            arena.ownership.iter()
                .chain(arena.references.iter())
                .map(|(name, link)| (self.get_arena(name), link))
                .any(|(owned, link)| {
                    owned.allocator == Allocator::Generational && *link == LinkType::Required
                })
        };

        !self.permanent_entities().any(mandatory_link_to_transient)
    }

    fn no_transient_has_mandatory_reference_to_non_owner_transient(&self) -> bool {
        let mandatory_reference_to_transient = |arena: &Arena| {
            arena.references.iter()
                .map(|(name, link)| (self.get_arena(name), link))
                .any(|(reference, link)| {
                    reference.allocator == Allocator::Generational
                        && *link == LinkType::Required
                        && !reference.owns(arena)
                })
        };

        !self.transient_entities().any(mandatory_reference_to_transient)
    }

    pub fn get_arena(&self, name: &CamelCase) -> &Arena {
        self.arenas.iter()
            .find(|a| a.name.eq(name))
            .unwrap_or_else(|| panic!("Expected arena not found in World: {}", name))
    }

    fn permanent_entities(&self) -> impl Iterator<Item=&Arena> {
        self.arenas.iter()
            .filter(|arena| arena.allocator == Allocator::Fixed)
    }

    fn transient_entities(&self) -> impl Iterator<Item=&Arena> {
        self.arenas.iter()
            .filter(|arena| arena.allocator == Allocator::Generational)
    }

    fn update_links(&mut self) {
        self.links = self.get_links();
    }

    fn get_links(&self) -> HashMap<links::Link, LinkType> {
        let mut map = HashMap::new();

        for arena in self.arenas.iter() {
            for (owned, link_type) in arena.ownership.iter() {
                let link = links::Link::new(&arena.name, owned);
                map.insert(link, *link_type);
            }

            for (reference, link_type) in arena.references.iter() {
                let link = links::Link::new(&arena.name, reference);
                map.insert(link, *link_type);
            }
        }

        map
    }

    pub fn get_link_implementations(&self) -> Vec<TraitImplementation> {
        self.links.iter()
            .filter_map(|(link, link_type)| {
                link.get_implementation(self, link_type)
            })
            .collect()

//        for (link, link_type) in self.links.iter() {
//
//        }
//
//
//        let mut matched_sets: Vec<links::Link> = vec![];
//        let mut implementations: Vec<TraitImplementation> = vec![];
//
//        let link_trait = crate::traits::get_link_trait();
//        let trait_function = &link_trait.functions[0];
//
//        let state = self.get_state();
//
//
//        for (link, link_type) in self.links.iter() {
//
//            let from_id = self.get_arena(&link.from).get_valid_id_type();
//            let to_id = self.get_arena(&link.to).get_valid_id_type();
//
//            if matched_sets.iter().any(|l| l.overlaps(link)) {
//                let implementation = implementations.iter_mut().find(|l| l.overlaps(link)).unwrap();
//                                                                                //self.from.to.insert(
//                implementation.functions[0].lines.push(CodeLine::new(0, &format!("self.{}.{}")));
//            } else {
//                let generics = Generics::two(link.from.as_str(), link.to.as_str());
//
//                let mut implementation = link_trait
//                    .impl_for(&state)
//                    .with_generics(generics)
//                    .add_associated_type(TypeName::new("IdA"), from_id)
//                    .add_associated_type(TypeName::new("IdB"), to_id);
//
//                let mut link_function = TraitFunction::new("link")
//                    .with_parameters(&trait_function.parameters);
//
//
//                implementation = implementation.add_function(link_function);
//
//                implementations.push(implementation);
//                matched_sets.push(link.clone());
//            }
//        }
//
//        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //	Transient	Permanent	Owns	    INVALID, child entity will leak if parent removed	-
    //	Transient	Permanent	Maybe Owns	INVALID, child entity will leak if parent removed	-
    #[test]
    #[should_panic]
    fn invalid_transient_owning_permanent() {
        let perm = Arena::fixed("Perm");

        let mut temp = Arena::generational("Temp");
        temp.add_ownership(&perm, LinkType::Required);

        let invalid = World::new()
            .add_arena(perm)
            .add_arena(temp);

        invalid.validate();
    }

    //	Permanent	Transient	Owns	    INVALID, no reason for child to be temp if it cannot unlink
    #[test]
    #[should_panic]
    fn invalid_permanent_cannot_mandatory_own_transient() {
        let temp = Arena::generational("Temp");

        let mut perm = Arena::fixed("Perm");
        perm.add_ownership(&temp, LinkType::Required);

        let invalid = World::new()
            .add_arena(perm)
            .add_arena(temp);

        invalid.validate();
    }

    //	Permanent	Transient	Ref	        INVALID, cannot be unlinked if child removed	    -
    #[test]
    #[should_panic]
    fn invalid_permanent_cannot_mandatory_refer_to_transient() {
        let temp = Arena::generational("Temp");

        let mut perm = Arena::fixed("Perm");
        perm.add_reference(&temp, LinkType::Required);

        let invalid = World::new()
            .add_arena(perm)
            .add_arena(temp);

        invalid.validate();
    }

    //	Transient	Permanent	Owns	    INVALID, child entity will leak if parent removed	-
    #[test]
    #[should_panic]
    fn invalid_transient_cannot_mandatory_own_permanent() {
        let perm = Arena::fixed("Perm");

        let mut temp = Arena::generational("Temp");
        temp.add_ownership(&perm, LinkType::Required);

        let invalid = World::new()
            .add_arena(perm)
            .add_arena(temp);

        invalid.validate();
    }

    //	Transient	Permanent	Maybe Owns	INVALID, child entity will leak if parent removed	-
    #[test]
    #[should_panic]
    fn invalid_transient_cannot_optionally_own_permanent() {
        let perm = Arena::fixed("Perm");

        let mut temp = Arena::generational("Temp");
        temp.add_ownership(&perm, LinkType::Optional);

        let invalid = World::new()
            .add_arena(perm)
            .add_arena(temp);

        invalid.validate();
    }

    //	Transient	Transient	Ref	        INVALID, cannot be unlinked if child removed	    must point at owner, so refer is deleted along with it
    #[test]
    #[should_panic]
    fn invalid_transient_cannot_have_mandatory_reference_to_transient() {
        let temp1 = Arena::generational("Temp1");

        let mut temp2 = Arena::generational("Temp2");
        temp2.add_reference(&temp1, LinkType::Required);

        let invalid = World::new()
            .add_arena(temp1)
            .add_arena(temp2);

        invalid.validate();
    }

    //	Transient	Transient	Ref	        MAYBE INVALID	    must point at owner, so refer is deleted along with it
    #[test]
    fn invalid_transient_can_have_mandatory_reference_to_transient_owner() {
        let mut owner = Arena::generational("Temp1");
        let mut owned = Arena::generational("Temp2");

        owner.add_ownership(&owned, LinkType::Required);
        owned.add_reference(&owner, LinkType::Required);

        let invalid = World::new()
            .add_arena(owner)
            .add_arena(owned);

        invalid.validate();
    }
}