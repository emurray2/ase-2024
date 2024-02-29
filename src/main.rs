use std::io::Write;
#[doc(inline)]
pub use vibrato::Vibrato;
pub use vibrato::VibratoParam;
pub use lfo::LFO;
pub use lfo::LFOParam;
#[doc(hidden)]
mod ring_buffer;
#[doc(hidden)]
mod vibrato;
#[doc(hidden)]
mod lfo;

#[doc(hidden)]
fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}
#[doc(hidden)]
fn main() {
   show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 6 {
        eprintln!("Usage: {} <input wave filename> <output text filename> <sample rate> <delay time> <modulation frequency>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;
    let block_size = 1024;
    let mut out = hound::WavWriter::create(&args[2], spec).unwrap();
    let mut sample_index = 0;
    let mut block_index: usize = 0;
    let mut input_buffer = vec![vec![f32::default(); channels as usize]; block_size];
    let mut output_buffer = vec![vec![f32::default(); channels as usize]; block_size];
    let mut vibrato = Vibrato::new(args[3].parse().unwrap(), args[4].parse().unwrap(), args[5].parse().unwrap(), channels as usize, 0.0);
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        let channel_index = i % channels as usize;
        sample_index = sample_index % block_size;
        input_buffer[sample_index][channel_index] = sample;
        if channel_index == (channels - 1) as usize {
            if sample_index == (block_size - 1) {
                let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
                let input_buffer_slice_2d = &input_buffer_slice[..];
                let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
                let output_buffer_slice_2d = &mut output_buffer_slice[..];
                vibrato.process(input_buffer_slice_2d, output_buffer_slice_2d);
                for sample in output_buffer_slice_2d.iter() {
                    for channel in sample.iter() {
                        let written_sample = (*channel * i16::MAX as f32) as i16;
                        out.write_sample(written_sample).unwrap();
                    }
                }
                block_index += 1;
            }
            sample_index += 1;
        }
    }
    // Handle remaining samples by zero padding another block
    if sample_index != 0 {
        for i in sample_index ..block_size {
            for j in 0 .. channels as usize {
                input_buffer[i][j] = 0.0;
            }
        }
        let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
        let input_buffer_slice_2d = &input_buffer_slice[..];
        let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
        let output_buffer_slice_2d = &mut output_buffer_slice[..];
        vibrato.process(input_buffer_slice_2d, output_buffer_slice_2d);
        for i in sample_index .. block_size as usize {
            for j in 0 .. channels as usize {
                let sample = output_buffer[i][j];
                let written_sample = (sample * i16::MAX as f32) as i16;
                out.write_sample(written_sample).unwrap();
            }
        }
    }
}
