#!/bin/bash

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

show_usage() {
    echo "Usage: $0 [IMAGE_TYPE] [COMMAND] [OPTIONS]"
    echo ""
    echo "Image Types:"
    echo "  basic        Use basic processor (mock ML)"
    echo "  pytorch      Use PyTorch-enabled processor"
    echo "  onnx         Use ONNX-enabled processor"
    echo "  web          Use web interface"
    echo ""
    echo "Commands:"
    echo "  single       Process single video (input.mp4)"
    echo "  batch        Process all videos in input_videos/"
    echo "  shell        Open interactive shell in container"
    echo "  web          Start web interface (web image only)"
    echo ""
    echo "Examples:"
    echo "  $0 basic single                    # Process single video with basic processor"
    echo "  $0 pytorch batch                   # Batch process with PyTorch"
    echo "  $0 onnx batch                      # Batch process with ONNX"
    echo "  $0 web web                         # Start web interface"
    echo "  $0 basic shell                     # Open shell in basic container"
}

# Function to run container
run_container() {
    local image_type=$1
    local command=$2
    shift 2
    local extra_args="$@"
    
    local image_name=""
    local gpu_args=""
    local port_args=""
    
    case $image_type in
        "basic")
            image_name="video-audio-processor:latest"
            ;;
        "pytorch")
            image_name="video-audio-processor:pytorch"
            gpu_args="--gpus all"
            ;;
        "onnx")
            image_name="video-audio-processor:onnx"
            ;;
        "web")
            image_name="video-audio-processor:web"
            port_args="-p 8080:8080"
            ;;
        *)
            echo "Unknown image type: $image_type"
            show_usage
            exit 1
            ;;
    esac
    
    # Check if image exists
    if ! docker image inspect "$image_name" &> /dev/null; then
        print_warning "Image $image_name not found. Building it now..."
        case $image_type in
            "basic")
                docker build -f Dockerfile -t "$image_name" .
                ;;
            "pytorch")
                docker build -f Dockerfile.pytorch -t "$image_name" .
                ;;
            "onnx")
                docker build -f Dockerfile.onnx -t "$image_name" .
                ;;
            "web")
                docker build -f Dockerfile.web -t "$image_name" .
                ;;
        esac
    fi
    
    # Create directories if they don't exist
    mkdir -p input_videos output_results models
    
    # Prepare volume mounts
    local volume_args="-v $(pwd)/input_videos:/app/input_videos"
    volume_args="$volume_args -v $(pwd)/output_results:/app/output_results"
    volume_args="$volume_args -v $(pwd)/models:/app/models"
    
    # Handle different commands
    case $command in
        "single")
            if [ ! -f "input.mp4" ]; then
                print_warning "input.mp4 not found. Please place your video file as input.mp4"
                exit 1
            fi
            volume_args="$volume_args -v $(pwd)/input.mp4:/app/input.mp4"
            docker run --rm -it $gpu_args $volume_args "$image_name" single $extra_args
            ;;
        "batch")
            if [ ! "$(ls -A input_videos 2>/dev/null)" ]; then
                print_warning "input_videos directory is empty. Please add video files to process."
                exit 1
            fi
            docker run --rm -it $gpu_args $volume_args "$image_name" batch $extra_args
            ;;
        "shell")
            docker run --rm -it $gpu_args $volume_args --entrypoint /bin/bash "$image_name"
            ;;
        "web")
            if [ "$image_type" != "web" ]; then
                print_warning "Web command only works with web image type"
                exit 1
            fi
            print_info "Starting web interface..."
            print_success "Web interface will be available at: http://localhost:8080"
            docker run --rm -it $port_args $volume_args "$image_name" web $extra_args
            ;;
        *)
            echo "Unknown command: $command"
            show_usage
            exit 1
            ;;
    esac
}

# Main script
if [ $# -lt 2 ]; then
    show_usage
    exit 1
fi

run_container "$@"
