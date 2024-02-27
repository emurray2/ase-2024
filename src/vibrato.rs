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
    modulation_length: f32
}

/// Parameters which characterize a Vibrato effect
pub enum VibratoParam {
    /// The delay (width) of the vibrato in seconds. Default value = 0.005 seconds (5 milliseconds).
    DelayTime,
    /// The modulation frequency of the vibrato in Hertz. Default value = 0.1 Hertz.
    ModulationFrequency
}

impl Vibrato {
    /// Creates a Vibrato effect with the given sample rate in samples, delay time, and modulation frequency
    pub fn new(sample_rate: f32, delay_time: f32, modulation_frequency: f32) -> Self {
        let delay_length = (delay_time*sample_rate).round() as usize;
        let modulation_length = modulation_frequency/sample_rate;
        Vibrato{sample_rate,delay_time,delay_length,modulation_frequency,modulation_length}
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
                self.modulation_length = self.modulation_frequency/self.sample_rate;
            }
        }
    }
    /// Reset the effect and its parameters to their default state
    pub fn reset(&mut self) {
        self.set_param(VibratoParam::DelayTime, 0.005);
        self.set_param(VibratoParam::ModulationFrequency, 0.1);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vibrato, VibratoParam};

    #[test]
    fn test_initialization() {
        let effect = Vibrato::new(44100.0, 1.0, 44100.0);
        assert_eq!(effect.delay_time, 1.0);
        assert_eq!(effect.delay_length, 44100);
        assert_eq!(effect.modulation_frequency, 44100.0);
        assert_eq!(effect.modulation_length, 1.0);
    }
    #[test]
    fn test_set_params() {
        let mut effect = Vibrato::new(44100.0, 1.0, 44100.0);
        assert_eq!(effect.delay_time, 1.0);
        assert_eq!(effect.delay_length, 44100);
        assert_eq!(effect.modulation_frequency, 44100.0);
        assert_eq!(effect.modulation_length, 1.0);
        effect.set_param(VibratoParam::DelayTime, 2.5);
        assert_eq!(effect.delay_time, 2.5);
        assert_eq!(effect.delay_length, 110250);
        effect.set_param(VibratoParam::ModulationFrequency, 148.267);
        assert_eq!(effect.modulation_frequency, 148.267);
        assert_eq!(effect.modulation_length, 0.0033620635);
    }
    #[test]
    fn test_reset() {
        let mut effect = Vibrato::new(44100.0, 1.0, 44100.0);
        effect.reset();
        assert_eq!(effect.delay_time, 0.005);
        assert_eq!(effect.delay_length, 221);
        assert_eq!(effect.modulation_frequency, 0.1);
        assert_eq!(effect.modulation_length, 0.000002267573696);
    }
}
