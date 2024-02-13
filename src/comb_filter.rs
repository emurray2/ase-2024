use std::collections::HashMap;
use std::iter::Filter;
use crate::ring_buffer::RingBuffer;

pub struct CombFilter {
    filter_type: FilterType,
    delay_line_list: Vec<RingBuffer<f32>>,
    parameters: HashMap<FilterParam, f32>,
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
            delay_line_list: vec![RingBuffer{buffer: vec![f32::default(), capacity], head: 0, tail: capacity-1}; num_channels],
            parameters: HashMap::from([(FilterParam::Gain, 0.5), (FilterParam::Delay, max_delay_secs)]),
        }
    }

    pub fn reset(&mut self) {
        todo!("implement")
    }

    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        todo!("implement");
    }

    pub fn set_param(&mut self, param: FilterParam, value: f32) -> Result<(), Error> {
        todo!("implement")
    }

    pub fn get_param(&self, param: FilterParam) -> f32 {
        todo!("implement")
    }

    // TODO: feel free to define other functions for your own use
}

// TODO: feel free to define other types (here or in other modules) for your own use
