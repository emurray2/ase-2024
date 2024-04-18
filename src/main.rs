use std::{fs::File, io::Write, str::FromStr, time::Duration, time::Instant};

use hound::{SampleFormat};
use fast_convolver::ConvolutionMode;

mod ring_buffer;
mod fast_convolver;

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
    // create a convolver
    let impulse_response  = [0.1, 0.1 ,0.0, 0.0, 0.0, 0.0];
    let mut convolver = fast_convolver::FastConvolver::new(&impulse_response, ConvolutionMode::TimeDomain);
    let mut fast_convolver = fast_convolver::FastConvolver::new(&impulse_response, ConvolutionMode::FrequencyDomain { block_size: 2 });

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;
    if channels > 1
    {
        panic!("Input channel larger than 1!");
    }
    let mut wav_data : Vec<f32> = vec![];
    // put the data into a vector for easy process
    // each vector is a channel 
    let mut total_length = 0;
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        wav_data.push(sample);
        total_length += 1;
    }
    let mut processed_wav_data : Vec<f32> = vec![0.0; total_length];
    let now = Instant::now();
    convolver.process(&wav_data[0..total_length],  &mut processed_wav_data[0..total_length]);
    let mut conv_tail : Vec<f32> = vec![0.0; impulse_response.len() - 1];
    convolver.flush(&mut conv_tail);
    let new_now = Instant::now();
    let duration = new_now.checked_duration_since(now).unwrap();
    let time = duration.as_secs_f32();
    let fast_now = Instant::now();
    let _result = fast_convolver.process_freq(&wav_data[0..total_length]);
    let fast_new_now = Instant::now();
    let fast_time = fast_new_now.checked_duration_since(fast_now).unwrap().as_secs_f32();
    println!("time convolution time: {time} seconds");
    println!("fast convolution time: {fast_time} seconds");
    // print out and see if the tail is correct
    for cur_tail in conv_tail
    {
        println!("{cur_tail}");
    }

    // write to wav
    let write_spec = hound::WavSpec {
        channels: 1 as u16,
        sample_rate: spec.sample_rate, 
        bits_per_sample: 32, 
        sample_format: SampleFormat::Float, // Integer samples
    };

    let mut writer = hound::WavWriter::create(&args[2], write_spec).unwrap();
    
    for data_index in 0..total_length
    {
        let _ = writer.write_sample(processed_wav_data[data_index]);
    
    }
    // wirte to txt 
    let mut output_name = String::from_str(&args[2]).unwrap();
    output_name += ".txt";
    println!("{output_name}");
    let mut out = File::create(output_name).expect("Unable to create file");
    for cur_data in processed_wav_data
    {
        writeln!(out, "{}", cur_data).unwrap();
    }
}
