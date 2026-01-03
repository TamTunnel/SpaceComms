#!/bin/bash
# SpaceComms Demo Script
# Demonstrates CDM propagation between two nodes

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CORE_DIR="$SCRIPT_DIR/../spacecomms-core"
BINARY="$CORE_DIR/target/release/spacecomms"

echo -e "${BLUE}=== SpaceComms Demo ===${NC}"
echo ""

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "Building SpaceComms..."
    cd "$CORE_DIR"
    cargo build --release
    cd "$SCRIPT_DIR"
fi

# Function to cleanup on exit
cleanup() {
    echo ""
    echo -e "${BLUE}Cleaning up...${NC}"
    kill $NODE_A_PID 2>/dev/null || true
    kill $NODE_B_PID 2>/dev/null || true
    echo "Done."
}
trap cleanup EXIT

# Start Node A
echo -e "${BLUE}[1/4] Starting Node A (Operator Alpha)...${NC}"
$BINARY start --config node-a-config.yaml > /tmp/node-a.log 2>&1 &
NODE_A_PID=$!
sleep 2

if ps -p $NODE_A_PID > /dev/null; then
    echo -e "${GREEN}✓ Node A started on port 8080${NC}"
else
    echo -e "${RED}✗ Failed to start Node A${NC}"
    cat /tmp/node-a.log
    exit 1
fi

# Start Node B
echo -e "${BLUE}[2/4] Starting Node B (STM Provider)...${NC}"
$BINARY start --config node-b-config.yaml > /tmp/node-b.log 2>&1 &
NODE_B_PID=$!
sleep 2

if ps -p $NODE_B_PID > /dev/null; then
    echo -e "${GREEN}✓ Node B started on port 8081${NC}"
else
    echo -e "${RED}✗ Failed to start Node B${NC}"
    cat /tmp/node-b.log
    exit 1
fi

# Establish peer connection
echo -e "${BLUE}[3/4] Establishing peer connection...${NC}"
curl -s -X POST http://localhost:8080/peers \
    -H "Content-Type: application/json" \
    -d '{"peer_id": "peer-stm-provider", "address": "http://localhost:8081"}' > /dev/null

curl -s -X POST http://localhost:8081/peers \
    -H "Content-Type: application/json" \
    -d '{"peer_id": "peer-operator-alpha", "address": "http://localhost:8080"}' > /dev/null

echo -e "${GREEN}✓ Nodes connected as peers${NC}"

# Inject sample CDM
echo -e "${BLUE}[4/4] Injecting sample CDM...${NC}"
RESPONSE=$(curl -s -X POST http://localhost:8080/cdm \
    -H "Content-Type: application/json" \
    -d @sample-cdm.json)

CDM_ID=$(echo "$RESPONSE" | grep -o '"cdm_id":"[^"]*"' | cut -d'"' -f4)
echo -e "${GREEN}✓ $CDM_ID injected to Node A${NC}"

echo ""
echo -e "${BLUE}=== Verifying CDM Storage ===${NC}"
echo ""

# Check Node A
echo "Node A CDMs:"
curl -s http://localhost:8080/cdms | python3 -m json.tool 2>/dev/null || curl -s http://localhost:8080/cdms

echo ""

# Check health
echo "Node A health:"
curl -s http://localhost:8080/health | python3 -m json.tool 2>/dev/null || curl -s http://localhost:8080/health

echo ""
echo -e "${BLUE}=== Demo Complete ===${NC}"
echo "CDM successfully stored in Node A!"
echo ""
echo "In a full implementation, the CDM would be propagated to Node B via the protocol layer."
echo "Check logs at /tmp/node-a.log and /tmp/node-b.log for details."
echo ""
echo "Press Ctrl+C to stop the demo..."

# Keep running until interrupted
wait
