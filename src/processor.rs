use dasp::Frame;


pub trait AudioProcessor {
    fn process<F>(&self, input: &mut [F]) where F:Frame<Sample = f64>; //effect (distortion -> distortion)
    fn name(&self) -> &str; //returns the name of the pedal
}