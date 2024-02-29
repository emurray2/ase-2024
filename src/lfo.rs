use std::f32::consts::PI;
use crate::ring_buffer::RingBuffer;
use crate::vibrato;

/// Generates a sine LFO for operating on other effects, such as [vibrato::Vibrato]
pub struct LFO {
    /// The amplitude of the LFO signal
    pub amplitude: f32,
    /// The frequency of the LFO signal in Hertz
    pub frequency: f32,
    #[doc(hidden)]
    buffer_list: Vec<RingBuffer<f32>>,
    sample_rate: f32
}

/// A type which describes the parameters characterizing the [LFO]
pub enum LFOParam {
    /// The amplitude of the LFO signal. Default value = 1.0
    Amplitude,
    /// The frequency of the LFO signal in Hertz. Default value = 440.0
    Frequency
}

impl LFO {
    /// Create a new LFO with the specified frequency, amplitude, length in samples, number of channels, and sample_rate
    pub fn new(frequency: f32, amplitude: f32, length: usize, num_channels: usize, sample_rate: f32) -> Self {
        LFO{amplitude,frequency,buffer_list: vec![RingBuffer::<f32>::new(length); num_channels],sample_rate}
    }
    /// Sets the parameter values for this effect. See [LFO]
    pub fn set_param(&mut self, param: LFOParam, value: f32) {
        match param {
            LFOParam::Amplitude => {
                self.amplitude = value
            }
            LFOParam::Frequency => {
                self.frequency = value
            }
        }
    }
    /// Resets the LFO and sets parameters back to default values
    pub fn reset(&mut self) {
        self.set_param(LFOParam::Amplitude, 1.0);
        self.set_param(LFOParam::Frequency, 440.0);
    }
    /// Process block for the effect. All the Digital Signal Processing happens here.
    pub fn process(&mut self, output: &mut[&mut[f32]]) {
        for (i, channel) in output.iter_mut().enumerate() {
            for (j, y_n) in channel.iter_mut().enumerate() {
                self.buffer_list[i].push(2.0*PI*self.frequency*(j as f32/self.sample_rate).sin() * self.amplitude);
                *y_n = self.buffer_list[i].pop();
            }
        }
    }
}

mod tests {
    use std::f32::consts::PI;
    use crate::lfo::LFO;
    use crate::LFOParam;

    fn test_initialization() {
        let effect = LFO::new(440.0, 1.0, 50, 2, 44100.0);
        assert!(true);
    }
    #[test]
    fn test_set_params() {
        let mut effect = LFO::new(440.0, 1.0, 50, 2, 44100.0);
        effect.set_param(LFOParam::Frequency, 880.0);
        assert_eq!(effect.frequency, 880.0);
        effect.set_param(LFOParam::Amplitude, 2.0);
        assert_eq!(effect.amplitude, 2.0);
    }
    #[test]
    fn test_reset() {
        let mut effect = LFO::new(123.4, 1.2, 50, 2, 44100.0);
        effect.reset();
        assert_eq!(effect.frequency, 440.0);
        assert_eq!(effect.amplitude, 1.0);
    }
    #[test]
    fn test_process() {
        let channels = 2;
        let block_size = 1024;
        let frequency = 440.0;
        let amplitude = 1.0;
        let sample_rate = 44100.0;
        let mut effect = LFO::new(frequency, amplitude, block_size, channels, sample_rate);
        let mut output_buffer = vec![vec![f32::default(); block_size]; channels];
        let mut output_buffer_slice = output_buffer[..].iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<_>>();
        let output_buffer_slice_2d = &mut output_buffer_slice[..];
        effect.process(output_buffer_slice_2d);
        for i in 0 .. channels {
            for j in 0 .. block_size {
                assert_eq!(output_buffer[i][j], 2.0*PI*frequency*(j as f32/sample_rate).sin() * amplitude);
            }
        }
    }
}
