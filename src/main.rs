extern crate image;

use std::path::Path;
use std::fs::File;

use image::GenericImage;

fn main() {
    // This is a "image::DynamicImage", which I guess wraps
    // the different images that can result in parsing an image
    // file?

    println!("opening lenna image");

    let mut img = image::open(&Path::new("Lenna.png")).unwrap();

    println!("inverting image");

    img.invert();

    println!("converting to luma");

    let imgbuf = img.to_rgb();

    println!("writing out inverted image");

    let ref mut fout = File::create(&Path::new("test-invert.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
