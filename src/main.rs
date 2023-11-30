extern crate flate2;

use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};

fn compress(source_file: &str, target_file: &str) {
    let mut input = BufReader::new(File::open(source_file).unwrap());
    let output = BufWriter::new(File::create(target_file).unwrap());
    let mut encoder = GzEncoder::new(output, Compression::default());

    let result = copy(&mut input, &mut encoder);
    match result {
        Ok(_) => {
            let source_len = File::open(source_file).unwrap().metadata().unwrap().len();
            let target_len = encoder.finish().unwrap().into_inner().unwrap().metadata().unwrap().len();

            println!("Source len: {:?}", source_len);
            println!("Target len: {:?}", target_len);
        }
        Err(e) => eprintln!("Error compressing: {:?}", e),
    }
}

fn decompress(source_file: &str, target_file: &str) {
    let  input = BufReader::new(File::open(source_file).unwrap());
    let mut  output = BufWriter::new(File::create(target_file).unwrap());
    let mut decoder = GzDecoder::new(input);

    let result = copy(&mut decoder, &mut output);
    match result {
        Ok(_) => {
            let source_len = File::open(source_file).unwrap().metadata().unwrap().len();
            let target_len = File::open(target_file).unwrap().metadata().unwrap().len();

            println!("Source len: {:?}", source_len);
            println!("Target len: {:?}", target_len);
        }
        Err(e) => eprintln!("Error decompressing: {:?}", e),
    }
}

fn main() {
    if args().len() != 4 {
        eprintln!("Usage: compress <source_file> <compressed_file>");
        eprintln!("usage: decompress <compressed_file> <target_file>");
        return;
    }

    let operation = args().nth(1).unwrap();
    let source_file = args().nth(2).unwrap();
    let target_file = args().nth(3).unwrap();

    match operation.as_str() {
        "compress" => compress(&source_file, &target_file),
        "decompress" => decompress(&source_file, &target_file),
        _ => eprintln!("Invalid operation: {}", operation),
    }
}
