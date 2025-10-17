# 🎬 Video-Audio Processor

## ✨ Features

### 🎥 Video Processing
- **Frame Extraction**: High-quality frame extraction from any video format
- **Object Detection**: ML-powered object recognition with bounding boxes
- **Batch Processing**: Process multiple videos automatically
- **Format Support**: MP4, AVI, MOV, MKV, WMV, FLV, WebM

### 🎵 Audio Processing
- **Audio Extraction**: Clean audio stream extraction
- **Speech Recognition**: Transcribe spoken content with timestamps
- **Multi-language Support**: Configurable language models
- **Audio Analysis**: Frequency and amplitude analysis

### 🤖 Machine Learning Backends
- **Mock Backend**: Realistic test data for development (default)
- **PyTorch**: Full deep learning with GPU acceleration
- **ONNX Runtime**: Cross-platform ML inference
- **Candle**: Rust-native ML framework

### 🔄 Synchronization & Output
- **Timestamp Sync**: Precise alignment of video and audio analysis
- **Multiple Formats**: JSON, CSV, TXT output options
- **Batch Reports**: Comprehensive processing summaries
- **Real-time Progress**: Live progress tracking


# Run automated setup
chmod +x build-with-fallback.sh
./build-with-fallback.sh

# Process a single video
echo "Place your video as input.mp4"
cargo run single

# Process multiple videos
mkdir input_videos
# Add your videos to input_videos/
cargo run batch

### Option 2: Docker (Zero Setup)
# Setup and build
./docker-setup.sh
./docker-build.sh all

# Process videos
./docker-run.sh basic batch

### Option 3: Manual Installation
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libavformat-dev libavcodec-dev libavutil-dev libswscale-dev

# Install dependencies (macOS)
brew install ffmpeg

# Build and run
cargo build --features mock-ml
cargo run batch

## 📋 Prerequisites

# Or manually choose features
cargo build --features mock-ml      # Recommended for testing
cargo build --features pytorch      # Requires PyTorch
cargo build --features onnx         # Requires ONNX Runtime
cargo build --features candle       # Rust-native ML

# With custom options
cargo run single --confidence 0.7 --save-frames

# Process all videos
cargo run batch

# Process with PyTorch (GPU acceleration)
./docker-run.sh pytorch batch

# Start web interface
./docker-run.sh web web
# Open http://localhost:8080

# Custom configuration
export MAX_CONCURRENT_VIDEOS=8
export CONFIDENCE_THRESHOLD=0.8
cargo run batch



