use std::{fs::File, io::Write};
use std::io::{BufRead, BufWriter, Read};
use crate::comb_filter::{CombFilter, FilterType};

mod comb_filter;
mod ring_buffer;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;

    // TODO: Modify this to process audio in blocks using your comb filter and write the result to an audio file.
    //       Use the following block size:
    let block_size = 1024;

    // Read audio data and write it to the output text file (one column per channel)
    //let mut out = File::create(&args[2]).expect("Unable to create file");
    let mut out = hound::WavWriter::create(&args[2], spec).unwrap();
    let mut sample_index = 0;
    let mut block_index: u32 = 0;
    let mut input_buffer = vec![vec![f32::default(); channels as usize]; block_size as usize];
    let mut output_buffer = vec![vec![f32::default(); channels as usize]; block_size as usize];
    let mut comb_filter = CombFilter::new(FilterType::FIR, 1.0, 44100.0, channels as usize);
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        let channel_index = i % channels as usize;
        if sample_index < block_size {
            input_buffer[sample_index][channel_index] = sample;
        } else {
            let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
            let input_buffer_slice_2d = &input_buffer_slice[..];
            let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
            let output_buffer_slice_2d = &mut output_buffer_slice[..];
            comb_filter.process(input_buffer_slice_2d, output_buffer_slice_2d);
            for block_index in output_buffer_slice_2d.iter_mut() {
                for channel in block_index.iter_mut() {
                    let written_sample = (*channel * i16::MAX as f32) as i16;
                    out.write_sample(written_sample).unwrap();
                }
            }
            sample_index = 0;
            input_buffer[sample_index][channel_index] = sample;
            block_index += 1;
        }
        if i % channels as usize == (channels - 1) as usize {
            sample_index += 1;
        }
    }
    let current_length = (block_index*block_size as u32);
    let remaining_length = (reader.len()/channels as u32) - current_length;
    let mut i: u32 = 0;
    while i < remaining_length {
        out.write_sample(0).unwrap();
        out.write_sample(0).unwrap();
        i += 1;
    }
}
