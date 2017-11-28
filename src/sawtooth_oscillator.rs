use std::f64;

pub struct SawtoothOscillator {
    frequency: f64,
    phase: f64,
}

impl SawtoothOscillator {
    /// Creates a new Sawtooth wave signal generator.
    pub fn new() -> SawtoothOscillator {
        SawtoothOscillator {
            frequency: 0.0,
            phase: 0.0,
        }
    }

    pub fn change_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    pub fn next_sample(&mut self, sample_rate: f64) -> f64 {
        let mut output = 0.0;

        for i in 1..((sample_rate/2.0/self.frequency) as u32) {
            let i_f64 = i as f64;
            output += (2.0 * f64::consts::PI * (self.phase * i_f64)).sin() / i_f64 / 2.0;
        }

        self.phase = (self.phase + self.frequency / sample_rate).fract();


        output
    }
}