mod processor;
mod dsp;

use processor::AudioProcessor;
use dsp::distortion::Distortion;

fn main() {
    let mut dist_pedal = Distortion::new(2.0, 5.0); //threshold, gain

    let input = vec![-0.5, 0.5, 2.0, 3.5, 0.1];
    
    /*
    INPUT:  [-0.5, 0.5, 2.0, 3.5, 0.1]
    OUTPUT: [-2.0, 2.0, 2.0, 2.0, 0.5]
    */

    println!("Checking {} pedal: ", dist_pedal.name());

    for temp in input {
        let res = dist_pedal.process(temp);
        println!("The result is: {}", res);
    }
}