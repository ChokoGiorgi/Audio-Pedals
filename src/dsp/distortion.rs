use crate::processor::AudioProcessor;

pub struct Distortion {
    pub threshold: f32,
    pub gain: f32,
}

impl Distortion {
    pub fn new(threshold: f32, gain: f32) -> Self {
        Self { threshold, gain } //returning new distortion pedal with threshold,gain attributes
    }
}

impl AudioProcessor for Distortion {
    fn name(&mut self) -> &str {
        "Distortion"
    } //name of the effect

    fn process(&mut self, input: f32) -> f32 {
        let mut boosted = input * self.gain;

        if boosted > self.threshold {
            boosted = self.threshold;
        }

        else if boosted < -self.threshold {
            boosted = -self.threshold;
        } //if the signal is not in the threshold range, then it clips

        boosted
    }
}