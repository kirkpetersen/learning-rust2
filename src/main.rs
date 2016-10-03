extern crate image;

use std::path::Path;

use image::GenericImage;

fn main() {
    let im = image::open(&Path::new("Lenna.png")).unwrap();

    println!("dimensions: {:?}", im.dimensions())
}
