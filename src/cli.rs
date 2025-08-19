use clap::Parser;
use hound::{WavReader, WavWriter};
use std::path::PathBuf;
use std::time::Instant;
use synthphone_vocals::{AutotuneConfig, AutotuneState, MusicalSettings, process_autotune};

#[derive(Parser)]
#[command(name = "autotune-cli")]
#[command(about = "Command-line autotune processor using synthphone_vocals library")]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Input WAV file path
    #[arg(short, long, value_name = "FILE", required_unless_present = "list_keys")]
    pub input: Option<PathBuf>,

    /// Output WAV file path
    #[arg(short, long, value_name = "FILE", required_unless_present = "list_keys")]
    pub output: Option<PathBuf>,

    /// Musical key (0-23: C Major, G Major, ..., F Minor)
    #[arg(short, long, default_value_t = 0, value_name = "KEY")]
    pub key: i32,

    /// Note mode (0 = Auto snap to key, 1-12 = specific note)
    #[arg(short, long, default_value_t = 0, value_name = "NOTE")]
    pub note: i32,

    /// Octave reference (0-4)
    #[arg(long, default_value_t = 2, value_name = "OCTAVE")]
    pub octave: i32,

    /// Formant shift in semitones (-12 to +12)
    #[arg(short, long, default_value_t = 0, value_name = "SEMITONES")]
    pub formant: i32,

    /// Pitch correction strength (0.0 to 1.0)
    #[arg(short = 's', long, default_value_t = 0.8, value_name = "STRENGTH")]
    pub strength: f32,

    /// Transition speed (0.01 to 1.0)
    #[arg(short, long, default_value_t = 0.1, value_name = "SPEED")]
    pub transition: f32,

    /// FFT size (currently only 1024 is supported)
    #[arg(long, default_value_t = 1024, value_name = "SIZE")]
    pub fft_size: usize,

    /// Hop size (overlap factor)
    #[arg(long, default_value_t = 256, value_name = "SIZE")]
    pub hop_size: usize,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// List available keys and exit
    #[arg(long)]
    pub list_keys: bool,
}

pub const KEY_NAMES: [&str; 24] = [
    "C Major", "G Major", "D Major", "A Major", "E Major", "B Major", "F# Major", "C# Major",
    "F Major", "Bb Major", "Eb Major", "Ab Major", "A Minor", "E Minor", "B Minor", "F# Minor",
    "C# Minor", "G# Minor", "D# Minor", "A# Minor", "D Minor", "G Minor", "C Minor", "F Minor",
];

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.list_keys {
        println!("Available Keys:");
        for (i, name) in KEY_NAMES.iter().enumerate() {
            println!("  {}: {}", i, name);
        }
        return Ok(());
    }

    // Validate arguments
    if cli.key < 0 || cli.key >= 24 {
        return Err("Key must be between 0 and 23. Use --list-keys to see available keys.".into());
    }

    if cli.strength < 0.0 || cli.strength > 1.0 {
        return Err("Pitch correction strength must be between 0.0 and 1.0".into());
    }

    if cli.transition < 0.01 || cli.transition > 1.0 {
        return Err("Transition speed must be between 0.01 and 1.0".into());
    }

    if cli.formant < -12 || cli.formant > 12 {
        return Err("Formant shift must be between -12 and +12 semitones".into());
    }

    if cli.octave < 0 || cli.octave > 4 {
        return Err("Octave must be between 0 and 4".into());
    }

    if cli.verbose {
        println!("🎵 Autotune CLI Processor");
        println!("========================");
        println!("Input: {}", cli.input.as_ref().unwrap().display());
        println!("Output: {}", cli.output.as_ref().unwrap().display());
        println!("Key: {} ({})", cli.key, KEY_NAMES[cli.key as usize]);
        println!(
            "Note Mode: {}",
            if cli.note == 0 {
                "Auto".to_string()
            } else {
                format!("Note {}", cli.note)
            }
        );
        println!("Octave: {}", cli.octave);
        println!("Formant Shift: {} semitones", cli.formant);
        println!("Pitch Correction: {:.1}%", cli.strength * 100.0);
        println!("Transition Speed: {:.2}", cli.transition);
        println!();
    }

    // Validate required arguments when not listing keys
    let input_path = cli.input.as_ref().ok_or("Input file is required")?;
    let output_path = cli.output.as_ref().ok_or("Output file is required")?;

    // Open input file
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();

    if cli.verbose {
        println!("📁 Input File Info:");
        println!("   Sample Rate: {}Hz", spec.sample_rate);
        println!("   Channels: {}", spec.channels);
        println!("   Bit Depth: {}", spec.bits_per_sample);
        println!("   Duration: {:.2}s", reader.duration() as f32 / spec.sample_rate as f32);
        println!();
    }

    // Validate audio format
    if spec.channels != 1 && spec.channels != 2 {
        return Err(format!(
            "Unsupported channel count: {}. Only mono and stereo are supported.",
            spec.channels
        )
        .into());
    }

    if spec.bits_per_sample != 16 && spec.bits_per_sample != 24 && spec.bits_per_sample != 32 {
        return Err(format!(
            "Unsupported bit depth: {}. Only 16, 24, and 32-bit are supported.",
            spec.bits_per_sample
        )
        .into());
    }

    // Read samples
    if cli.verbose {
        println!("📖 Reading audio data...");
    }

    let samples: Result<Vec<i32>, _> = reader.samples().collect();
    let samples = samples?;
    let total_samples = samples.len();

    if cli.verbose {
        println!("   Read {} samples", total_samples);
    }

    // Convert to f32
    let scale_factor = match spec.bits_per_sample {
        16 => 1.0 / 32768.0,
        24 => 1.0 / 8388608.0,
        32 => 1.0 / 2147483648.0,
        _ => return Err("Unsupported bit depth".into()),
    };

    let audio_data: Vec<f32> = samples.iter().map(|&x| x as f32 * scale_factor).collect();

    // Convert stereo to mono if needed
    let mono_data = if spec.channels == 2 {
        if cli.verbose {
            println!("🔄 Converting stereo to mono...");
        }
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

    // Configure autotune
    let config = AutotuneConfig {
        fft_size: cli.fft_size,
        hop_size: cli.hop_size,
        sample_rate: spec.sample_rate as f32,
        pitch_correction_strength: cli.strength,
        transition_speed: cli.transition,
        ..Default::default()
    };

    let musical_settings =
        MusicalSettings { key: cli.key, note: cli.note, octave: cli.octave, formant: cli.formant };

    if cli.verbose {
        println!("🎛️  Processing Configuration:");
        println!("   FFT Size: {}", config.fft_size);
        println!("   Hop Size: {}", config.hop_size);
        println!("   Sample Rate: {}Hz", config.sample_rate);
        println!();
        println!("🎵 Starting autotune processing...");
    }

    // Process audio
    let start_time = Instant::now();
    let mut autotune_state = AutotuneState::new(config);
    let mut processed_audio = Vec::new();

    let fft_size = config.fft_size;
    let hop_size = config.hop_size;
    let total_chunks = (mono_data.len() + hop_size - 1) / hop_size;

    let mut input_buffer = vec![0.0f32; fft_size];
    let mut output_buffer = vec![0.0f32; fft_size];
    let mut sample_pos = 0;
    let mut chunk_count = 0;

    while sample_pos + fft_size <= mono_data.len() {
        // Fill input buffer
        input_buffer.copy_from_slice(&mono_data[sample_pos..sample_pos + fft_size]);

        // Process with autotune
        match process_autotune(
            &input_buffer,
            &mut output_buffer,
            &mut autotune_state,
            &musical_settings,
        ) {
            Ok(_) => {
                // Overlap-add
                if processed_audio.len() < sample_pos + fft_size {
                    processed_audio.resize(sample_pos + fft_size, 0.0);
                }

                for (i, &sample) in output_buffer.iter().enumerate() {
                    processed_audio[sample_pos + i] += sample;
                }
            }
            Err(e) => {
                eprintln!("Warning: Processing error at sample {}: {:?}", sample_pos, e);
                // Fallback to original
                if processed_audio.len() < sample_pos + fft_size {
                    processed_audio.resize(sample_pos + fft_size, 0.0);
                }
                for (i, &sample) in input_buffer.iter().enumerate() {
                    processed_audio[sample_pos + i] += sample;
                }
            }
        }

        sample_pos += hop_size;
        chunk_count += 1;

        if cli.verbose && chunk_count % 100 == 0 {
            let progress = (chunk_count as f32 / total_chunks as f32) * 100.0;
            println!("   Progress: {:.1}%", progress);
        }
    }

    // Handle remaining samples
    if sample_pos < mono_data.len() {
        let remaining = mono_data.len() - sample_pos;
        input_buffer.fill(0.0);
        input_buffer[..remaining].copy_from_slice(&mono_data[sample_pos..]);

        if let Ok(_) = process_autotune(
            &input_buffer,
            &mut output_buffer,
            &mut autotune_state,
            &musical_settings,
        ) {
            if processed_audio.len() < sample_pos + remaining {
                processed_audio.resize(sample_pos + remaining, 0.0);
            }
            for i in 0..remaining {
                processed_audio[sample_pos + i] += output_buffer[i];
            }
        }
    }

    // Normalize to prevent clipping
    if cli.verbose {
        println!("🔧 Normalizing audio...");
    }

    let max_amplitude = processed_audio.iter().map(|&x| x.abs()).fold(0.0, f32::max);
    if max_amplitude > 1.0 {
        let scale = 0.95 / max_amplitude;
        for sample in &mut processed_audio {
            *sample *= scale;
        }
        if cli.verbose {
            println!("   Applied normalization: {:.2}x", scale);
        }
    }

    // Convert back to stereo if needed
    let output_samples = if spec.channels == 2 {
        if cli.verbose {
            println!("🔄 Converting back to stereo...");
        }
        let mut stereo = Vec::with_capacity(processed_audio.len() * 2);
        for sample in processed_audio {
            stereo.push(sample); // Left
            stereo.push(sample); // Right
        }
        stereo
    } else {
        processed_audio
    };

    // Convert back to integer format
    if cli.verbose {
        println!("💾 Writing output file...");
    }

    let output_scale = match spec.bits_per_sample {
        16 => 32767.0,
        24 => 8388607.0,
        32 => 2147483647.0,
        _ => return Err("Unsupported bit depth".into()),
    };

    let integer_samples: Vec<i32> =
        output_samples.iter().map(|&x| (x * output_scale).round() as i32).collect();

    // Write output file
    let mut writer = WavWriter::create(output_path, spec)?;
    for &sample in &integer_samples {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;

    let duration = start_time.elapsed();

    if cli.verbose {
        println!();
        println!("✅ Processing complete!");
        println!(
            "   Processed {} samples in {:.2}s",
            integer_samples.len(),
            duration.as_secs_f32()
        );
        println!("   Output saved to: {}", output_path.display());

        let original_duration =
            total_samples as f32 / (spec.sample_rate * spec.channels as u32) as f32;
        let processing_ratio = original_duration / duration.as_secs_f32();
        println!("   Processing speed: {:.1}x real-time", processing_ratio);
    } else {
        println!(
            "Autotune processing complete: {} -> {}",
            input_path.display(),
            output_path.display()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_names_length() {
        assert_eq!(KEY_NAMES.len(), 24);
    }

    #[test]
    fn test_cli_parsing() {
        use clap::Parser;

        // Test basic parsing
        let args = vec!["autotune-cli", "-i", "input.wav", "-o", "output.wav"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());
    }
}
