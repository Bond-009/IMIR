#[macro_use]
extern crate clap;
extern crate image;

use clap::{Arg, App};
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
                               .takes_value(true)
                               .default_value("png")
                               .required(false))
                          .get_matches();

    let in_file_name = matches.value_of("input file").unwrap();

    let in_file = Path::new(in_file_name);

    if !in_file.exists() {
        panic!("Input file '{}' not found", in_file_name);
    }

    let img = image::open(&in_file).unwrap();

    let out_file_name = matches.value_of("output file").unwrap();

    let ref mut out_file = File::create(&Path::new(out_file_name)).unwrap();

    let fmt_string = matches.value_of("output format").unwrap();

    let fmt = match fmt_string {
        "png" => image::PNG,
        "jpeg" => image::JPEG,
        "gif" => image::GIF,
        "bmp" => image::BMP,
        "ico" => image::ICO,
        "ppm" => image::PPM,
        _ => panic!("Unknown format '{}'", fmt_string)
    };

    let _ = img.save(out_file, fmt).unwrap();
}
