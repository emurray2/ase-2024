use std::f32::consts::PI;
use crate::lfo::LFO;
use crate::ring_buffer::RingBuffer;

// The file is designed similar to how we designed the comb filter in the last assignment,
// except there is a mod index to store the index of an LFO

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
    delay_line_list: Vec<RingBuffer<f32>>,
    mod_index: f32,
    modulation_amplitude: f32
}

/// Parameters which characterize a Vibrato effect
pub enum VibratoParam {
    /// The delay (width) of the vibrato in seconds. Default value = 0.005 seconds (5 milliseconds).
    DelayTime,
    /// The modulation frequency of the vibrato in Hertz. Default value = 0.1 Hertz.
    ModulationFrequency
}

impl Vibrato {
    /// Creates a Vibrato effect with the given sample rate in samples, delay time, modulation frequency, number of channels, and modulation amplitude
    pub fn new(sample_rate: f32, delay_time: f32, modulation_frequency: f32, num_channels: usize, modulation_amplitude: f32) -> Self {
        let delay_length = (delay_time*sample_rate).round() as usize;
        let capacity = 2+delay_length+delay_length*2;
        Vibrato{
            sample_rate,
            delay_time,
            delay_length,
            modulation_frequency,
            modulator: LFO::new(modulation_frequency, 1.0, delay_length, num_channels, sample_rate),
            delay_line_list: vec![RingBuffer{buffer: vec![f32::default(); capacity], head: 0, tail: 0}; num_channels],
            mod_index: 0.0,
            modulation_amplitude
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
        let block_size = input.len();
        let num_channels = input[0].len();
        let mut modulator_buffers = vec![vec![f32::default(); num_channels]; block_size];
        let mut output_buffer_slice = modulator_buffers[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
        let output_buffer_slice_2d = &mut output_buffer_slice[..];
        self.modulator.process(output_buffer_slice_2d);
        for (i, block) in output.iter_mut().enumerate() {
            let lfo_val = 1.0 * (2.0*PI*self.mod_index*(self.modulation_frequency/self.sample_rate)).sin();
            for (j, channel) in block.iter_mut().enumerate() {
                let x_n = input[i][j];
                let pointer = 1.0+self.delay_length as f32+(self.delay_length as f32*lfo_val);
                self.delay_line_list[j].push(x_n);
                self.delay_line_list[j].pop();
                *channel = self.delay_line_list[j].get_frac(pointer);
            }
            self.mod_index += 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::{Vibrato, VibratoParam};
    use crate::ring_buffer::RingBuffer;

    #[test]
    fn test_initialization() {
        let effect = Vibrato::new(44100.0, 1.0, 44100.0, 2, 1.0);
        assert_eq!(effect.delay_time, 1.0);
        assert_eq!(effect.delay_length, 44100);
        assert_eq!(effect.modulation_frequency, 44100.0);
    }
    #[test]
    fn test_set_params() {
        let mut effect = Vibrato::new(44100.0, 1.0, 44100.0, 2, 1.0);
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
        let mut effect = Vibrato::new(44100.0, 1.0, 44100.0, 2, 1.0);
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
        let mut effect = Vibrato::new(sample_rate, 1.0, 44100.0, channels, 1.0);
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
        effect.process(input_buffer_slice_2d, output_buffer_slice_2d);
        for i in 0 .. block_size {
            for j in 0 .. channels {
                let input_sample = input_buffer[i][j];
                let output_sample = output_buffer[i][j];
                if input_sample != 0.0 && output_sample != 0.0 {
                    assert_ne!(output_buffer[i][j], input_buffer[i][j]);
                }
            }
        }
    }
    #[test]
    fn test_modulation_zero() {
        let channels = 2;
        let block_size = 1024;
        let frequency = 440.0;
        let sample_rate = 44100.0;
        let mut effect = Vibrato::new(sample_rate, 1.0, 44100.0, channels, 0.0);
        let mut input_buffer = vec![vec![f32::default(); channels]; block_size];
        let mut output_buffer = vec![vec![f32::default(); channels]; block_size];
        let mut delayed_buffer = vec![vec![f32::default(); channels]; block_size];
        let delay_length = effect.delay_length;
        let capacity = 2+delay_length+delay_length*2;
        let mut ring_buffer_list = vec![RingBuffer{buffer: vec![f32::default(); capacity], head: 0, tail: 0}; channels];
        for i in 0 .. block_size {
            for j in 0 .. channels {
                input_buffer[i][j] = (2.0*PI*frequency*(i as f32/sample_rate)).sin();
                ring_buffer_list[j].push(input_buffer[i][j]);
                delayed_buffer[i][j] = ring_buffer_list[j].pop();
            }
        }
        let input_buffer_slice = &input_buffer[..].iter().map(|v| v.as_slice()).collect::<Vec<_>>();
        let input_buffer_slice_2d = &input_buffer_slice[..];
        let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
        let output_buffer_slice_2d = &mut output_buffer_slice[..];
        effect.process(input_buffer_slice_2d, output_buffer_slice_2d);
        for i in 0 .. block_size {
            for j in 0 .. channels {
                let input_sample = input_buffer[i][j];
                let output_sample = output_buffer[i][j];
                if input_sample != 0.0 && output_sample != 0.0 {
                    assert_eq!(output_buffer[i][j], delayed_buffer[i][j]);
                }
            }
        }
    }
}
