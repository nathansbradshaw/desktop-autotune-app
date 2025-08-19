#!/bin/bash

# Desktop Autotune Application Build Script
# Builds both GUI and CLI versions with optimizations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BUILD_DIR="target/release"
DIST_DIR="dist"
VERSION="0.1.0"

echo -e "${BLUE}🎵 Desktop Autotune Application Build Script${NC}"
echo -e "${BLUE}===============================================${NC}"
echo ""

# Check dependencies
echo -e "${YELLOW}🔍 Checking dependencies...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Error: Cargo is not installed. Please install Rust and Cargo.${NC}"
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

if ! command -v pkg-config &> /dev/null; then
    echo -e "${YELLOW}⚠️  Warning: pkg-config not found. Some GUI dependencies might fail.${NC}"
fi

echo -e "${GREEN}✅ Rust toolchain found: $(rustc --version)${NC}"
echo ""

# Clean previous builds
echo -e "${YELLOW}🧹 Cleaning previous builds...${NC}"
cargo clean
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Generate test files if they don't exist
echo -e "${YELLOW}🎼 Generating test files...${NC}"
if [ ! -f "test_audio.wav" ] || [ ! -f "test_audio_off_pitch.wav" ]; then
    cargo run --example generate_test_wav
    echo -e "${GREEN}✅ Test files generated${NC}"
else
    echo -e "${GREEN}✅ Test files already exist${NC}"
fi
echo ""

# Build release versions
echo -e "${YELLOW}🔨 Building release binaries...${NC}"
echo "   This may take several minutes on first build..."

# Build GUI application
echo -e "${BLUE}Building GUI application (autotune-gui)...${NC}"
cargo build --release --bin autotune-gui

# Build CLI application
echo -e "${BLUE}Building CLI application (autotune-cli)...${NC}"
cargo build --release --bin autotune-cli

echo -e "${GREEN}✅ Build completed successfully!${NC}"
echo ""

# Run tests
echo -e "${YELLOW}🧪 Running tests...${NC}"
cargo test --release --quiet
echo -e "${GREEN}✅ All tests passed${NC}"
echo ""

# Copy binaries to dist directory
echo -e "${YELLOW}📦 Packaging distribution...${NC}"

# Determine OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
DIST_NAME="autotune-desktop-${VERSION}-${OS}-${ARCH}"
PACKAGE_DIR="${DIST_DIR}/${DIST_NAME}"

mkdir -p "$PACKAGE_DIR/bin"
mkdir -p "$PACKAGE_DIR/examples"
mkdir -p "$PACKAGE_DIR/docs"

# Copy binaries
cp "${BUILD_DIR}/autotune-gui" "${PACKAGE_DIR}/bin/" 2>/dev/null || cp "${BUILD_DIR}/autotune-gui.exe" "${PACKAGE_DIR}/bin/" 2>/dev/null || true
cp "${BUILD_DIR}/autotune-cli" "${PACKAGE_DIR}/bin/" 2>/dev/null || cp "${BUILD_DIR}/autotune-cli.exe" "${PACKAGE_DIR}/bin/" 2>/dev/null || true

# Copy documentation and examples
cp README.md "${PACKAGE_DIR}/docs/"
cp run.sh "${PACKAGE_DIR}/"
cp test_audio*.wav "${PACKAGE_DIR}/examples/" 2>/dev/null || true

# Create installation script
cat > "${PACKAGE_DIR}/install.sh" << 'EOF'
#!/bin/bash

# Installation script for Desktop Autotune Application

set -e

INSTALL_DIR="$HOME/.local/bin"
CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🎵 Installing Desktop Autotune Application..."
echo "============================================="

# Create install directory
mkdir -p "$INSTALL_DIR"

# Copy binaries
if [ -f "$CURRENT_DIR/bin/autotune-gui" ]; then
    cp "$CURRENT_DIR/bin/autotune-gui" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/autotune-gui"
    echo "✅ GUI application installed to: $INSTALL_DIR/autotune-gui"
fi

if [ -f "$CURRENT_DIR/bin/autotune-cli" ]; then
    cp "$CURRENT_DIR/bin/autotune-cli" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/autotune-cli"
    echo "✅ CLI application installed to: $INSTALL_DIR/autotune-cli"
fi

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  Note: $INSTALL_DIR is not in your PATH."
    echo "   Add this line to your ~/.bashrc or ~/.zshrc:"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
    echo ""
    echo "   Or run applications with full path:"
    echo "   $INSTALL_DIR/autotune-gui"
    echo "   $INSTALL_DIR/autotune-cli"
else
    echo ""
    echo "🚀 Installation complete! You can now run:"
    echo "   autotune-gui    (GUI version)"
    echo "   autotune-cli    (command line version)"
fi

echo ""
echo "📚 Documentation and examples are in:"
echo "   $CURRENT_DIR/docs/"
echo "   $CURRENT_DIR/examples/"
EOF

chmod +x "${PACKAGE_DIR}/install.sh"
chmod +x "${PACKAGE_DIR}/run.sh"

# Create usage instructions
cat > "${PACKAGE_DIR}/USAGE.txt" << 'EOF'
Desktop Autotune Application
============================

This package contains both GUI and CLI versions of the autotune application.

Quick Start:
1. Run ./install.sh to install to your system
2. Or run binaries directly from bin/ directory

GUI Application:
- Run: ./bin/autotune-gui
- Provides intuitive interface for processing audio files
- Real-time progress tracking and parameter adjustment

CLI Application:
- Run: ./bin/autotune-cli --help
- Perfect for automation and batch processing
- Example: ./bin/autotune-cli -i input.wav -o output.wav -k 0 -s 0.8

Test Files:
- Use examples/test_audio_off_pitch.wav to test the autotune effect
- The off-pitch version demonstrates dramatic pitch correction

Documentation:
- See docs/README.md for complete usage guide
- Lists all parameters, keys, and examples

Installation:
- Run ./install.sh for system-wide installation
- Installs to ~/.local/bin (add to PATH if needed)

Support:
- Check the GitHub repository for issues and updates
- See docs/README.md for troubleshooting guide
EOF

echo -e "${GREEN}✅ Distribution packaged in: ${PACKAGE_DIR}${NC}"
echo ""

# Show binary information
echo -e "${YELLOW}📊 Binary Information:${NC}"
if [ -f "${BUILD_DIR}/autotune-gui" ]; then
    GUI_SIZE=$(du -h "${BUILD_DIR}/autotune-gui" | cut -f1)
    echo "   GUI Application: ${GUI_SIZE}"
fi

if [ -f "${BUILD_DIR}/autotune-cli" ]; then
    CLI_SIZE=$(du -h "${BUILD_DIR}/autotune-cli" | cut -f1)
    echo "   CLI Application: ${CLI_SIZE}"
fi

echo ""

# Performance test with CLI
echo -e "${YELLOW}⚡ Running performance test...${NC}"
if [ -f "test_audio_off_pitch.wav" ]; then
    echo "   Processing test file..."
    START_TIME=$(date +%s.%N)
    "${BUILD_DIR}/autotune-cli" -i test_audio_off_pitch.wav -o test_perf_output.wav -k 0 -s 0.8 > /dev/null 2>&1
    END_TIME=$(date +%s.%N)
    DURATION=$(echo "$END_TIME - $START_TIME" | bc -l 2>/dev/null || python3 -c "print(f'{$END_TIME - $START_TIME:.2f}')" 2>/dev/null || echo "N/A")

    if [ "$DURATION" != "N/A" ]; then
        echo -e "${GREEN}   ✅ Processed 8-second audio file in ${DURATION}s${NC}"
        RATIO=$(echo "scale=1; 8.0 / $DURATION" | bc -l 2>/dev/null || python3 -c "print(f'{8.0 / float($DURATION):.1f}')" 2>/dev/null || echo "N/A")
        if [ "$RATIO" != "N/A" ]; then
            echo -e "${GREEN}   🚀 Performance: ${RATIO}x real-time${NC}"
        fi
    else
        echo -e "${GREEN}   ✅ Performance test completed${NC}"
    fi

    rm -f test_perf_output.wav
else
    echo -e "${YELLOW}   ⚠️  No test file available for performance test${NC}"
fi

echo ""

# Final instructions
echo -e "${GREEN}🎉 Build completed successfully!${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "1. Test the applications:"
echo "   ${BUILD_DIR}/autotune-gui"
echo "   ${BUILD_DIR}/autotune-cli --help"
echo ""
echo "2. Install system-wide:"
echo "   cd ${PACKAGE_DIR}"
echo "   ./install.sh"
echo ""
echo "3. Or create distribution archive:"
echo "   cd ${DIST_DIR}"
echo "   tar -czf ${DIST_NAME}.tar.gz ${DIST_NAME}/"
echo ""
echo -e "${YELLOW}📁 Distribution directory: ${PACKAGE_DIR}${NC}"
echo -e "${YELLOW}🔧 Build artifacts: ${BUILD_DIR}/${NC}"
echo ""
echo -e "${GREEN}Happy autotuning! 🎵${NC}"
