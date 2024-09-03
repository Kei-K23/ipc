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
}
