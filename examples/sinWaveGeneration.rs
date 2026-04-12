use dasp::{Signal, signal::{self, Phase, sine}};
use hound::{WavSpec, WavWriter};

fn main () {
	const A_HERTZ: f64 = 440.0_f64;
	let SAMPLE_RATE: u32 = 44100;
	// 1. Define the audio format (The "Spec")
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // 2. Create a writer
    let mut writer = WavWriter::create("sine.wav", spec).unwrap();

    // 3. Create a signal using dasp (440Hz sine wave)
    let mut signal = signal::rate(SAMPLE_RATE.into()).const_hz(A_HERTZ).sine();

		let num_secs = 3;
    // 4. Generate 2 seconds of audio (44100 samples per second * 2)
    for _ in 0..(SAMPLE_RATE * num_secs) {
        let frame = signal.next(); // Get next sample
        
        // dasp samples are usually f32 (-1.0 to 1.0)
        // We need to scale it to i16 for our WAV file
        let sample = (frame * i16::MAX as f64) as i16;
        
        writer.write_sample(sample).unwrap();
    }

    // Writer is automatically finalized when it goes out of scope
    println!("File 'sine.wav' generated successfully!");
}
