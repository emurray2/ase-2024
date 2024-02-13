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
        let capacity = (sample_rate_hz / max_delay_secs).round() as usize;
        CombFilter {
            filter_type,
            max_delay_secs,
            sample_rate_hz,
            num_channels,
            delay_line_list: vec![RingBuffer{buffer: vec![f32::default(); capacity], head: 0, tail: capacity-1}; num_channels],
            parameter_list: vec![0.5, max_delay_secs],
        }
    }

    pub fn reset(&mut self) {
        for buffer in self.delay_line_list.iter_mut() {
            buffer.reset();
        }
    }

    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        let gain = self.get_param(FilterParam::Gain);
        let delay= self.get_param(FilterParam::Delay);
        let mut i = 0;
        for output_channel_buffer in output.iter_mut() {
            let read_location = self.delay_line_list[i].get_read_index();
            self.delay_line_list[i].set_write_index(read_location + (self.sample_rate_hz*delay) as usize);
            let input_channel_buffer = input[i];
            let mut j = 0;
            for mut output_sample in output_channel_buffer.iter_mut() {
                let x_n = input_channel_buffer[j];
                let delay_line_value = self.delay_line_list[i].pop();
                match self.filter_type {
                    FilterType::FIR => {
                        output_sample = &mut (x_n + gain * delay_line_value);
                        self.delay_line_list[i].push(x_n);
                    },
                    FilterType::IIR => {
                        output_sample = &mut (x_n + gain * delay_line_value);
                        self.delay_line_list[i].push(x_n + gain * delay_line_value);
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
                if value > self.max_delay_secs {
                    Result::Err(Error::InvalidValue {param, value})
                } else {
                    self.parameter_list[1] = value;
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
