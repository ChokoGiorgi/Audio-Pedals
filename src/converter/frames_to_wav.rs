use hound::{WavSpec, WavWriter};

pub fn frames_to_wav<const N: usize>(frames: &[[f64; N]], sample_rate: u32) -> Result<(), String> {
	if N == 0 {
		return Err("Frame width must be greater than 0".to_string());
	}

	let spec = WavSpec {
		channels: N as u16,
		sample_rate,
		bits_per_sample: 16,
		sample_format: hound::SampleFormat::Int,
	};

	let mut writer = WavWriter::create("output.wav", spec)
		.map_err(|e| format!("Failed to create output.wav: {e}"))?;

	for frame in frames {
		for &sample in frame {
			let clamped = sample.clamp(-1.0, 1.0);
			writer
				.write_sample((clamped * i16::MAX as f64) as i16)
				.map_err(|e| format!("Failed to write WAV sample: {e}"))?;
		}
	}

	writer
		.finalize()
		.map_err(|e| format!("Failed to finalize WAV file: {e}"))?;

	Ok(())
}