#!/bin/bash

set -e

echo "🐳 Video-Audio Processor Docker Build Script"
echo "============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    print_error "Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Function to build a specific image
build_image() {
    local dockerfile=$1
    local tag=$2
    local description=$3
    
    print_status "Building $description..."
    
    if docker build -f "$dockerfile" -t "$tag" .; then
        print_success "$description built successfully"
        return 0
    else
        print_error "Failed to build $description"
        return 1
    fi
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  all          Build all Docker images"
    echo "  basic        Build basic image (mock ML)"
    echo "  pytorch      Build PyTorch-enabled image"
    echo "  onnx         Build ONNX-enabled image"
    echo "  web          Build web interface image"
    echo "  compose      Build and start with Docker Compose"
    echo "  clean        Remove all built images"
    echo "  help         Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 all                    # Build all images"
    echo "  $0 basic                  # Build only basic image"
    echo "  $0 compose                # Build and start with compose"
}

# Create necessary directories
create_directories() {
    print_status "Creating necessary directories..."
    mkdir -p input_videos output_results models web/dist
    print_success "Directories created"
}

# Build all images
build_all() {
    print_status "Building all Docker images..."
    
    local success=0
    local total=4
    
    build_image "Dockerfile" "video-audio-processor:latest" "Basic processor" && ((success++))
    build_image "Dockerfile.pytorch" "video-audio-processor:pytorch" "PyTorch processor" && ((success++))
    build_image "Dockerfile.onnx" "video-audio-processor:onnx" "ONNX processor" && ((success++))
    build_image "Dockerfile.web" "video-audio-processor:web" "Web interface" && ((success++))
    
    print_status "Build summary: $success/$total images built successfully"
    
    if [ $success -eq $total ]; then
        print_success "All images built successfully!"
        return 0
    else
        print_warning "Some images failed to build. Check the logs above."
        return 1
    fi
}

# Clean up images
clean_images() {
    print_status "Removing Docker images..."
    
    docker rmi -f video-audio-processor:latest 2>/dev/null || true
    docker rmi -f video-audio-processor:pytorch 2>/dev/null || true
    docker rmi -f video-audio-processor:onnx 2>/dev/null || true
    docker rmi -f video-audio-processor:web 2>/dev/null || true
    
    # Clean up dangling images
    docker image prune -f
    
    print_success "Images cleaned up"
}

# Start with Docker Compose
start_compose() {
    print_status "Building and starting with Docker Compose..."
    
    create_directories
    
    if docker-compose up --build -d; then
        print_success "Services started successfully!"
        print_status "Access the web interface at: http://localhost:8080"
        print_status "View logs with: docker-compose logs -f"
        print_status "Stop services with: docker-compose down"
    else
        print_error "Failed to start services with Docker Compose"
        return 1
    fi
}

# Main script logic
case "${1:-help}" in
    "all")
        create_directories
        build_all
        ;;
    "basic")
        create_directories
        build_image "Dockerfile" "video-audio-processor:latest" "Basic processor"
        ;;
    "pytorch")
        create_directories
        build_image "Dockerfile.pytorch" "video-audio-processor:pytorch" "PyTorch processor"
        ;;
    "onnx")
        create_directories
        build_image "Dockerfile.onnx" "video-audio-processor:onnx" "ONNX processor"
        ;;
    "web")
        create_directories
        build_image "Dockerfile.web" "video-audio-processor:web" "Web interface"
        ;;
    "compose")
        start_compose
        ;;
    "clean")
        clean_images
        ;;
    "help"|*)
        show_usage
        ;;
esac
