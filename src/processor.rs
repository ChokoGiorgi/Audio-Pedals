pub trait AudioProcessor {
    fn process(&mut self, input: f32) -> f32; //effect (distortion -> distortion)
    fn name(&mut self) -> &str; //returns the name of the pedal
}