/// Vibrato effect based on the DAFX Chapter 3 MATLAB implementation
pub struct Vibrato {
    /// Length of the delay in samples
    delay_length: usize,
    /// Length of the modulation in samples
    modulation_length: f32
}

impl Vibrato {
    /// Creates a Vibrato effect with the given sample rate in sample, delay time in seconds, and modulation frequency in hertz.
    pub fn new(sample_rate: f32, delay_time: f32, modulation_frequency: f32) -> Self {
        let delay_length = (delay_time*sample_rate).round() as usize;
        let modulation_length = modulation_frequency/sample_rate;
        Vibrato{delay_length,modulation_length}
    }
}

#[cfg(test)]
mod tests {
    use crate::Vibrato;

    #[test]
    fn test_initialization() {
        let effect = Some(Vibrato::new(44100.0, 5.0, 10.0));
        assert!(true)
    }
}
