use hound::{WavSpec, WavWriter};
use std::f32::consts::PI;
use std::path::Path;

/// Generate a test WAV file with multiple tones for testing autotune
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create("test_audio.wav", spec)?;

    let sample_rate = spec.sample_rate as f32;
    let duration = 10.0; // seconds
    let total_samples = (duration * sample_rate) as usize;

    // Generate a sequence of musical notes
    let notes = [
        261.63, // C4
        293.66, // D4
        329.63, // E4
        349.23, // F4
        392.00, // G4
        440.00, // A4
        493.88, // B4
        523.25, // C5
    ];

    let note_duration = duration / notes.len() as f32;
    let samples_per_note = (note_duration * sample_rate) as usize;

    println!("Generating test WAV file with {} notes...", notes.len());
    println!("Duration: {:.1}s, Sample rate: {}Hz", duration, sample_rate);

    for (note_idx, &frequency) in notes.iter().enumerate() {
        println!("Generating note {}: {:.2}Hz", note_idx + 1, frequency);

        let start_sample = note_idx * samples_per_note;
        let end_sample = ((note_idx + 1) * samples_per_note).min(total_samples);

        for i in start_sample..end_sample {
            let t = i as f32 / sample_rate;

            // Generate a sine wave with some harmonics for more realistic sound
            let fundamental = (2.0 * PI * frequency * t).sin();
            let second_harmonic = 0.3 * (2.0 * PI * frequency * 2.0 * t).sin();
            let third_harmonic = 0.15 * (2.0 * PI * frequency * 3.0 * t).sin();

            let sample = fundamental + second_harmonic + third_harmonic;

            // Add envelope to prevent clicks
            let note_progress = (i - start_sample) as f32 / (end_sample - start_sample) as f32;
            let envelope = if note_progress < 0.1 {
                note_progress / 0.1
            } else if note_progress > 0.9 {
                (1.0 - note_progress) / 0.1
            } else {
                1.0
            };

            let final_sample = sample * envelope * 0.7; // Scale to prevent clipping
            let amplitude = (final_sample * 32767.0) as i16;
            writer.write_sample(amplitude)?;
        }
    }

    writer.finalize()?;
    println!("Test WAV file 'test_audio.wav' generated successfully!");

    // Generate a second test file with off-pitch notes for dramatic autotune effect
    generate_off_pitch_test()?;

    Ok(())
}

fn generate_off_pitch_test() -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create("test_audio_off_pitch.wav", spec)?;

    let sample_rate = spec.sample_rate as f32;
    let duration = 8.0; // seconds
    let total_samples = (duration * sample_rate) as usize;

    // Generate notes that are slightly off-pitch for dramatic autotune effect
    let base_notes = [
        261.63, // C4
        293.66, // D4
        329.63, // E4
        349.23, // F4
        392.00, // G4
        440.00, // A4
        493.88, // B4
        523.25, // C5
    ];

    // Add random pitch deviation (±30 cents)
    let off_pitch_notes: Vec<f32> = base_notes
        .iter()
        .enumerate()
        .map(|(i, &freq)| {
            let cents_deviation = if i % 2 == 0 { 25.0 } else { -25.0 }; // Alternate sharp/flat
            freq * (2.0_f32).powf(cents_deviation / 1200.0)
        })
        .collect();

    let note_duration = duration / off_pitch_notes.len() as f32;
    let samples_per_note = (note_duration * sample_rate) as usize;

    println!("Generating off-pitch test WAV file...");

    for (note_idx, &frequency) in off_pitch_notes.iter().enumerate() {
        let deviation = ((frequency / base_notes[note_idx] - 1.0) * 1200.0).round();
        println!("Note {}: {:.2}Hz ({:+.0} cents)", note_idx + 1, frequency, deviation);

        let start_sample = note_idx * samples_per_note;
        let end_sample = ((note_idx + 1) * samples_per_note).min(total_samples);

        for i in start_sample..end_sample {
            let t = i as f32 / sample_rate;

            // Generate a more complex waveform to simulate vocal characteristics
            let fundamental = (2.0 * PI * frequency * t).sin();
            let second_harmonic = 0.4 * (2.0 * PI * frequency * 2.0 * t).sin();
            let third_harmonic = 0.2 * (2.0 * PI * frequency * 3.0 * t).sin();
            let fourth_harmonic = 0.1 * (2.0 * PI * frequency * 4.0 * t).sin();

            // Add some vibrato for realism
            let vibrato_rate = 4.5; // Hz
            let vibrato_depth = 0.02;
            let vibrato = 1.0 + vibrato_depth * (2.0 * PI * vibrato_rate * t).sin();

            let sample =
                (fundamental + second_harmonic + third_harmonic + fourth_harmonic) * vibrato;

            // Add envelope
            let note_progress = (i - start_sample) as f32 / (end_sample - start_sample) as f32;
            let envelope = if note_progress < 0.15 {
                note_progress / 0.15
            } else if note_progress > 0.85 {
                (1.0 - note_progress) / 0.15
            } else {
                1.0
            };

            let final_sample = sample * envelope * 0.6;
            let amplitude = (final_sample * 32767.0) as i16;
            writer.write_sample(amplitude)?;
        }
    }

    writer.finalize()?;
    println!("Off-pitch test WAV file 'test_audio_off_pitch.wav' generated successfully!");
    println!("This file will demonstrate the autotune effect more dramatically.");

    Ok(())
}
