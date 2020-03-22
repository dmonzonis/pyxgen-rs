mod pixel_map;

fn main() {
    let img = pixel_map::generate_sprite_map().image();
    img.save("generated.png").unwrap();
}
