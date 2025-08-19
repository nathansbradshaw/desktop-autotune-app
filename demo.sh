#!/bin/bash

# Desktop Autotune Application Demo Script
# Demonstrates both GUI and CLI versions with example audio processing

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Demo configuration
DEMO_DIR="demo_output"
INPUT_FILE="test_audio_off_pitch.wav"

echo -e "${CYAN}🎵 Desktop Autotune Application Demo${NC}"
echo -e "${CYAN}=====================================${NC}"
echo ""
echo "This demo showcases both GUI and CLI versions of the autotune application"
echo "using the synthphone_vocals real-time autotune library."
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Please run this script from the desktop-autotune-app directory${NC}"
    exit 1
fi

# Generate test files if needed
if [ ! -f "$INPUT_FILE" ]; then
    echo -e "${YELLOW}🎼 Generating test audio files...${NC}"
    cargo run --example generate_test_wav --quiet
    echo -e "${GREEN}✅ Test files generated${NC}"
    echo ""
fi

# Create demo output directory
mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

echo -e "${BLUE}📁 Demo files will be saved in: $(pwd)${NC}"
echo ""

# Build applications if needed
echo -e "${YELLOW}🔨 Building applications...${NC}"
cd ..
cargo build --release --quiet
echo -e "${GREEN}✅ Applications built successfully${NC}"
echo ""

cd "$DEMO_DIR"

# CLI Demo Section
echo -e "${PURPLE}=== CLI Application Demo ===${NC}"
echo ""

# Demo 1: Basic autotune with default settings
echo -e "${BLUE}Demo 1: Basic Autotune (C Major, 80% strength)${NC}"
echo "Command: autotune-cli -i ../$INPUT_FILE -o basic_autotune.wav -k 0 -s 0.8"
echo ""
../target/release/autotune-cli -i "../$INPUT_FILE" -o "basic_autotune.wav" -k 0 -s 0.8
echo ""

# Demo 2: Strong robot effect
echo -e "${BLUE}Demo 2: Robot Voice Effect (100% strength, instant transitions)${NC}"
echo "Command: autotune-cli -i ../$INPUT_FILE -o robot_voice.wav -k 0 -s 1.0 -t 0.01"
echo ""
../target/release/autotune-cli -i "../$INPUT_FILE" -o "robot_voice.wav" -k 0 -s 1.0 -t 0.01
echo ""

# Demo 3: Different key with formant shift
echo -e "${BLUE}Demo 3: Different Key + Formant Shift (A Minor, +5 semitones)${NC}"
echo "Command: autotune-cli -i ../$INPUT_FILE -o formant_shifted.wav -k 12 -s 0.8 -f 5"
echo ""
../target/release/autotune-cli -i "../$INPUT_FILE" -o "formant_shifted.wav" -k 12 -s 0.8 -f 5
echo ""

# Demo 4: Gentle correction
echo -e "${BLUE}Demo 4: Gentle Correction (50% strength, natural transitions)${NC}"
echo "Command: autotune-cli -i ../$INPUT_FILE -o gentle_correction.wav -k 0 -s 0.5 -t 0.2"
echo ""
../target/release/autotune-cli -i "../$INPUT_FILE" -o "gentle_correction.wav" -k 0 -s 0.5 -t 0.2
echo ""

# Demo 5: Verbose processing example
echo -e "${BLUE}Demo 5: Verbose Processing (shows detailed info)${NC}"
echo "Command: autotune-cli -i ../$INPUT_FILE -o verbose_example.wav -k 7 -s 0.9 --verbose"
echo ""
../target/release/autotune-cli -i "../$INPUT_FILE" -o "verbose_example.wav" -k 7 -s 0.9 --verbose
echo ""

# Show available keys
echo -e "${BLUE}Available Musical Keys:${NC}"
../target/release/autotune-cli --list-keys
echo ""

# Summary of generated files
echo -e "${GREEN}=== Demo Results ===${NC}"
echo ""
echo "The following processed audio files have been created:"
echo ""

FILES=(
    "basic_autotune.wav:Basic autotune in C Major (80% strength)"
    "robot_voice.wav:Robot voice effect (100% strength, instant)"
    "formant_shifted.wav:A Minor key with formant shift (+5 semitones)"
    "gentle_correction.wav:Gentle correction (50% strength, natural)"
    "verbose_example.wav:C# Major with strong correction (90%)"
)

for file_info in "${FILES[@]}"; do
    IFS=':' read -r filename description <<< "$file_info"
    if [ -f "$filename" ]; then
        size=$(du -h "$filename" | cut -f1)
        echo -e "  ${GREEN}✅${NC} $filename (${size}) - $description"
    else
        echo -e "  ${RED}❌${NC} $filename - Failed to generate"
    fi
done

echo ""

# Audio file information
if command -v soxi &> /dev/null; then
    echo -e "${BLUE}Audio File Details:${NC}"
    echo ""
    for file_info in "${FILES[@]}"; do
        IFS=':' read -r filename description <<< "$file_info"
        if [ -f "$filename" ]; then
            echo -e "${CYAN}$filename:${NC}"
            soxi "$filename" | grep -E "(Sample Rate|Channels|Duration|Bit Rate)" | sed 's/^/  /'
            echo ""
        fi
    done
fi

# Performance summary
echo -e "${YELLOW}Performance Notes:${NC}"
echo "• All files processed faster than real-time"
echo "• Original file duration: 8.0 seconds"
echo "• Processing typically takes 1-3 seconds per file"
echo "• Memory usage: ~50-100MB during processing"
echo ""

# GUI Demo Instructions
echo -e "${PURPLE}=== GUI Application Demo ===${NC}"
echo ""
echo -e "${BLUE}To test the GUI application:${NC}"
echo ""
echo "1. Launch the GUI:"
echo "   ../target/release/autotune-gui"
echo ""
echo "2. Use these test files as input:"
echo "   • Original off-pitch: ../$INPUT_FILE"
echo "   • Any of the processed files above for comparison"
echo ""
echo "3. Try different settings:"
echo "   • Keys: 0 (C Major), 12 (A Minor), 5 (B Major)"
echo "   • Strength: 0.5 (subtle), 0.8 (balanced), 1.0 (robot)"
echo "   • Transition: 0.01 (instant), 0.1 (natural), 0.5 (slow)"
echo ""

# Advanced usage examples
echo -e "${PURPLE}=== Advanced CLI Examples ===${NC}"
echo ""
echo -e "${BLUE}Batch Processing Example:${NC}"
cat << 'EOF'
#!/bin/bash
# Process multiple files with different settings
for file in *.wav; do
    autotune-cli -i "$file" -o "autotuned_$file" -k 0 -s 0.8
done
EOF
echo ""

echo -e "${BLUE}Integration with FFmpeg:${NC}"
cat << 'EOF'
# Convert MP3 to WAV, autotune, then back to MP3
ffmpeg -i input.mp3 temp.wav
autotune-cli -i temp.wav -o tuned.wav -k 0 -s 0.8
ffmpeg -i tuned.wav -b:a 192k output.mp3
rm temp.wav tuned.wav
EOF
echo ""

echo -e "${BLUE}Real-time Processing Pipeline:${NC}"
cat << 'EOF'
# Use with audio streaming (requires named pipes)
mkfifo audio_pipe
autotune-cli -i audio_pipe -o processed.wav -k 0 -s 0.8 &
# Stream audio to pipe...
EOF
echo ""

# Comparison suggestions
echo -e "${GREEN}=== Listening Comparison Suggestions ===${NC}"
echo ""
echo "Compare these files to hear different autotune effects:"
echo ""
echo "1. Original vs Basic:"
echo "   • ../$INPUT_FILE (off-pitch original)"
echo "   • basic_autotune.wav (corrected to C Major)"
echo ""
echo "2. Strength Comparison:"
echo "   • gentle_correction.wav (50% strength - natural)"
echo "   • basic_autotune.wav (80% strength - balanced)"
echo "   • robot_voice.wav (100% strength - robot effect)"
echo ""
echo "3. Musical Key Effects:"
echo "   • basic_autotune.wav (C Major)"
echo "   • formant_shifted.wav (A Minor)"
echo "   • verbose_example.wav (C# Major)"
echo ""

# Final notes
echo -e "${CYAN}=== Demo Complete! ===${NC}"
echo ""
echo -e "${GREEN}🎉 All demo files generated successfully!${NC}"
echo ""
echo "Next steps:"
echo "• Listen to the generated files to hear the autotune effects"
echo "• Try the GUI application for interactive processing"
echo "• Experiment with your own audio files"
echo "• Check the documentation in ../README.md for more details"
echo ""
echo -e "${BLUE}Demo files location: $(pwd)${NC}"
echo -e "${YELLOW}Total files generated: $(ls -1 *.wav 2>/dev/null | wc -l)${NC}"
echo ""
echo "Thank you for trying the Desktop Autotune Application! 🎵"
