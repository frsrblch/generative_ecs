use generative_ecs::*;

fn main() {
    let target = std::path::Path::new(r#"C:\Users\Fraser Balch\source\generative_target\src\lib.rs"#);

    let mut spot = Arena::fixed("Spot")
        .add_component(ComponentType::dense("area", "f32"));

    let mut leopard = Arena::fixed("Leopard")
        .add_component(ComponentType::dense("name", "String"));

    leopard.add_ownership(&spot, LinkType::Optional);
    spot.add_reference(&leopard, LinkType::Required);

    let world = World::new()
        .add_arena(spot)
        .add_arena(leopard);

    std::fs::write(target, world.to_string()).ok();
}