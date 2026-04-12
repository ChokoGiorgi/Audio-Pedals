use dasp::{Frame, Signal, frame::{Mono, Stereo}, signal};

use crate::processor::{AudioProcessor};

pub struct Distortion {
    pub threshold: f64, // value between 0 and 1,
    pub gain: f64, // a value between 0 and 10
}

impl Distortion {
    pub fn new(threshold: f64, gain: f64) -> Self {
        Self { threshold, gain } //returning new distortion pedal with threshold,gain attributes
    }
}


impl AudioProcessor for Distortion {
    fn name(&self) -> &str {
        "Distortion"
    } //name of the effect

    fn process<F>(&self, input: &mut [F]) 
    where 
        F: Frame<Sample = f64> + Copy
    {
        
        let makeup = if self.threshold > 0.0 { 1.0 / self.threshold } else { 1.0 };

        for frame in input.iter_mut() {
            // We mutate the frame in place. 
            *frame = frame.map(|s| {
                // 1. Gain
                let driven = s * self.gain;
                
                // 2. Hard Clip (The manual way)
                let clipped = if driven > self.threshold {
                    self.threshold
                } else if driven < -self.threshold {
                    -self.threshold
                } else {
                    driven
                };

                // 3. Makeup Gain
                clipped * makeup
            });
        }

    }
    
}