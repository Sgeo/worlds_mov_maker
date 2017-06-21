extern crate image;

fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided!");
    let image = image::open(&filename).expect("Unable to open image!");
}
