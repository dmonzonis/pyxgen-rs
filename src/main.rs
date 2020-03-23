mod filename;
mod pixel_map;

fn main() {
    let img = pixel_map::generate_sprite_map().image();
    let filename = filename::random_string(16) + ".png";
    img.save(filename).unwrap();
}
