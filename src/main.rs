use std::path::Path;
use std::{fs::File, io::Write};
use std::fmt::Debug;
use std::io::Read;
use std::ptr::write;
use hound::WavReader;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    // First argument is input .wav file, second argument is output text file.
    let args: Vec<String> = std::env::args().collect();
    // TODO: your code here
    let input_filename = &args[1];
    let output_filename = &args[2];

    // Open the input wave file and determine number of channels
    // TODO: your code here; see `hound::WavReader::open`.
    let path = Path::new(input_filename);
    let write_path = Path::new(output_filename);
    let mut reader = hound::WavReader::open(&path);
    let existing_reader = reader.as_mut().unwrap();
    // Read audio data and write it to the output text file (one column per channel)
    // TODO: your code here; we suggest using `hound::WavReader::samples`, `File::create`, and `write!`.
    //       Remember to convert the samples to floating point values and respect the number of channels!
    let samples = hound::WavReader::samples::<i16>((existing_reader));
    let file = File::create(&write_path);
    let mut existing_file = file.unwrap();
    for sample in samples {
        let existing_sample = sample.unwrap() as f64;
        let min = -32768.0;
        let max = 32767.0;
        let mut floating_sample = 0.0;
        if existing_sample.is_sign_positive() {
            floating_sample = existing_sample / max;
        } else if existing_sample.is_sign_negative() {
            floating_sample = (existing_sample / min) * -1.0;
        } else {
            floating_sample = 0.0;
        }
        for i in 0..2 {
            write!(existing_file, "{}", floating_sample).unwrap();
            write!(existing_file, ",").unwrap();
            continue
        }
        write!(existing_file, "\n").unwrap();
    }
}
