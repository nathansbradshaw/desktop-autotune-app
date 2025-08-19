# Desktop Autotune Application

A comprehensive desktop solution for autotuning WAV audio files using the `synthphone_vocals` real-time autotune library. Available in both GUI and command-line versions.

![Desktop Autotune App](https://img.shields.io/badge/Rust-GUI-orange?style=flat&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![CLI Support](https://img.shields.io/badge/CLI-supported-green)

## Features

### GUI Application (`autotune-gui`)
- 🎵 **Intuitive GUI** - Easy-to-use desktop interface built with egui
- 📁 **File Processing** - Load and process WAV audio files with drag-and-drop
- 🎼 **Musical Controls** - Select keys, modes, and musical parameters
- ⚙️ **Autotune Parameters** - Adjust pitch correction strength and transition speed
- 📊 **Real-time Progress** - Track processing progress with visual indicators
- 🔄 **Background Processing** - Non-blocking audio processing in separate threads
- 📈 **Audio Info Display** - View file information (duration, sample rate, channels)

### CLI Application (`autotune-cli`)
- 💻 **Command Line Interface** - Perfect for batch processing and automation
- 🔧 **Scriptable** - Easy integration with audio production pipelines
- ⚡ **Fast Processing** - Optimized for headless operation
- 📋 **Verbose Output** - Detailed processing information and statistics
- 🎛️ **Full Control** - Access to all autotune parameters via command line

### Audio Support
- 💾 **Format Support** - Handles 16, 24, and 32-bit WAV files
- 🔊 **Stereo/Mono** - Supports both mono and stereo audio files
- 📏 **Any Sample Rate** - Works with 44.1kHz, 48kHz, and other standard rates
- 🔄 **Format Preservation** - Maintains original bit depth and channel configuration

## Quick Start

### GUI Version
```bash
./run.sh
```
Or manually:
```bash
cargo run --bin autotune-gui
```

### CLI Version
```bash
# Basic usage
cargo run --bin autotune-cli -- -i input.wav -o output.wav

# With custom settings
cargo run --bin autotune-cli -- -i input.wav -o output.wav -k 5 -s 0.9 -t 0.2 --verbose

# List available keys
cargo run --bin autotune-cli -- --list-keys
```

## Interface Overview

### GUI Application
The GUI provides an intuitive interface with clear sections for:
- **File Selection**: Input/output file selection with audio info display
- **Musical Settings**: Key selection, note mode, octave, and formant shift
- **Autotune Parameters**: Pitch correction strength and transition speed
- **Processing Controls**: Start button with real-time progress tracking
- **Status Display**: Processing status and completion notifications

## Installation

### Prerequisites

- Rust (1.70 or later)
- Operating System: Windows, macOS, or Linux

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd synthphone_vocals/desktop-autotune-app
```

2. Build both applications:
```bash
cargo build --release
```

3. Run the GUI application:
```bash
cargo run --release --bin autotune-gui
```

4. Or run the CLI application:
```bash
cargo run --release --bin autotune-cli -- --help
```

## Usage

### GUI Application Quick Start

1. **Generate test files** (optional):
   ```bash
   ./run.sh
   ```

2. **Launch the GUI**:
   ```bash
   cargo run --bin autotune-gui
   ```

3. **Process audio**:
   - Click "Select Input WAV" and choose your audio file
   - Click "Select Output Path" and choose where to save
   - Adjust musical settings (key, octave, formant shift)
   - Set autotune parameters (strength, transition speed)
   - Click "🚀 Start Processing" and wait for completion

### CLI Application Usage

#### Basic Commands

```bash
# Process with default settings
autotune-cli -i input.wav -o output.wav

# Show help
autotune-cli --help

# List available musical keys
autotune-cli --list-keys
```

#### Advanced Examples

```bash
# Strong autotune effect in C Major
autotune-cli -i vocals.wav -o tuned.wav -k 0 -s 1.0 -t 0.05

# Gentle correction in A Minor with formant preservation
autotune-cli -i vocals.wav -o tuned.wav -k 12 -s 0.5 -t 0.2 -f 0

# Verbose processing with custom octave
autotune-cli -i vocals.wav -o tuned.wav -k 5 --octave 3 --verbose

# Batch processing script
for file in *.wav; do
    autotune-cli -i "$file" -o "tuned_$file" -k 0 -s 0.8
done
```

#### CLI Parameters

- `-i, --input <FILE>`: Input WAV file path
- `-o, --output <FILE>`: Output WAV file path  
- `-k, --key <KEY>`: Musical key (0-23, use --list-keys to see all)
- `-n, --note <NOTE>`: Note mode (0=Auto, 1-12=specific note)
- `--octave <OCTAVE>`: Octave reference (0-4, default: 2)
- `-f, --formant <SEMITONES>`: Formant shift (-12 to +12)
- `-s, --strength <STRENGTH>`: Pitch correction strength (0.0-1.0, default: 0.8)
- `-t, --transition <SPEED>`: Transition speed (0.01-1.0, default: 0.1)
- `--fft-size <SIZE>`: FFT size (default: 1024)
- `--hop-size <SIZE>`: Hop size (default: 256)
- `-v, --verbose`: Enable verbose output
- `--list-keys`: Show available keys and exit

### Configuration Guide

#### Musical Settings

**Key Selection (0-23)**:
- **Major Keys**: 0=C, 1=G, 2=D, 3=A, 4=E, 5=B, 6=F#, 7=C#, 8=F, 9=Bb, 10=Eb, 11=Ab
- **Minor Keys**: 12=Am, 13=Em, 14=Bm, 15=F#m, 16=C#m, 17=G#m, 18=D#m, 19=A#m, 20=Dm, 21=Gm, 22=Cm, 23=Fm

**Note Mode**:
- **0 (Auto)**: Snaps to nearest note in selected key (recommended)
- **1-12**: Forces specific note (advanced users)

**Octave Reference**:
- **0-4**: Reference octave for pitch detection (2 is typical for vocals)

**Formant Shift**:
- **-12 to +12**: Preserves vocal timbre while changing pitch
- **0**: No formant adjustment (default)
- **Positive**: Higher formants (more childlike)  
- **Negative**: Lower formants (more mature)

#### Autotune Parameters

**Pitch Correction Strength (0.0-1.0)**:
- **0.0**: No correction (bypass mode)
- **0.3-0.5**: Subtle correction (natural sound)
- **0.8**: Strong correction (default, good balance)
- **1.0**: Maximum correction ("T-Pain" effect)

**Transition Speed (0.01-1.0)**:
- **0.01-0.05**: Very slow pitch changes (robotic)
- **0.1**: Natural pitch changes (default)
- **0.5**: Quick pitch changes
- **1.0**: Instant pitch snapping

### Supported Audio Formats

- **Container**: WAV files only
- **Bit Depth**: 16-bit, 24-bit, 32-bit
- **Channels**: Mono or Stereo  
- **Sample Rates**: Any standard rate (44.1kHz, 48kHz, 96kHz, etc.)

### Musical Keys Available

**Major Keys (0-11)**:
- C, G, D, A, E, B, F#, C#, F, Bb, Eb, Ab Major

**Minor Keys (12-23)**:
- A, E, B, F#, C#, G#, D#, A#, D, G, C, F Minor

## Technical Details

### Audio Processing Pipeline

1. **File Loading**: WAV file is loaded and validated
2. **Format Conversion**: Audio is converted to 32-bit float, stereo is downmixed to mono
3. **Chunk Processing**: Audio is processed in overlapping frames (1024 samples with 256 sample hop)
4. **Autotune Processing**: Each chunk is processed using FFT-based pitch detection and correction
5. **Overlap-Add**: Processed chunks are combined using overlap-add synthesis
6. **Normalization**: Output is normalized to prevent clipping
7. **Format Restoration**: Audio is converted back to original format and saved

### Performance Characteristics

- **FFT Size**: 1024 samples (fixed)
- **Hop Size**: 256 samples (4x overlap)
- **Latency**: ~21ms at 48kHz (suitable for file processing)
- **Memory Usage**: ~16KB for autotune state + file buffers
- **Processing Speed**: Typically faster than real-time

## Configuration Options

### Autotune Parameters

- **Pitch Correction Strength** (0.0-1.0):
  - 0.0: No correction (bypass)
  - 0.5: Moderate correction
  - 1.0: Maximum correction (robot voice effect)

- **Transition Speed** (0.01-1.0):
  - 0.01: Very slow pitch changes
  - 0.1: Natural pitch changes (default)
  - 1.0: Instant pitch snapping

### Musical Settings

- **Key**: Determines which notes are considered "correct"
- **Note Mode**: Currently "Auto" (snaps to nearest note in selected key)
- **Octave**: Reference octave for pitch detection
- **Formant Shift**: Preserves vocal character while changing pitch

## Troubleshooting

### Common Issues

**"Failed to open input file"**
- Ensure the file is a valid WAV file
- Check file permissions
- Try a different audio file

**"Unsupported channel count" or "Unsupported bit depth"**
- Convert your audio file to a supported format:
  - Channels: 1 (mono) or 2 (stereo)
  - Bit depth: 16, 24, or 32-bit

**Processing errors during autotune**
- The application will fall back to the original audio for problematic chunks
- Check the console output for detailed error messages
- Try reducing pitch correction strength

**Out of memory errors**
- Large audio files may require significant memory
- Consider processing shorter segments
- Ensure sufficient system RAM

### Debug Mode

Run with debug logging enabled:
```bash
RUST_LOG=debug cargo run --release
```

## Development

### Project Structure

```
src/
├── main.rs              # Main application and GUI
├── audio_processor.rs   # Audio processing logic
└── ui.rs               # UI helper functions
```

### Dependencies

- `synthphone_vocals`: Core autotune library
- `eframe`/`egui`: GUI framework
- `hound`: WAV file I/O
- `rfd`: File dialogs
- `tokio`: Async runtime

### Running Tests

```bash
cargo test
```

### Building for Distribution

```bash
# Release build with optimizations
cargo build --release

# The executable will be in target/release/
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with the `synthphone_vocals` real-time autotune library
- GUI created using the excellent `egui` immediate-mode GUI framework
- Audio file handling powered by the `hound` crate

## Support

For issues and questions:
1. Check the troubleshooting section above
2. Search existing GitHub issues
3. Create a new issue with detailed information about your problem

---

## Examples

### Generate Test Files

```bash
# Generate test WAV files for experimentation
cargo run --example generate_test_wav

# This creates:
# - test_audio.wav (in-tune notes)
# - test_audio_off_pitch.wav (off-pitch notes for dramatic effect)
```

### Common Use Cases

**Vocal Correction**:
```bash
# Natural vocal correction
autotune-cli -i vocals.wav -o corrected.wav -k 0 -s 0.5 -t 0.15

# Strong "robot voice" effect  
autotune-cli -i vocals.wav -o robot.wav -k 0 -s 1.0 -t 0.01
```

**Instrument Tuning**:
```bash
# Guitar tuning in E Major
autotune-cli -i guitar.wav -o tuned_guitar.wav -k 4 -s 0.7
```

**Creative Effects**:
```bash
# Formant-shifted vocals (chipmunk effect)
autotune-cli -i vocals.wav -o chipmunk.wav -k 0 -s 0.8 -f 12

# Deep voice effect
autotune-cli -i vocals.wav -o deep.wav -k 0 -s 0.8 -f -8
```

## Performance Benchmarks

Typical processing speeds on modern hardware:

- **44.1kHz Mono**: ~10-20x real-time
- **48kHz Stereo**: ~8-15x real-time  
- **Memory Usage**: ~50-100MB peak
- **CPU Usage**: Single-threaded, moderate

Example: A 3-minute song typically processes in 10-20 seconds.

**Note**: This application is designed for file processing. For real-time autotune applications, consider using the embedded version of the `synthphone_vocals` library directly.