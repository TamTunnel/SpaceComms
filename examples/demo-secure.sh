#!/bin/bash
# SpaceComms Secure Demo (mTLS)
#
# Demonstrates secure peer communication with mutual TLS authentication.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║       SpaceComms Secure Demo (mTLS)                       ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Check for certificates
if [ ! -f "$PROJECT_ROOT/dev-certs/ca.crt" ]; then
    echo "Certificates not found. Generating..."
    cd "$PROJECT_ROOT/dev-certs"
    chmod +x generate-certs.sh
    ./generate-certs.sh
    cd "$PROJECT_ROOT"
fi

# Build
echo "[1/4] Building SpaceComms..."
cd "$PROJECT_ROOT"
cargo build --release -p spacecomms 2>/dev/null || {
    echo "Build failed. Make sure Rust is installed."
    exit 1
}

# Start Node A with TLS
echo "[2/4] Starting Secure Node A on port 8443..."
./target/release/spacecomms start --config examples/node-a-tls-config.yaml &
NODE_A_PID=$!
sleep 3

# Start Node B with TLS
echo "[3/4] Starting Secure Node B on port 8444..."
./target/release/spacecomms start --config examples/node-b-tls-config.yaml &
NODE_B_PID=$!
sleep 3

# Cleanup
cleanup() {
    echo ""
    echo "Shutting down secure nodes..."
    kill $NODE_A_PID 2>/dev/null || true
    kill $NODE_B_PID 2>/dev/null || true
    echo "Done."
}
trap cleanup EXIT

echo "[4/4] Testing secure connections..."
echo ""
echo "═══════════════════════════════════════════════════════════"
echo "Secure nodes running:"
echo "  Node A (TLS): https://localhost:8443"
echo "  Node B (TLS): https://localhost:8444"
echo ""
echo "Test with client certificate:"
echo ""
echo "  curl --cacert dev-certs/ca.crt \\"
echo "       --cert dev-certs/client.crt \\"
echo "       --key dev-certs/client.key \\"
echo "       https://localhost:8443/health"
echo ""
echo "What to look for in logs:"
echo "  - 'TLS enabled' message at startup"
echo "  - 'Secure peer session established' when peering"
echo ""
echo "Press Ctrl+C to stop."
echo "═══════════════════════════════════════════════════════════"
echo ""

# Keep running
wait
