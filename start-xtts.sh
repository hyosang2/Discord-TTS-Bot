#!/bin/bash

# Native XTTS Server Startup Script for Apple Silicon M4 Pro
# This script starts the XTTS API server using the conda environment

set -e

echo "🎤 Starting Native XTTS Server on Apple Silicon M4 Pro..."

# Check if conda is available
if ! command -v conda &> /dev/null; then
    echo "❌ Error: conda is not available. Please install miniconda or anaconda."
    exit 1
fi

# Source conda initialization
if [ -f "$HOME/.zshrc" ]; then
    source "$HOME/.zshrc"
fi

# Activate the xtts environment
echo "🔄 Activating conda environment: xtts"
eval "$(conda shell.bash hook)"
conda activate xtts

# Set environment variable to agree to Coqui TOS
export COQUI_TOS_AGREED=1

# Check if the environment was activated successfully
if [ "$CONDA_DEFAULT_ENV" != "xtts" ]; then
    echo "❌ Error: Failed to activate xtts environment"
    exit 1
fi

# Verify xtts-api-server is installed
if ! python -c "import xtts_api_server" 2>/dev/null; then
    echo "❌ Error: xtts-api-server is not installed in the xtts environment"
    echo "Run: conda activate xtts && pip install xtts-api-server"
    exit 1
fi

# Set up paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VOICE_CLIPS_DIR="$SCRIPT_DIR/xtts_voice_clips"
OUTPUT_DIR="$SCRIPT_DIR/output"

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

echo "📁 Voice clips directory: $VOICE_CLIPS_DIR"
echo "📁 Output directory: $OUTPUT_DIR"

# Start the XTTS server
echo "🚀 Starting XTTS API Server..."
echo "   - Host: 0.0.0.0"
echo "   - Port: 8000"
echo "   - Device: CPU (Apple Silicon M4 Pro)"
echo "   - Voice clips: $VOICE_CLIPS_DIR"
echo "   - Caching: Enabled"
echo ""
echo "💡 Server will be available at: http://localhost:8000"
echo "💡 Press Ctrl+C to stop the server"
echo ""

# Start the server with optimized settings for M4 Pro
python -m xtts_api_server \
    --host 0.0.0.0 \
    --port 8000 \
    --device cpu \
    --speaker-folder "$VOICE_CLIPS_DIR" \
    --output "$OUTPUT_DIR" \
    --use-cache \
    --model-source api \
    --listen

echo "✅ XTTS server stopped"