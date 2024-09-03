use clap::Parser;
use image::GenericImageView;
use std::{
    fs::{self, File},
    io::BufReader,
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

    let file = File::open(&args.file).expect("Could not open file");
    let reader = BufReader::new(file);

    let img = image::open(reader).expect("Could not read image");

    let dimensions = img.dimensions();
    let file_size = fs::metadata(&args.file).expect("Could not open file");
}
