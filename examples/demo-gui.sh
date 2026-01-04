#!/bin/bash
# SpaceComms GUI Demo
#
# Starts the SpaceComms node and opens the web UI dashboard

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║       SpaceComms GUI Demo                                 ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Build
echo "[1/3] Building SpaceComms..."
cd "$PROJECT_ROOT"
cargo build --release -p spacecomms 2>/dev/null || {
    echo "Build failed. Make sure Rust is installed."
    exit 1
}

# Start node
echo "[2/3] Starting SpaceComms node..."
./target/release/spacecomms start --config examples/config.yaml &
NODE_PID=$!
sleep 2

# Cleanup
cleanup() {
    echo ""
    echo "Shutting down..."
    kill $NODE_PID 2>/dev/null || true
    echo "Done."
}
trap cleanup EXIT

# Serve UI files
echo "[3/3] Starting UI server..."
cd "$PROJECT_ROOT/ui"

# Try to use Python's built-in HTTP server
if command -v python3 &> /dev/null; then
    echo ""
    echo "═══════════════════════════════════════════════════════════"
    echo "SpaceComms is running!"
    echo ""
    echo "  Node API:  http://localhost:8080"
    echo "  Dashboard: http://localhost:3000"
    echo ""
    echo "Open http://localhost:3000 in your browser to see the dashboard."
    echo "Press Ctrl+C to stop."
    echo "═══════════════════════════════════════════════════════════"
    echo ""
    python3 -m http.server 3000
else
    echo ""
    echo "═══════════════════════════════════════════════════════════"
    echo "SpaceComms is running at http://localhost:8080"
    echo ""
    echo "To view the dashboard, open ui/index.html in a browser"
    echo "or run: python3 -m http.server 3000"
    echo ""
    echo "Press Ctrl+C to stop."
    echo "═══════════════════════════════════════════════════════════"
    wait
fi
