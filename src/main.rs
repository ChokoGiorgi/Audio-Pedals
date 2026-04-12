mod processor;
mod dsp;
mod converter;

use std::error::Error;

use processor::AudioProcessor;
use dsp::distortion::Distortion;
use converter::mp3_to_frames;

use crate::converter::frames_to_wav::frames_to_wav;

fn main() -> Result<(), Box<dyn Error>> {
    let dist_pedal = Distortion::new(0.05, 5.0); //threshold, gain

    let audio = mp3_to_frames("/Users/choko/Desktop/Projects/Audio-Pedals/guitar.mp3").map_err(std::io::Error::other)?;
    let mut frames = audio.to_frame_vec::<2>().map_err(std::io::Error::other)?;

    println!("Checking {} pedal: ", dist_pedal.name());
    dist_pedal.process(frames.as_mut_slice());
    println!("Processed {} frames", frames.len());

    frames_to_wav(frames.as_slice(), audio.sample_rate).map_err(std::io::Error::other)?;
    println!("Created output.wav");
    Ok(())
}