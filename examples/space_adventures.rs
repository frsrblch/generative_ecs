use generative_ecs::*;

fn main() {
    let target = std::path::Path::new(r#"C:\Users\Fraser Balch\source\generative_target\src\lib.rs"#);

    let system = Arena::fixed("System")
        .add_component(Component::dense("name", "String"))
        .add_component(Component::dense_from_type("Position"))
        .add_component(Component::dense("radius", "Length"))
        .add_component(Component::dense_from_type("Temperature"))
        .add_default_component(Component::dense_from_type("Camera"));

    let body = Arena::fixed("Body")
        .add_component(Component::sparse("name", "String"))
        .add_component(Component::dense("parameters", "OrbitParameters"))
        .add_component(Component::sparse("parent", "Id<Body>"))
        .add_default_component(Component::dense_from_type("Position"))
        .add_default_component(Component::dense("relative_pos", "Position"));

    let surface = Arena::fixed("Surface")
        .add_component(Component::dense_from_type("Area"))
        .add_default_component(Component::dense_from_type("Temperature"));

//        let atmosphere = Arena::fixed("Atmosphere")
//            .add_component(Component::dense("breathability", "bool"))
//            .add_component(Component::dense_from_type("GreenhouseRatio"));

    let world = World::new()
        .add_static_component(StaticComponent::from_type("Time"))
        .add_static_component(StaticComponent::from_type("Starfield"))
        .add_arena(system)
        .add_arena(body)
        .add_arena(surface)
//            .add_arena(atmosphere)
        ;

    let r = std::fs::write(target, world.to_string());
    println!("{:?}", r);
}