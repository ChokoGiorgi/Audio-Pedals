use std::fs::File;

use symphonia::core::audio::{AudioBufferRef, SampleBuffer};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};

#[derive(Clone, Debug)]
pub struct AudioFrames {
    pub samples: Vec<f32>, // stored in interleaving channel order
    pub sample_rate: u32,
    pub channels: u8,
}

impl AudioFrames {
    pub fn frame_count(&self) -> usize {
        if self.channels == 0 {
            return 0;
        }
        self.samples.len() / self.channels as usize
    }

    pub fn get_channels(&self) -> Vec<Vec<f32>> {
        if self.channels == 0 {
            return Vec::new();
        }

        let mut channels = vec![Vec::new(); self.channels as usize];
        for (i, &sample) in self.samples.iter().enumerate() {
            channels[i % self.channels as usize].push(sample);
        }
        channels
    }

    /// Convert interleaved samples into a vector of fixed-size frames.
    ///
    /// Use `result.as_slice()` or `result.as_mut_slice()` to obtain a frame slice.
    pub fn to_frame_vec<const N: usize>(&self) -> Result<Vec<[f64; N]>, String> {
        if N == 0 {
            return Err("Frame width N must be greater than 0".to_string());
        }

        if self.channels as usize != N {
            return Err(format!(
                "Channel count mismatch: AudioFrames has {} channels but requested frame width is {N}",
                self.channels
            ));
        }

        if self.samples.len() % N != 0 {
            return Err(format!(
                "Invalid interleaved sample length {} for frame width {N}",
                self.samples.len()
            ));
        }

        let mut frames = Vec::with_capacity(self.samples.len() / N);
        for chunk in self.samples.chunks_exact(N) {
            let mut frame = [0.0_f64; N];
            for (dst, &sample) in frame.iter_mut().zip(chunk.iter()) {
                *dst = sample as f64;
            }
            frames.push(frame);
        }

        Ok(frames)
    }
}

fn append_interleaved_samples(decoded: AudioBufferRef<'_>, out: &mut Vec<f32>) {
    let mut sample_buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
    sample_buf.copy_interleaved_ref(decoded);
    out.extend_from_slice(sample_buf.samples());
}

pub fn mp3_to_frames(file_path: &str) -> Result<AudioFrames, String> {
    let file = File::open(file_path).map_err(|e| format!("Failed to open MP3 file: {e}"))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("mp3");

    let probed = get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| format!("Failed to probe media format: {e}"))?;

    let mut format = probed.format;

    let track = format
        .default_track()
        .ok_or("No audio track found")?;

    let track_id = track.id;
    let sample_rate = track
        .codec_params
        .sample_rate
        .ok_or("No sample rate information found")?;
    let channels = track
        .codec_params
        .channels
        .ok_or("No channel information found")?
        .count() as u8;

    let mut decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| format!("Failed to create decoder: {e}"))?;

    let mut all_samples = Vec::new();

    loop {
        match format.next_packet() {
            Ok(packet) => {
                if packet.track_id() != track_id {
                    continue;
                }

                match decoder.decode(&packet) {
                    Ok(decoded) => append_interleaved_samples(decoded, &mut all_samples),
                    Err(SymphoniaError::DecodeError(_)) => continue,
                    Err(SymphoniaError::IoError(e))
                        if e.kind() == std::io::ErrorKind::UnexpectedEof =>
                    {
                        break;
                    }
                    Err(e) => return Err(format!("Decode error: {e}")),
                }
            }
            Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => return Err(format!("Read packet error: {e}")),
        }
    }

    if all_samples.is_empty() {
        return Err("No audio data found in MP3 file".to_string());
    }

    Ok(AudioFrames {
        samples: all_samples,
        sample_rate,
        channels,
    })
}

pub fn mp3_to_samples(file_path: &str) -> Result<Vec<f32>, String> {
    Ok(mp3_to_frames(file_path)?.samples)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_frames_frame_count() {
        let frames = AudioFrames {
            samples: vec![0.1, 0.2, 0.3, 0.4],
            sample_rate: 44_100,
            channels: 2,
        };
        assert_eq!(frames.frame_count(), 2);
    }
    #[test]
    fn test_audio_frames_get_channels() {
        let frames = AudioFrames {
            samples: vec![0.1, 0.2, 0.3, 0.4],
            sample_rate: 44_100,
            channels: 2,
        };

        let channels = frames.get_channels();
        assert_eq!(channels.len(), 2);
        assert_eq!(channels[0], vec![0.1, 0.3]);
        assert_eq!(channels[1], vec![0.2, 0.4]);
    }

    #[test]
    fn test_audio_frames_to_frame_vec_stereo() {
        let frames = AudioFrames {
            samples: vec![0.1, 0.2, 0.3, 0.4],
            sample_rate: 44_100,
            channels: 2,
        };

        let frame_vec = frames.to_frame_vec::<2>().unwrap();
        assert_eq!(frame_vec, vec![[0.1, 0.2], [0.3, 0.4]]);
    }

    #[test]
    fn test_audio_frames_to_frame_vec_channel_mismatch() {
        let frames = AudioFrames {
            samples: vec![0.1, 0.2, 0.3, 0.4],
            sample_rate: 44_100,
            channels: 2,
        };

        let err = frames.to_frame_vec::<1>().unwrap_err();
        assert!(err.contains("Channel count mismatch"));
    }
}
