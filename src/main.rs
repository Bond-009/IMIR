#[macro_use]
extern crate clap;
extern crate image;

use clap::{Arg, App};
use image::ImageFormat;
use std::fs::File;
use std::path::Path;

fn main() {
    let matches = App::new("imir")
                          .version(crate_version!())
                          .author(crate_authors!())
                          .about(crate_description!())
                          .arg(Arg::with_name("input file")
                               .short("i")
                               .long("input")
                               .value_name("INPUT")
                               .help("Sets the input file")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("output file")
                               .short("o")
                               .long("output")
                               .value_name("OUTPUT")
                               .help("Sets the output file")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("output format")
                               .short("f")
                               .long("format")
                               .value_name("FORMAT")
                               .help("Sets the output format")
                               .takes_value(true))
                          .get_matches();

    let in_file = Path::new(matches.value_of("input file").unwrap());

    if !in_file.exists() {
        panic!("Input file '{}' not found", in_file.file_name().unwrap().to_str().unwrap());
    }

    let img = image::open(in_file).unwrap();

    let out_file = Path::new(matches.value_of("output file").unwrap());

    let fmt_string = matches.value_of("output format")
                                .unwrap_or(out_file.extension().expect("File has no extension").to_str().unwrap());

    let fmt = match get_image_format(fmt_string) {
        None => panic!("Unsupported format '{}'", fmt_string),
        Some(value) => value
    };

    let _ = img.save(&mut File::create(&out_file).unwrap(), fmt).unwrap();
}

fn get_image_format(input : &str) -> Option<ImageFormat> {
    match input {
        "png" => Some(ImageFormat::PNG),
        "jpg" | "jpeg" => Some(ImageFormat::JPEG),
        "gif" => Some(ImageFormat::GIF),
        "bmp" => Some(ImageFormat::BMP),
        "ico" => Some(ImageFormat::ICO),
        "ppm" => Some(ImageFormat::PPM),
        _ => None
    }
}
