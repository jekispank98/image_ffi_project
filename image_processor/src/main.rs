use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use image::ImageReader;
use crate::args::Args;

pub mod args;

fn main() {
    let args = Args::parse();
    let file_path = args.input;
    if check_is_image_exist(&file_path) {
        let img = ImageReader::open(&file_path).expect("Couldn't open the image").decode();
    }
}

fn check_is_image_exist(path: &String) -> bool {
    let path_buf = PathBuf::from(path);
    path_buf.exists()
}