use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};

use converters::Converter;
use converters::kaspersky::KasperskyPasswordManager;

mod converters;

fn main() -> Result<()> {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() != 3 {
        panic!("Usage: {} <input file> <output file>", arguments[0])
    };

    let input_file = File::open(&arguments[1])?;
    let input = BufReader::new(input_file);

    let output_file = File::create(&arguments[2])?;
    let output = BufWriter::new(output_file);

    let converter = KasperskyPasswordManager::create(input, output);
    converter.convert()
}