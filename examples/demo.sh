#!/bin/bash
# SpaceComms Multi-Service Demo
# 
# This script starts all demo services and demonstrates CDM propagation
# across SpaceComms nodes with adapter integration.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║       SpaceComms Multi-Service Demo                       ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Build all components
echo -e "${YELLOW}[1/6] Building components...${NC}"
cd "$PROJECT_ROOT"
cargo build --release --workspace 2>/dev/null || {
    echo -e "${RED}Build failed. Make sure Rust is installed.${NC}"
    exit 1
}

# Start Space-Track Mock
echo -e "${YELLOW}[2/6] Starting Space-Track Mock on port 9000...${NC}"
cd "$PROJECT_ROOT/spacecomms-adapters/space-track-mock"
PORT=9000 cargo run --release &
SPACE_TRACK_PID=$!
sleep 2

# Start SpaceComms Node A
echo -e "${YELLOW}[3/6] Starting SpaceComms Node A on port 8080...${NC}"
cd "$PROJECT_ROOT"
./target/release/spacecomms start --config examples/config.yaml &
NODE_A_PID=$!
sleep 2

# Start SpaceComms Node B
echo -e "${YELLOW}[4/6] Starting SpaceComms Node B on port 8081...${NC}"
cat > /tmp/node-b-config.yaml << EOF
node:
  id: "node-b-demo"
  name: "SpaceComms Node B"

server:
  host: "0.0.0.0"
  port: 8081

peers: []

security:
  enable_tls: false

logging:
  level: "info"
  format: "pretty"
EOF
./target/release/spacecomms start --config /tmp/node-b-config.yaml &
NODE_B_PID=$!
sleep 2

# Start Constellation Hub Mock
echo -e "${YELLOW}[5/6] Starting Constellation Hub Mock on port 9001...${NC}"
cd "$PROJECT_ROOT/spacecomms-adapters/constellation-hub-mock"
PORT=9001 SPACECOMMS_URL=http://localhost:8080 cargo run --release &
CONSTELLATION_PID=$!
sleep 3

# Cleanup function
cleanup() {
    echo -e "\n${YELLOW}Shutting down services...${NC}"
    kill $SPACE_TRACK_PID 2>/dev/null || true
    kill $NODE_A_PID 2>/dev/null || true
    kill $NODE_B_PID 2>/dev/null || true
    kill $CONSTELLATION_PID 2>/dev/null || true
    rm -f /tmp/node-b-config.yaml
    echo -e "${GREEN}All services stopped.${NC}"
}
trap cleanup EXIT

# Demo flow
echo -e "${YELLOW}[6/6] Running demo flow...${NC}"
echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}All services are running:${NC}"
echo -e "  • Space-Track Mock:       ${BLUE}http://localhost:9000${NC}"
echo -e "  • SpaceComms Node A:      ${BLUE}http://localhost:8080${NC}"
echo -e "  • SpaceComms Node B:      ${BLUE}http://localhost:8081${NC}"
echo -e "  • Constellation Hub Mock: ${BLUE}http://localhost:9001${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# Fetch CDMs from Space-Track mock and inject into SpaceComms
echo -e "${YELLOW}Fetching CDMs from Space-Track Mock...${NC}"
CDM=$(curl -s http://localhost:9000/cdms | jq '.[0]')
echo -e "${GREEN}Got CDM: $(echo $CDM | jq -r .cdm_id)${NC}"

echo ""
echo -e "${YELLOW}Injecting CDM into SpaceComms Node A...${NC}"
RESULT=$(curl -s -X POST http://localhost:8080/cdm \
  -H "Content-Type: application/json" \
  -d "$CDM")
echo -e "${GREEN}Result: $(echo $RESULT | jq -r .status)${NC}"

echo ""
echo -e "${YELLOW}Checking Node A CDM list...${NC}"
curl -s http://localhost:8080/cdms | jq '.total, .cdms[].cdm_id'

echo ""
echo -e "${YELLOW}Checking Constellation Hub alerts (satellite STARLINK-1234 is registered)...${NC}"
sleep 5  # Wait for poller
curl -s http://localhost:9001/alerts | jq '.total, .unacknowledged, .alerts[].severity'

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}Demo complete! Services are still running.${NC}"
echo -e "${GREEN}Press Ctrl+C to stop all services.${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# Keep running until interrupted
wait
