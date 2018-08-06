// extern crate image;
// use image::GenericImage;
// use std::env;
// use std::path::Path;
// use std::fs::File;
// use image::png::PNGEncoder;

extern crate clap;
use clap::{Arg, App};

fn main() {

    let matches = App::new("Atlas Builder")
                            .version("0.0.1")
                            .about("Creates a spritesheet from provided images.")
                            .arg(Arg::with_name("max_size")
                                .short("m")
                                .long("max_size")
                                .value_name("MAX_SIZE")
                                .help("Specify max size of spritesheet.")
                                .takes_value(true))
                            .arg(Arg::with_name("path")
                                .short("i")
                                .long("path")
                                .help("Path to directory with images.")
                                .required(true)
                                .takes_value(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .long("output")
                                .help("Output file")
                                .takes_value(true))
                            .get_matches();

    let output = matches.value_of("output").unwrap_or("output.png");
    println!("Output: {}", output);

    let path = matches.value_of("path").unwrap();
    println!("Path: {}", path);

    let max = matches.value_of("max_size").unwrap_or("not specified");
    println!("Max size: {}", max);

    // let args: Vec<_> = env::args().collect();
    // let path_1 = Path::new(&args[1]);
    // let path_2 = Path::new(&args[2]);
    // let path_3 = Path::new(&args[3]);

    // let image_1 = image::open(path_1).unwrap();
    // let image_2 = image::open(path_2).unwrap();

    // let mut buf_1 = image_1.raw_pixels();
    // let mut buf_2 = image_2.raw_pixels();

    // let file = File::create(path_3).unwrap();
    // let encoder = PNGEncoder::new(file);

    // buf_1.append(&mut buf_2);

    // encoder.encode(&buf_1, image_1.width(), image_1.height() + image_2.height(), image::ColorType::RGBA(8)).unwrap();
}
