use hound::{SampleFormat, WavSpec};
use core::f32::consts::PI;
use crate::comb_filter::FilterParam::{Delay, Gain};
use crate::comb_filter::FilterType::{FIR, IIR};
use crate::ring_buffer::RingBuffer;

pub struct CombFilter {
    filter_type: FilterType,
    max_delay_secs: f32,
    sample_rate_hz: f32,
    num_channels: usize,
    delay_line_list: Vec<RingBuffer<f32>>,
    parameter_list: Vec<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    FIR,
    IIR,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterParam {
    Gain,
    Delay,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidValue { param: FilterParam, value: f32 }
}

impl CombFilter {
    pub fn new(filter_type: FilterType, max_delay_secs: f32, sample_rate_hz: f32, num_channels: usize) -> Self {
        let capacity = (sample_rate_hz*max_delay_secs) as usize;
        CombFilter {
            filter_type,
            max_delay_secs,
            sample_rate_hz,
            num_channels,
            delay_line_list: vec![RingBuffer{buffer: vec![f32::default(); capacity+1], head: 0, tail: 1}; num_channels],
            parameter_list: vec![0.5, max_delay_secs],
        }
    }

    pub fn reset(&mut self) {
        self.set_param(FilterParam::Gain, 0.5).unwrap();
        self.set_param(FilterParam::Delay, self.max_delay_secs).unwrap();
    }

    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        let gain = self.get_param(FilterParam::Gain);
        let delay= self.get_param(FilterParam::Delay);
        let mut i = 0;
        for block in output.iter_mut() {
            let mut j = 0;
            for sample in block.iter_mut() {
                let x_n = input[i][j];
                match self.filter_type {
                    FilterType::FIR => {
                        *sample = x_n + gain * self.delay_line_list[j].pop();
                        self.delay_line_list[j].push(x_n);
                    },
                    FilterType::IIR => {
                        let value = self.delay_line_list[j].pop();
                        *sample = x_n + gain * value;
                        self.delay_line_list[j].push(x_n + gain * value);
                    },
                }
                j += 1;
            }
            i += 1;
        }
    }

    pub fn set_param(&mut self, param: FilterParam, value: f32) -> Result<(), Error> {
        match param {
            FilterParam::Delay => {
                if value > self.max_delay_secs || value < 0.0 {
                    Result::Err(Error::InvalidValue {param, value})
                } else {
                    self.parameter_list[1] = value;
                    for buffer in self.delay_line_list.iter_mut() {
                        buffer.set_read_index(buffer.capacity() - (self.sample_rate_hz*value) as usize - 1);
                    }
                    Result::Ok(())
                }
            },
            FilterParam::Gain => {
                self.parameter_list[0] = value;
                Result::Ok(())
            },
        }
    }

    pub fn get_param(&self, param: FilterParam) -> f32 {
        match param {
            FilterParam::Delay => {
                self.parameter_list[1]
            },
            FilterParam::Gain => {
                self.parameter_list[0]
            }
        }
    }

    // TODO: feel free to define other functions for your own use
}

// TODO: feel free to define other types (here or in other modules) for your own use
#[test]
fn test_zero_output() {
    let mut block_size: usize = 1024;
    let channels = 1;
    let sample_rate = 44100.0;
    let frequency: f32 = 500.0;
    let delay = 1.0 / (frequency*2.0);
    let mut comb_filter = CombFilter::new(FIR, delay, sample_rate, channels);
    _ = comb_filter.set_param(Gain, 1.0);
    let mut input_buffer = vec![vec![f32::default(); channels]; block_size];
    let mut output_buffer = vec![vec![f32::default(); channels]; block_size];
    for i in 0 .. block_size {
        for j in 0 .. channels {
            input_buffer[i][j] = (2.0*PI*frequency*(i as f32/sample_rate)).sin();
        }
    }
    let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let input_buffer_slice_2d = &input_buffer_slice[..];
    let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
    let output_buffer_slice_2d = &mut output_buffer_slice[..];
    comb_filter.process(input_buffer_slice_2d, output_buffer_slice_2d);
    for i in 0 .. block_size {
        for j in 0 .. channels {
            let value = output_buffer[i][j];
            if i > 50 {
                assert!(value <= 0.01);
            }
        }
    }
}
#[test]
fn test_magnitude_increase() {
    let mut block_size: usize = 1024;
    let channels = 1;
    let sample_rate = 44100.0;
    let frequency: f32 = 500.0;
    let delay = 1.0 / (frequency*2.0);
    let mut comb_filter = CombFilter::new(IIR, delay, sample_rate, channels);
    _ = comb_filter.set_param(Gain, 0.5);
    let mut input_buffer = vec![vec![f32::default(); channels]; block_size];
    let mut output_buffer = vec![vec![f32::default(); channels]; block_size];
    for i in 0 .. block_size {
        for j in 0 .. channels {
            input_buffer[i][j] = 0.5 * (2.0*PI*frequency*(i as f32/sample_rate)).sin();
        }
    }
    let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let input_buffer_slice_2d = &input_buffer_slice[..];
    let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
    let output_buffer_slice_2d = &mut output_buffer_slice[..];
    comb_filter.process(input_buffer_slice_2d, output_buffer_slice_2d);
    for i in 0 .. block_size {
        for j in 0 .. channels {
            if i > 50 {
                let value = output_buffer[i][j] / input_buffer[i][j];
                assert!(f32::abs(value) >= 0.4);
            }
        }
    }
}
#[test]
fn test_varying_block_size() {
    let mut block_size: usize = 1024;
    let channels = 1;
    let sample_rate = 44100.0;
    let frequency: f32 = 500.0;
    let delay = 1.0 / (frequency*2.0);
    let mut comb_filter = CombFilter::new(IIR, delay, sample_rate, channels);
    _ = comb_filter.set_param(Gain, 0.5);
    let mut input_buffer = vec![vec![f32::default(); channels]; block_size];
    let mut output_buffer = vec![vec![f32::default(); channels]; block_size];
    for i in 0 .. block_size {
        for j in 0 .. channels {
            input_buffer[i][j] = 0.5 * (2.0*PI*frequency*(i as f32/sample_rate)).sin();
        }
    }
    let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let input_buffer_slice_2d = &input_buffer_slice[..];
    let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
    let output_buffer_slice_2d = &mut output_buffer_slice[..];
    comb_filter.process(input_buffer_slice_2d, output_buffer_slice_2d);
    for i in 0 .. block_size {
        for j in 0 .. channels {
            if i > 50 {
                let value = output_buffer[i][j] / input_buffer[i][j];
                assert!(f32::abs(value) >= 0.4);
            }
        }
    }
    block_size = 2048;
    input_buffer = vec![vec![f32::default(); channels]; block_size];
    output_buffer = vec![vec![f32::default(); channels]; block_size];
    for i in 0 .. block_size {
        for j in 0 .. channels {
            input_buffer[i][j] = 0.5 * (2.0*PI*frequency*(i as f32/sample_rate)).sin();
        }
    }
    let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let input_buffer_slice_2d = &input_buffer_slice[..];
    let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
    let output_buffer_slice_2d = &mut output_buffer_slice[..];
    comb_filter.process(input_buffer_slice_2d, output_buffer_slice_2d);
    for i in 0 .. block_size {
        for j in 0 .. channels {
            if i > 50 {
                let value = output_buffer[i][j] / input_buffer[i][j];
                assert!(f32::abs(value) >= 0.5);
            }
        }
    }
}
#[test]
fn test_correct_zero() {
    let mut block_size: usize = 1024;
    let channels = 1;
    let sample_rate = 44100.0;
    let frequency: f32 = 500.0;
    let delay = 1.0 / (frequency*2.0);
    let mut comb_filter = CombFilter::new(IIR, delay, sample_rate, channels);
    _ = comb_filter.set_param(Gain, 0.5);
    let mut input_buffer = vec![vec![f32::default(); channels]; block_size];
    let mut output_buffer = vec![vec![f32::default(); channels]; block_size];
    for i in 0 .. block_size {
        for j in 0 .. channels {
            input_buffer[i][j] = 0.0;
        }
    }
    let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let input_buffer_slice_2d = &input_buffer_slice[..];
    let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
    let output_buffer_slice_2d = &mut output_buffer_slice[..];
    comb_filter.process(input_buffer_slice_2d, output_buffer_slice_2d);
    for i in 0 .. block_size {
        for j in 0 .. channels {
            if i > 50 {
                let value = output_buffer[i][j];
                assert!(f32::abs(value) <= f32::EPSILON);
            }
        }
    }
    let mut comb_filter_fir = CombFilter::new(FIR, delay, sample_rate, channels);
    for i in 0 .. block_size {
        for j in 0 .. channels {
            input_buffer[i][j] = 0.0;
        }
    }
    let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let input_buffer_slice_2d = &input_buffer_slice[..];
    let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
    let output_buffer_slice_2d = &mut output_buffer_slice[..];
    comb_filter_fir.process(input_buffer_slice_2d, output_buffer_slice_2d);
    for i in 0 .. block_size {
        for j in 0 .. channels {
            if i > 50 {
                let value = output_buffer[i][j];
                assert!(f32::abs(value) <= f32::EPSILON);
            }
        }
    }
}
#[test]
// Test if a mono channel and the same signal in stereo give the same test result
fn test_mono_and_multichannel() {
    let mut block_size: usize = 1024;
    let channels= 1;
    let multi_channels = 2;
    let sample_rate = 44100.0;
    let frequency: f32 = 500.0;
    let delay = 1.0 / (frequency*2.0);
    let mut comb_filter = CombFilter::new(IIR, delay, sample_rate, channels);
    _ = comb_filter.set_param(Gain, 0.5);
    let mut comb_filter_multi = CombFilter::new(IIR, delay, sample_rate, multi_channels);
    _ = comb_filter_multi.set_param(Gain, 0.5);
    let mut input_buffer = vec![vec![f32::default(); 1]; block_size];
    let mut input_buffer_multi = vec![vec![f32::default(); 2]; block_size];
    let mut output_buffer = vec![vec![f32::default(); 1]; block_size];
    let mut output_buffer_multi = vec![vec![f32::default(); 2]; block_size];
    for i in 0 .. block_size {
        for j in 0 .. channels {
            input_buffer[i][j] = (2.0*PI*frequency*(i as f32/sample_rate)).sin();
        }
    }
    for i in 0 .. block_size {
        for j in 0 .. multi_channels {
            input_buffer_multi[i][j] = (2.0*PI*frequency*(i as f32/sample_rate)).sin();
        }
    }
    let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let input_buffer_slice_2d = &input_buffer_slice[..];
    let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
    let output_buffer_slice_2d = &mut output_buffer_slice[..];
    let input_buffer_slice_multi = &input_buffer_multi[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let input_buffer_slice_2d_multi = &input_buffer_slice_multi[..];
    let mut output_buffer_slice_multi = output_buffer_multi[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
    let output_buffer_slice_2d_multi = &mut output_buffer_slice_multi[..];
    comb_filter.process(input_buffer_slice_2d, output_buffer_slice_2d);
    comb_filter_multi.process(input_buffer_slice_2d_multi, output_buffer_slice_2d_multi);
    for i in 0 .. block_size {
        for j in 0 .. multi_channels {
            if i > 50 {
                let value = output_buffer[i][0];
                let multi_value = output_buffer_multi[i][j];
                let channel_value = value - multi_value;
                assert!(f32::abs(channel_value) <= f32::EPSILON);
            }
        }
    }
}
