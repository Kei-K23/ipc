use clap::Parser;
use exif::Reader;
use image::GenericImageView;
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

/*
Simple CLI to extract metadata from image file
*/

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    // let file = File::open(&args.file).expect("Could not open file");
    let path = Path::new(&args.file);

    let img = image::open(path).expect("Could not read image");

    let dimensions = img.dimensions();
    let file_size = fs::metadata(&args.file).expect("Could not open file").len();

    println!("File: {}", args.file);
    println!("Dimensions: {}x{}", dimensions.0, dimensions.1);
    println!("File size: {}", file_size);

    // Extract EXIF data
    let file = File::open(&args.file).expect("Could not open file");
    let exif_reader = Reader::new();
    let exif = exif_reader
        .read_from_container(&mut BufReader::new(file))
        .expect("Could not read EXIF data");

    for field in exif.fields() {
        match field.value {
            exif::Value::Ascii(ref v) => println!("{}: {:?}", field.tag, v),
            exif::Value::Short(ref v) => println!("{}: {:?}", field.tag, v),
            exif::Value::Long(ref v) => println!("{}: {:?}", field.tag, v),
            exif::Value::Rational(ref v) => println!("{}: {:?}", field.tag, v),
            _ => continue,
        }
    }
}
