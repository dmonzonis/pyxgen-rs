mod filename;
mod pixel_map;

use clap::{value_t, values_t, App, Arg, ArgMatches};

const GREEN: [u8; 3] = [0, 255, 0];
const DARK_GREEN: [u8; 3] = [0, 180, 0];
const WHITE: [u8; 3] = [255, 255, 255];

fn convert_arg_to_rgb(matches: &ArgMatches, arg: &str, default: &[u8; 3]) -> [u8; 3] {
    // Will panic if color does not have at least length 3
    match values_t!(matches, arg, u8) {
        Ok(rgb) => [rgb[0], rgb[1], rgb[2]],
        Err(_) => [default[0], default[1], default[2]],
    }
}

fn main() {
    // Set up argument parser
    let matches = App::new("pyxgen-rs")
        .arg(
            Arg::with_name("color")
                .short("c")
                .value_names(&["R", "G", "B"])
                .help("Main color of the sprite")
                .number_of_values(3),
        )
        .arg(
            Arg::with_name("outline")
                .short("o")
                .value_names(&["R", "G", "B"])
                .help("Color of the sprite's outline")
                .number_of_values(3),
        )
        .arg(
            Arg::with_name("background")
                .short("b")
                .value_names(&["R", "G", "B"])
                .help("Color of the background, if not transparent")
                .number_of_values(3),
        )
        .arg(
            Arg::with_name("transparency")
                .short("t")
                .help("Use transparent background"),
        )
        .get_matches();

    // Retrieve info from arguments
    let color = convert_arg_to_rgb(&matches, "color", &GREEN);
    // TODO: if outline is not set, use a darker version of color
    let outline = convert_arg_to_rgb(&matches, "outline", &DARK_GREEN);
    let background = convert_arg_to_rgb(&matches, "background", &WHITE);
    let transparency = value_t!(matches, "transparency", bool).unwrap_or(false);

    let img = pixel_map::generate_sprite_map().image(color, outline, background, transparency);
    let filename = filename::random_string(16) + ".png";
    img.save(filename).unwrap();
}
