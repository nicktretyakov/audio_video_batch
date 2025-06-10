# 🎬 Video-Audio Processor

A high-performance Rust application for comprehensive video and audio analysis using machine learning. Extract frames, detect objects, transcribe speech, and synchronize results with timestamp precision.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)

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

## 🚀 Quick Start

### Option 1: Automated Setup (Recommended)
\`\`\`bash
# Clone the repository
git clone https://github.com/your-username/video-audio-processor.git
cd video-audio-processor

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
\`\`\`

### Option 2: Docker (Zero Setup)
\`\`\`bash
# Quick start with Docker
git clone https://github.com/your-username/video-audio-processor.git
cd video-audio-processor

# Setup and build
./docker-setup.sh
./docker-build.sh all

# Process videos
./docker-run.sh basic batch
\`\`\`

### Option 3: Manual Installation
\`\`\`bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libavformat-dev libavcodec-dev libavutil-dev libswscale-dev

# Install dependencies (macOS)
brew install ffmpeg

# Build and run
cargo build --features mock-ml
cargo run batch
\`\`\`

## 📋 Prerequisites

### System Requirements
- **OS**: Linux, macOS, Windows
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 2GB free space
- **CPU**: Multi-core recommended for batch processing

### Dependencies
- **Rust**: 1.75.0 or later
- **FFmpeg**: Development libraries
- **Optional**: CUDA for GPU acceleration

### Quick Dependency Check
\`\`\`bash
# Check if everything is ready
./build-with-fallback.sh check
\`\`\`

## 🛠️ Installation

### 1. Install Rust
\`\`\`bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
\`\`\`

### 2. Install FFmpeg Development Libraries

#### Ubuntu/Debian
\`\`\`bash
sudo apt-get update
sudo apt-get install libavformat-dev libavcodec-dev libavutil-dev libswscale-dev
\`\`\`

#### macOS
\`\`\`bash
brew install ffmpeg
\`\`\`

#### Windows
Download FFmpeg development libraries from [ffmpeg.org](https://ffmpeg.org/download.html)

### 3. Build the Application
\`\`\`bash
# Automatic build with best available features
./build-with-fallback.sh

# Or manually choose features
cargo build --features mock-ml      # Recommended for testing
cargo build --features pytorch      # Requires PyTorch
cargo build --features onnx         # Requires ONNX Runtime
cargo build --features candle       # Rust-native ML
\`\`\`

## 📖 Usage

### Single Video Processing
\`\`\`bash
# Place your video file as input.mp4
cargo run single

# With custom options
cargo run single --confidence 0.7 --save-frames
\`\`\`

### Batch Processing
\`\`\`bash
# Create input directory and add videos
mkdir input_videos
cp /path/to/your/videos/* input_videos/

# Process all videos
cargo run batch

# View results
ls output_results/
\`\`\`

### Docker Usage
\`\`\`bash
# Process with basic backend
./docker-run.sh basic batch

# Process with PyTorch (GPU acceleration)
./docker-run.sh pytorch batch

# Start web interface
./docker-run.sh web web
# Open http://localhost:8080
\`\`\`

### Configuration
\`\`\`bash
# Show configuration options
cargo run batch --config

# Custom configuration
export MAX_CONCURRENT_VIDEOS=8
export CONFIDENCE_THRESHOLD=0.8
cargo run batch
\`\`\`

## 📁 Project Structure

\`\`\`
video-audio-processor/
├── src/
│   ├── main.rs              # Main application entry
│   ├── video_processor.rs   # Video frame extraction
│   ├── frame_analyzer.rs    # ML frame analysis
│   ├── audio_processor.rs   # Audio extraction & transcription
│   ├── ml_backend.rs        # ML backend abstraction
│   ├── batch_processor.rs   # Batch processing logic
│   └── synchronizer.rs      # Result synchronization
├── input_videos/            # Place videos here for batch processing
├── output_results/          # Processing results appear here
├── models/                  # ML model files
├── docker/                  # Docker configurations
├── scripts/                 # Utility scripts
└── docs/                    # Documentation
\`\`\`

## 📊 Output Structure

\`\`\`
output_results/
├── batch_summary.txt        # Overall processing summary
├── video1/
│   ├── frames/             # Extracted frames (optional)
│   ├── audio.aac          # Extracted audio (optional)
│   └── results.json       # Analysis results
└── video2/
    ├── frames/
    ├── audio.aac
    └── results.json
\`\`\`

### Sample Results JSON
\`\`\`json
[
  {
    "timestamp": 1.5,
    "video_objects": [
      {
        "label": "person",
        "confidence": 0.95,
        "bbox": [100, 100, 200, 300]
      }
    ],
    "audio_text": "Hello, this is a sample transcription"
  }
]
\`\`\`

## 🐳 Docker Support

### Available Images
- **Basic**: `video-audio-processor:latest` - Mock ML backend
- **PyTorch**: `video-audio-processor:pytorch` - GPU acceleration
- **ONNX**: `video-audio-processor:onnx` - Cross-platform inference
- **Web**: `video-audio-processor:web` - Web interface

### Docker Commands
\`\`\`bash
# Build all images
./docker-build.sh all

# Run specific backend
./docker-run.sh pytorch batch

# Start with Docker Compose
docker-compose up -d

# View logs
docker-compose logs -f
\`\`\`

## ⚙️ Configuration

### Environment Variables
\`\`\`bash
# Logging
export RUST_LOG=info

# Processing
export MAX_CONCURRENT_VIDEOS=4
export CONFIDENCE_THRESHOLD=0.5

# GPU
export CUDA_VISIBLE_DEVICES=0

# Output
export SAVE_FRAMES=false
export SAVE_AUDIO=false
export OUTPUT_FORMAT=json
\`\`\`

### Configuration File
Create `config.toml`:
\`\`\`toml
[batch]
input_directory = "input_videos"
output_directory = "output_results"
max_concurrent_videos = 4

[ml_models]
confidence_threshold = 0.5
use_gpu = true

[output]
save_frames = false
save_audio = false
output_format = "json"
\`\`\`

## 🔧 ML Backend Comparison

| Backend | Setup Difficulty | Performance | GPU Support | Model Support |
|---------|------------------|-------------|-------------|---------------|
| Mock    | ⭐ Easy         | ⭐⭐⭐ Fast | ❌ No      | ✅ Test Data |
| Candle  | ⭐⭐ Medium     | ⭐⭐ Good   | ✅ Yes     | ⭐⭐ Growing |
| PyTorch | ⭐⭐⭐ Hard     | ⭐⭐⭐ Best | ✅ Yes     | ⭐⭐⭐ Excellent |
| ONNX    | ⭐⭐ Medium     | ⭐⭐ Good   | ✅ Yes     | ⭐⭐⭐ Excellent |

## 🚀 Performance Tips

### For Better Speed
- Use GPU acceleration with PyTorch backend
- Process videos in parallel with batch mode
- Reduce frame extraction rate for faster processing
- Use SSD storage for input/output directories

### For Lower Memory Usage
- Process smaller videos or split large ones
- Use mock backend for testing
- Reduce concurrent video processing count
- Clear output directories regularly

### Docker Optimization
\`\`\`yaml
# docker-compose.yml
services:
  video-processor:
    deploy:
      resources:
        limits:
          memory: 8G
          cpus: '4'
\`\`\`

## 🔍 Troubleshooting

### Common Issues

#### Build Errors
\`\`\`bash
# Try the automated build script
./build-with-fallback.sh

# Check dependencies
./build-with-fallback.sh check
\`\`\`

#### PyTorch Version Mismatch
\`\`\`bash
# Use alternative backend
cargo build --features mock-ml
\`\`\`

#### FFmpeg Not Found
\`\`\`bash
# Ubuntu/Debian
sudo apt-get install libavformat-dev libavcodec-dev libavutil-dev libswscale-dev

# macOS
brew install ffmpeg
\`\`\`

#### Docker Issues
\`\`\`bash
# Check Docker installation
docker --version
docker-compose --version

# Rebuild images
./docker-build.sh clean
./docker-build.sh all
\`\`\`

### Debug Mode
\`\`\`bash
# Enable verbose logging
export RUST_LOG=debug
cargo run batch

# Check system info
./build-with-fallback.sh check
\`\`\`

For more troubleshooting, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).

## 📚 Documentation

- [Installation Guide](docs/INSTALLATION.md)
- [Docker Guide](DOCKER.md)
- [Troubleshooting](TROUBLESHOOTING.md)
- [API Reference](docs/API.md)
- [Contributing](CONTRIBUTING.md)

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup
\`\`\`bash
# Clone and setup
git clone https://github.com/your-username/video-audio-processor.git
cd video-audio-processor

# Install development dependencies
cargo install cargo-watch cargo-audit

# Run tests
cargo test

# Run with auto-reload
cargo watch -x run
\`\`\`

### Adding New ML Backends
1. Implement the `MLBackend` trait in `src/ml_backend.rs`
2. Add feature flag in `Cargo.toml`
3. Update the factory function
4. Add tests and documentation

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [FFmpeg](https://ffmpeg.org/) for video/audio processing
- [PyTorch](https://pytorch.org/) for deep learning capabilities
- [ONNX Runtime](https://onnxruntime.ai/) for cross-platform inference
- [Candle](https://github.com/huggingface/candle) for Rust-native ML
- [Rust Community](https://www.rust-lang.org/community) for excellent ecosystem

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/your-username/video-audio-processor/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/video-audio-processor/discussions)
- **Email**: support@your-domain.com

## 🗺️ Roadmap

### Version 1.1
- [ ] Real-time video stream processing
- [ ] Web-based UI improvements
- [ ] More ML model integrations
- [ ] Performance optimizations

### Version 1.2
- [ ] Cloud deployment support
- [ ] Kubernetes manifests
- [ ] Advanced audio analysis
- [ ] Custom model training

### Version 2.0
- [ ] WebAssembly support
- [ ] Mobile app integration
- [ ] Distributed processing
- [ ] Advanced analytics dashboard

---

<div align="center">

**⭐ Star this repository if you find it useful!**

[Report Bug](https://github.com/your-username/video-audio-processor/issues) · [Request Feature](https://github.com/your-username/video-audio-processor/issues) · [Documentation](docs/)

</div>
