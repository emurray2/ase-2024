/// Vibrato effect based on the DAFX Chapter 3 MATLAB implementation
pub struct Vibrato {
    /// The delay (width) of the vibrato in seconds
    pub delay_time: f32,
    #[doc(hidden)]
    delay_length: usize,
    /// The modulation frequency of the vibrato in Hertz
    pub modulation_frequency: f32,
    #[doc(hidden)]
    modulation_length: f32,
}

/// Parameters which characterize a Vibrato effect
pub enum VibratoParam {

}

impl Vibrato {
    /// Creates a Vibrato effect with the given sample rate in samples, delay time, and modulation frequency
    pub fn new(sample_rate: f32, delay_time: f32, modulation_frequency: f32) -> Self {
        let delay_length = (delay_time*sample_rate).round() as usize;
        let modulation_length = modulation_frequency/sample_rate;
        Vibrato{delay_time,delay_length,modulation_frequency,modulation_length}
    }
}

#[cfg(test)]
mod tests {
    use crate::Vibrato;

    #[test]
    fn test_initialization() {
        let effect = Vibrato::new(44100.0, 1.0, 44100.0);
        assert_eq!(effect.delay_time, 1.0);
        assert_eq!(effect.delay_length, 44100);
        assert_eq!(effect.modulation_frequency, 44100.0);
        assert_eq!(effect.modulation_length, 1.0);
    }
}
