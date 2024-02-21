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
        for buffer in self.delay_line_list.iter_mut() {
            buffer.reset();
            buffer.set_read_index((self.sample_rate_hz*self.max_delay_secs) as usize - 1);
        }
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
