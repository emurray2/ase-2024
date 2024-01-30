use std::path::Path;
use std::{fs::File, io::Write};

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
    dbg!(input_filename);
    dbg!(output_filename);

    // Open the input wave file and determine number of channels
    // TODO: your code here; see `hound::WavReader::open`.
    let path = Path::new(input_filename);
    // Read audio data and write it to the output text file (one column per channel)
    // TODO: your code here; we suggest using `hound::WavReader::samples`, `File::create`, and `write!`.
    //       Remember to convert the samples to floating point values and respect the number of channels!
}
