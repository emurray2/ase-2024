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
        todo!("implement");
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
