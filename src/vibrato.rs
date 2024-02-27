/// Vibrato effect based on the DAFX Chapter 3 MATLAB implementation
pub struct Vibrato {
    /// Length of the delay in samples
    delay_length: usize
}

impl Vibrato {
    /// Creates a Vibrato effect with the given sample rate and delay time in seconds.
    pub fn new(sample_rate: f32, delay_time: f32) -> Self {
        let delay_length = (delay_time*sample_rate).round() as usize;
        Vibrato{delay_length}
    }
}
