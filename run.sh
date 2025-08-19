#!/bin/bash

# Desktop Autotune Application Runner
# This script generates test files and runs the GUI application

set -e

echo "🎵 Desktop Autotune Application Setup"
echo "======================================"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo is not installed. Please install Rust and Cargo."
    exit 1
fi

# Generate test WAV files if they don't exist
if [ ! -f "test_audio.wav" ] || [ ! -f "test_audio_off_pitch.wav" ]; then
    echo "🎼 Generating test WAV files..."
    cargo run --example generate_test_wav
    echo "✅ Test files generated successfully!"
    echo ""
fi

# Build and run the application
echo "🚀 Building and launching the Desktop Autotune Application..."
echo ""
echo "Instructions:"
echo "1. Use 'test_audio.wav' or 'test_audio_off_pitch.wav' as input files"
echo "2. Select an output location for the processed audio"
echo "3. Adjust musical settings and autotune parameters"
echo "4. Click 'Start Processing' to autotune your audio"
echo ""
echo "The off-pitch test file will demonstrate the autotune effect more dramatically."
echo ""

# Run the GUI application
cargo run --release --bin autotune-gui

echo ""
echo "👋 Thanks for using the Desktop Autotune Application!"
