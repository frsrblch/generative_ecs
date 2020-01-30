use generative_ecs::*;

fn main() {
    let target = std::path::Path::new(r#"C:\Users\Fraser Balch\source\generative_target\src\lib.rs"#);

    let spot = Arena::fixed("Spot")
        .add_component(ComponentType::dense_from_type("Color"));

    let mut leopard = Arena::fixed("Leopard")
        .add_component(ComponentType::dense("name", "String"));
    leopard.add_ownership(&spot, LinkType::Many);

    let world = World::new()
        .add_arena(spot)
        .add_arena(leopard);

    std::fs::write(target, world.to_string()).ok();
}