use hound::{WavReader, WavWriter};
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::time::Instant;
use synthphone_vocals::{AutotuneConfig, AutotuneState, MusicalSettings, process_autotune};

#[derive(Debug, Clone)]
pub enum ProcessingProgress {
    Progress(f32), // 0.0 to 1.0
    Status(String),
}

#[derive(Debug, Clone)]
pub enum ProcessingResult {
    Success { samples_processed: usize, duration_ms: f64 },
    Error(String),
}

#[derive(Default)]
pub struct AudioProcessor;

impl AudioProcessor {
    pub fn process_file(
        input_path: &PathBuf,
        output_path: &PathBuf,
        config: AutotuneConfig,
        settings: MusicalSettings,
        progress_sender: Sender<ProcessingProgress>,
    ) -> ProcessingResult {
        let start_time = Instant::now();

        // Send initial status
        let _ =
            progress_sender.send(ProcessingProgress::Status("Opening input file...".to_string()));

        // Open input WAV file
        let mut reader = match WavReader::open(input_path) {
            Ok(reader) => reader,
            Err(e) => return ProcessingResult::Error(format!("Failed to open input file: {}", e)),
        };

        let spec = reader.spec();
        log::info!("Input file spec: {:?}", spec);

        // Validate audio format
        if spec.channels != 1 && spec.channels != 2 {
            return ProcessingResult::Error(format!(
                "Unsupported channel count: {}. Only mono and stereo are supported.",
                spec.channels
            ));
        }

        if spec.bits_per_sample != 16 && spec.bits_per_sample != 24 && spec.bits_per_sample != 32 {
            return ProcessingResult::Error(format!(
                "Unsupported bit depth: {}. Only 16, 24, and 32-bit are supported.",
                spec.bits_per_sample
            ));
        }

        // Read all samples
        let _ =
            progress_sender.send(ProcessingProgress::Status("Reading audio data...".to_string()));

        let samples: Result<Vec<i32>, _> = reader.samples().collect();
        let samples = match samples {
            Ok(samples) => samples,
            Err(e) => return ProcessingResult::Error(format!("Failed to read samples: {}", e)),
        };

        let total_samples = samples.len();
        log::info!("Read {} samples", total_samples);

        // Convert to f32 and handle channels
        let _ = progress_sender
            .send(ProcessingProgress::Status("Converting audio format...".to_string()));

        let mut audio_data = Vec::with_capacity(total_samples);
        let scale_factor = match spec.bits_per_sample {
            16 => 1.0 / 32768.0,
            24 => 1.0 / 8388608.0,
            32 => 1.0 / 2147483648.0,
            _ => return ProcessingResult::Error("Unsupported bit depth".to_string()),
        };

        for sample in samples {
            audio_data.push(sample as f32 * scale_factor);
        }

        // Convert stereo to mono if needed (simple average)
        let mono_data = if spec.channels == 2 {
            let _ = progress_sender
                .send(ProcessingProgress::Status("Converting stereo to mono...".to_string()));

            let mut mono = Vec::with_capacity(audio_data.len() / 2);
            for i in (0..audio_data.len()).step_by(2) {
                let left = audio_data[i];
                let right = audio_data.get(i + 1).copied().unwrap_or(0.0);
                mono.push((left + right) * 0.5);
            }
            mono
        } else {
            audio_data
        };

        let mono_samples = mono_data.len();
        log::info!("Processing {} mono samples", mono_samples);

        // Update config with correct sample rate
        let mut processing_config = config;
        processing_config.sample_rate = spec.sample_rate as f32;

        // Create autotune state
        let _ = progress_sender
            .send(ProcessingProgress::Status("Initializing autotune...".to_string()));

        let mut autotune_state = AutotuneState::new(processing_config);

        // Process audio in chunks
        let _ = progress_sender.send(ProcessingProgress::Status("Processing audio...".to_string()));

        let fft_size = processing_config.fft_size;
        let hop_size = processing_config.hop_size;

        let mut processed_audio = Vec::new();
        let mut chunk_index = 0;
        let total_chunks = (mono_samples + fft_size - 1) / hop_size;

        // Process overlapping frames
        let mut input_buffer = vec![0.0f32; fft_size];
        let mut output_buffer = vec![0.0f32; fft_size];

        let mut sample_pos = 0;
        while sample_pos + fft_size <= mono_samples {
            // Fill input buffer
            input_buffer.copy_from_slice(&mono_data[sample_pos..sample_pos + fft_size]);

            // Process with autotune
            match process_autotune(
                &input_buffer,
                &mut output_buffer,
                &mut autotune_state,
                &settings,
            ) {
                Ok(_) => {
                    // Add processed samples to output (overlap-add)
                    if processed_audio.len() < sample_pos + fft_size {
                        processed_audio.resize(sample_pos + fft_size, 0.0);
                    }

                    for (i, &sample) in output_buffer.iter().enumerate() {
                        processed_audio[sample_pos + i] += sample;
                    }
                }
                Err(e) => {
                    log::warn!("Autotune processing error at sample {}: {:?}", sample_pos, e);

                    // Fallback to original audio for this chunk
                    if processed_audio.len() < sample_pos + fft_size {
                        processed_audio.resize(sample_pos + fft_size, 0.0);
                    }

                    for (i, &sample) in input_buffer.iter().enumerate() {
                        processed_audio[sample_pos + i] += sample;
                    }
                }
            }

            // Update progress
            chunk_index += 1;
            let progress = chunk_index as f32 / total_chunks as f32;
            let _ = progress_sender.send(ProcessingProgress::Progress(progress));

            // Advance by hop size for overlap
            sample_pos += hop_size;
        }

        // Handle any remaining samples
        if sample_pos < mono_samples {
            let remaining = mono_samples - sample_pos;
            input_buffer.fill(0.0);
            input_buffer[..remaining].copy_from_slice(&mono_data[sample_pos..]);

            match process_autotune(
                &input_buffer,
                &mut output_buffer,
                &mut autotune_state,
                &settings,
            ) {
                Ok(_) => {
                    if processed_audio.len() < sample_pos + remaining {
                        processed_audio.resize(sample_pos + remaining, 0.0);
                    }

                    for i in 0..remaining {
                        processed_audio[sample_pos + i] += output_buffer[i];
                    }
                }
                Err(_) => {
                    // Fallback to original
                    if processed_audio.len() < sample_pos + remaining {
                        processed_audio.resize(sample_pos + remaining, 0.0);
                    }

                    for i in 0..remaining {
                        processed_audio[sample_pos + i] += mono_data[sample_pos + i];
                    }
                }
            }
        }

        // Normalize audio to prevent clipping
        let _ =
            progress_sender.send(ProcessingProgress::Status("Normalizing audio...".to_string()));

        let max_amplitude = processed_audio.iter().map(|&x| x.abs()).fold(0.0, f32::max);
        if max_amplitude > 1.0 {
            let scale = 0.95 / max_amplitude;
            for sample in &mut processed_audio {
                *sample *= scale;
            }
        }

        // Convert back to stereo if original was stereo
        let output_samples = if spec.channels == 2 {
            let _ = progress_sender
                .send(ProcessingProgress::Status("Converting to stereo...".to_string()));

            let mut stereo = Vec::with_capacity(processed_audio.len() * 2);
            for sample in processed_audio {
                stereo.push(sample); // Left channel
                stereo.push(sample); // Right channel (same as left)
            }
            stereo
        } else {
            processed_audio
        };

        // Convert back to integer format
        let _ =
            progress_sender.send(ProcessingProgress::Status("Writing output file...".to_string()));

        let output_scale = match spec.bits_per_sample {
            16 => 32767.0,
            24 => 8388607.0,
            32 => 2147483647.0,
            _ => return ProcessingResult::Error("Unsupported bit depth".to_string()),
        };

        let integer_samples: Vec<i32> =
            output_samples.iter().map(|&x| (x * output_scale).round() as i32).collect();

        // Write output WAV file
        let mut writer = match WavWriter::create(output_path, spec) {
            Ok(writer) => writer,
            Err(e) => {
                return ProcessingResult::Error(format!("Failed to create output file: {}", e));
            }
        };

        for &sample in &integer_samples {
            if let Err(e) = writer.write_sample(sample) {
                return ProcessingResult::Error(format!("Failed to write sample: {}", e));
            }
        }

        if let Err(e) = writer.finalize() {
            return ProcessingResult::Error(format!("Failed to finalize output file: {}", e));
        }

        let duration = start_time.elapsed();
        let _ = progress_sender.send(ProcessingProgress::Status("Complete!".to_string()));

        ProcessingResult::Success {
            samples_processed: integer_samples.len(),
            duration_ms: duration.as_millis() as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use tempfile::NamedTempFile;

    #[test]
    fn test_audio_processor_creation() {
        // Test that we can create the processor
        let _processor = AudioProcessor;
    }

    #[test]
    fn test_progress_enum() {
        let progress = ProcessingProgress::Progress(0.5);
        assert!(matches!(progress, ProcessingProgress::Progress(_)));

        let status = ProcessingProgress::Status("test".to_string());
        assert!(matches!(status, ProcessingProgress::Status(_)));
    }

    #[test]
    fn test_result_enum() {
        let success = ProcessingResult::Success { samples_processed: 1000, duration_ms: 500.0 };
        assert!(matches!(success, ProcessingResult::Success { .. }));

        let error = ProcessingResult::Error("test error".to_string());
        assert!(matches!(error, ProcessingResult::Error(_)));
    }
}
