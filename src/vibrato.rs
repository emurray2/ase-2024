use crate::lfo::LFO;
use crate::ring_buffer::RingBuffer;

/// Vibrato effect based on the DAFX Chapter 3 MATLAB implementation
pub struct Vibrato {
    #[doc(hidden)]
    sample_rate: f32,
    /// The delay (width) of the vibrato in seconds
    pub delay_time: f32,
    #[doc(hidden)]
    delay_length: usize,
    /// The modulation frequency of the vibrato in Hertz
    pub modulation_frequency: f32,
    #[doc(hidden)]
    modulator: LFO,
    delay_line_list: Vec<RingBuffer<f32>>
}

/// Parameters which characterize a Vibrato effect
pub enum VibratoParam {
    /// The delay (width) of the vibrato in seconds. Default value = 0.005 seconds (5 milliseconds).
    DelayTime,
    /// The modulation frequency of the vibrato in Hertz. Default value = 0.1 Hertz.
    ModulationFrequency
}

impl Vibrato {
    /// Creates a Vibrato effect with the given sample rate in samples, delay time, modulation frequency, and number of channels
    pub fn new(sample_rate: f32, delay_time: f32, modulation_frequency: f32, num_channels: usize) -> Self {
        let delay_length = (delay_time*sample_rate).round() as usize;
        let capacity = 2+delay_length+delay_length*2;
        Vibrato{
            sample_rate,
            delay_time,
            delay_length,
            modulation_frequency,
            modulator: LFO::new(modulation_frequency, 1.0, delay_length, num_channels, sample_rate),
            delay_line_list: vec![RingBuffer{buffer: vec![f32::default(); capacity+1], head: 0, tail: 1}; num_channels]
        }
    }
    /// Sets the parameter values for this effect. See [VibratoParam].
    pub fn set_param(&mut self, param: VibratoParam, value: f32) {
        match param {
            VibratoParam::DelayTime => {
                self.delay_time = value;
                self.delay_length = (self.delay_time*self.sample_rate).round() as usize;
            }
            VibratoParam::ModulationFrequency => {
                self.modulation_frequency = value;
                self.modulator.frequency = self.modulation_frequency;
            }
        }
    }
    /// Reset the effect and its parameters to their default state
    pub fn reset(&mut self) {
        self.set_param(VibratoParam::DelayTime, 0.005);
        self.set_param(VibratoParam::ModulationFrequency, 0.1);
    }
    /// Process block for the effect. All the Digital Signal Processing happens here.
    pub fn process(&mut self, input: &[&[f32]], output: &mut[&mut[f32]]) {
        let block_size = input[0].len();
        let num_channels = input.len();
        let mut modulator_buffers = vec![vec![f32::default(); block_size]; num_channels];
        let mut output_buffer_slice = modulator_buffers[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
        let output_buffer_slice_2d = &mut output_buffer_slice[..];
        self.modulator.process(output_buffer_slice_2d);
        for (i, channel) in output.iter_mut().enumerate() {
            for (j, y_n) in channel.iter_mut().enumerate() {
                let x_n = input[i][j];
                let lfo_val = modulator_buffers[i][j];
                let pointer = 1.0+self.delay_length as f32+self.delay_length as f32*lfo_val;
                let rounded_pointer = f32::floor(pointer);
                let frac = pointer-rounded_pointer;
                self.delay_line_list[i].push(x_n);
                *y_n = self.delay_line_list[i].get_frac(frac);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::{Vibrato, VibratoParam};

    #[test]
    fn test_initialization() {
        let effect = Vibrato::new(44100.0, 1.0, 44100.0, 2);
        assert_eq!(effect.delay_time, 1.0);
        assert_eq!(effect.delay_length, 44100);
        assert_eq!(effect.modulation_frequency, 44100.0);
    }
    #[test]
    fn test_set_params() {
        let mut effect = Vibrato::new(44100.0, 1.0, 44100.0, 2);
        assert_eq!(effect.delay_time, 1.0);
        assert_eq!(effect.delay_length, 44100);
        assert_eq!(effect.modulation_frequency, 44100.0);
        effect.set_param(VibratoParam::DelayTime, 2.5);
        assert_eq!(effect.delay_time, 2.5);
        assert_eq!(effect.delay_length, 110250);
        effect.set_param(VibratoParam::ModulationFrequency, 148.267);
        assert_eq!(effect.modulation_frequency, 148.267);
    }
    #[test]
    fn test_reset() {
        let mut effect = Vibrato::new(44100.0, 1.0, 44100.0, 2);
        effect.reset();
        assert_eq!(effect.delay_time, 0.005);
        assert_eq!(effect.delay_length, 221);
        assert_eq!(effect.modulation_frequency, 0.1);
    }
    #[test]
    fn test_process() {
        let channels = 2;
        let block_size = 1024;
        let frequency = 440.0;
        let sample_rate = 44100.0;
        let mut effect = Vibrato::new(sample_rate, 1.0, 44100.0, channels);
        let mut input_buffer = vec![vec![f32::default(); block_size]; channels];
        let mut output_buffer = vec![vec![f32::default(); block_size]; channels];
        for i in 0 .. channels {
            for j in 0 .. block_size {
                input_buffer[i][j] = (2.0*PI*frequency*(j as f32/sample_rate)).sin();
            }
        }
        let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
        let input_buffer_slice_2d = &input_buffer_slice[..];
        let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
        let output_buffer_slice_2d = &mut output_buffer_slice[..];
        effect.process(input_buffer_slice_2d, output_buffer_slice_2d);
        for i in 0 .. channels {
            for j in 0 .. block_size {
                let input_sample = input_buffer[i][j];
                let output_sample = output_buffer[i][j];
                if input_sample != 0.0 && output_sample != 0.0 {
                    assert_ne!(output_buffer[i][j], input_buffer[i][j]);
                }
            }
        }
    }
}
