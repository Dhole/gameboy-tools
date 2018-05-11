extern crate image;

extern crate gameboy_tools;

use image::GenericImage;
use gameboy_tools::*;
use std::process;
use std::env;
use std::path::Path;

// Reads an image in png format from a file, divides it into 8x8 subimages and outputs the
// corresponding gameboy tile data in C array format.
fn main() {
    let filename = match env::args().nth(1) {
        None => {
            eprintln!("Usage: {:} image_file", env::args().nth(0).unwrap());
            process::exit(1);
        }
        Some(filename) => filename,
    };
    let img = match image::open(&Path::new(&filename)) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Error opening image at file {:?}: {:?}", filename, err);
            process::exit(1);
        }
    };
    let img = img.grayscale();
    let mut img = img.to_luma();

    if (img.width() % 8 != 0) || (img.height() % 8 != 0) {
        eprintln!(
            "Image dimensions are not multiple of 8: {:?}x{:?}",
            img.width(),
            img.height()
        );
        process::exit(1);
    }

    let mut gbtiles = Vec::<Vec<u8>>::new();
    for row in 0..img.height() / 8 {
        for col in 0..img.width() / 8 {
            let gbtile = img8x8_to_gbtile(img.sub_image(col * 8, row * 8, 8, 8));
            gbtiles.push(gbtile);
        }
    }

    println!(
        "const UINT8 {:}_tiles[] = {{",
        Path::new(&filename).file_stem().unwrap().to_str().unwrap()
    );
    for gbtile in gbtiles {
        print!("    ");
        for b in gbtile {
            print!("0x{:02x},", b);
        }
        println!();
    }
    println!("}};");
}

