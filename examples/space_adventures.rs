use generative_ecs::*;

fn main() {
    let target = std::path::Path::new(r#"C:\Users\Fraser Balch\source\generative_target\src\lib.rs"#);

    let system = Arena::fixed("System")
        .add_component(ComponentType::dense("name", "String"))
        .add_component(ComponentType::dense_from_type("Position"))
        .add_component(ComponentType::dense("radius", "Length"))
        .add_component(ComponentType::dense_from_type("Temperature"))
        .add_default_component(ComponentType::dense_from_type("Camera"));

    let mut body = Arena::fixed("Body")
        .add_component(ComponentType::sparse("name", "String"))
        .add_component(ComponentType::dense("parameters", "OrbitParameters"))
        .add_component(ComponentType::sparse("parent", "Id<Body>"))
        .add_default_component(ComponentType::dense_from_type("Position"))
        .add_default_component(ComponentType::dense("relative_pos", "Position"));

    let mut surface = Arena::fixed("Surface")
        .add_component(ComponentType::dense_from_type("Area"))
        .add_default_component(ComponentType::dense_from_type("Temperature"));

    let mut atmosphere = Arena::fixed("Atmosphere")
        .add_component(ComponentType::dense("breathability", "bool"))
        .add_component(ComponentType::dense_from_type("GreenhouseRatio"));

    body.add_reference(&system, LinkType::Required);

    body.add_ownership(&surface, LinkType::Optional);
    surface.add_reference(&body, LinkType::Required);

    body.add_ownership(&atmosphere, LinkType::Optional);
    atmosphere.add_reference(&body, LinkType::Required);

    let mut world = World::new()
        .add_use("physics::*")
        .add_static_component(StaticComponent::from_type("Time"))
        .add_static_component(StaticComponent::from_type("Starfield"))
        .add_arena(system)
        .add_arena(body.clone())
        .add_arena(surface.clone())
        .add_arena(atmosphere.clone())
        ;

    let planet = CompoundEntity::new("Planet", &body, vec![&surface, &atmosphere]);

    world.compound_entities.push(planet);

    let r = std::fs::write(target, world.to_string());
    println!("{:?}", r);
}