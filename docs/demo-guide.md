# SpaceComms Demo Guide

_Step-by-step walkthrough for demonstrating SpaceComms_

---

## Overview

This guide walks you through a demonstration of SpaceComms showing:

1. Starting multiple SpaceComms nodes
2. Connecting nodes as peers
3. Injecting a Conjunction Data Message (CDM)
4. Watching the CDM propagate between nodes
5. Observing maneuver coordination

**Time required**: ~15 minutes

**Skill level**: No programming required, basic command-line familiarity helpful

---

## Prerequisites

### Option A: Pre-built Binary

Download the latest release:

```bash
# macOS
curl -LO https://github.com/your-org/spacecomms/releases/latest/download/spacecomms-macos
chmod +x spacecomms-macos
mv spacecomms-macos spacecomms

# Linux
curl -LO https://github.com/your-org/spacecomms/releases/latest/download/spacecomms-linux
chmod +x spacecomms-linux
mv spacecomms-linux spacecomms
```

### Option B: Build from Source

```bash
# Requires Rust 1.75+
git clone https://github.com/your-org/spacecomms.git
cd spacecomms/spacecomms-core
cargo build --release
cp target/release/spacecomms ../examples/
```

---

## Demo Script

### Automated Demo

The fastest way to see SpaceComms in action:

```bash
cd examples
./demo.sh
```

This script automatically:

- Starts two SpaceComms nodes
- Establishes peer connection
- Injects a sample CDM
- Shows propagation in real-time

**Expected output**:

```
=== SpaceComms Demo ===

[1/4] Starting Node A (Operator Alpha)...
✓ Node A started on port 8080

[2/4] Starting Node B (STM Provider)...
✓ Node B started on port 8081

[3/4] Establishing peer connection...
✓ Nodes connected as peers

[4/4] Injecting sample CDM...
✓ CDM-2024-DEMO-001 injected to Node A

=== Watching propagation ===

[Node A] CDM received: CDM-2024-DEMO-001
[Node A] Forwarding to peer: node-b
[Node B] CDM received: CDM-2024-DEMO-001
[Node B] Stored CDM for objects: NORAD-12345, NORAD-99999

=== Demo Complete ===
CDM successfully propagated from Node A to Node B!
```

---

### Manual Demo

For a more detailed walkthrough:

#### Step 1: Open Three Terminal Windows

- Terminal 1: Node A (Operator)
- Terminal 2: Node B (STM Provider)
- Terminal 3: Commands

#### Step 2: Start Node A

In Terminal 1:

```bash
cd examples
./spacecomms start --config node-a-config.yaml
```

**What you should see**:

```
INFO  spacecomms::node > Starting SpaceComms node: node-alpha
INFO  spacecomms::node > Listening on 0.0.0.0:8080
INFO  spacecomms::node > Node ready
```

This represents **Operator Alpha's** ground station running SpaceComms.

#### Step 3: Start Node B

In Terminal 2:

```bash
cd examples
./spacecomms start --config node-b-config.yaml
```

**What you should see**:

```
INFO  spacecomms::node > Starting SpaceComms node: node-stm-provider
INFO  spacecomms::node > Listening on 0.0.0.0:8081
INFO  spacecomms::node > Node ready
```

This represents an **STM Provider** (like SDA or a commercial provider) running SpaceComms.

#### Step 4: Check Node Status

In Terminal 3:

```bash
# Check Node A
curl http://localhost:8080/health | jq

# Check Node B
curl http://localhost:8081/health | jq
```

**Expected**: Both nodes report `"status": "healthy"` with 0 peers connected.

#### Step 5: Connect Nodes as Peers

In Terminal 3:

```bash
# Tell Node A to peer with Node B
curl -X POST http://localhost:8080/peers \
  -H "Content-Type: application/json" \
  -d '{
    "peer_id": "peer-stm-provider",
    "address": "http://localhost:8081"
  }'
```

**Watch Terminal 1 (Node A)**:

```
INFO  spacecomms::node::peer > Initiating peer connection to peer-stm-provider
INFO  spacecomms::node::peer > HELLO sent to peer-stm-provider
INFO  spacecomms::node::peer > HELLO received from peer-stm-provider
INFO  spacecomms::node::peer > Peer session established: peer-stm-provider
```

**Watch Terminal 2 (Node B)**:

```
INFO  spacecomms::node::peer > Incoming peer connection from node-alpha
INFO  spacecomms::node::peer > HELLO received from node-alpha
INFO  spacecomms::node::peer > HELLO sent to node-alpha
INFO  spacecomms::node::peer > Peer session established: node-alpha
```

**What this means**: The nodes have exchanged capability information and can now share space traffic data.

#### Step 6: Verify Peer Connection

```bash
curl http://localhost:8080/peers | jq
```

**Expected**:

```json
{
  "peers": [
    {
      "peer_id": "peer-stm-provider",
      "status": "connected",
      "messages_sent": 1,
      "messages_received": 1
    }
  ]
}
```

#### Step 7: Inject a CDM

Now simulate receiving a conjunction warning:

```bash
curl -X POST http://localhost:8080/cdm \
  -H "Content-Type: application/json" \
  -d @sample-cdm.json
```

**Watch Terminal 1 (Node A)**:

```
INFO  spacecomms::cdm > CDM received: CDM-2024-DEMO-001
INFO  spacecomms::cdm > TCA: 2024-01-17T08:30:00Z
INFO  spacecomms::cdm > Miss distance: 150.5m
INFO  spacecomms::cdm > Collision probability: 1.2e-4
INFO  spacecomms::node::routing > CDM accepted, forwarding to 1 peers
INFO  spacecomms::node::routing > CDM_ANNOUNCE sent to peer-stm-provider
```

**Watch Terminal 2 (Node B)**:

```
INFO  spacecomms::node::protocol > CDM_ANNOUNCE received from node-alpha
INFO  spacecomms::cdm > CDM received: CDM-2024-DEMO-001
INFO  spacecomms::cdm > Objects involved: STARLINK-1234, DEBRIS-FRAGMENT
INFO  spacecomms::storage > CDM stored: CDM-2024-DEMO-001
```

**What this means**:

- Operator Alpha's system detected (or received) a conjunction warning
- The CDM was automatically propagated to the STM Provider
- Both nodes now have awareness of this collision risk

#### Step 8: Verify CDM at Both Nodes

```bash
# Check Node A
curl http://localhost:8080/cdms | jq

# Check Node B
curl http://localhost:8081/cdms | jq
```

**Expected**: Both nodes list `CDM-2024-DEMO-001`.

#### Step 9: Announce Maneuver Intent (Optional)

Simulate Operator Alpha deciding to maneuver:

```bash
curl -X POST http://localhost:8080/maneuvers \
  -H "Content-Type: application/json" \
  -d '{
    "object_id": "NORAD-12345",
    "related_cdm_id": "CDM-2024-DEMO-001",
    "planned_start": "2024-01-16T06:00:00.000Z",
    "planned_duration_s": 30,
    "maneuver_type": "COLLISION_AVOIDANCE"
  }'
```

**Watch Terminal 2 (Node B)**:

```
INFO  spacecomms::node::protocol > MANEUVER_INTENT received from node-alpha
INFO  spacecomms::maneuver > Object NORAD-12345 planning maneuver
INFO  spacecomms::maneuver > Related to CDM: CDM-2024-DEMO-001
```

**What this means**: The STM Provider now knows that Operator Alpha plans to avoid the collision. They can update their predictions accordingly.

#### Step 10: Cleanup

```bash
# Stop nodes with Ctrl+C in Terminals 1 and 2
# Or use the cleanup script
./demo-cleanup.sh
```

---

## Understanding the Demo

### What SpaceComms Demonstrates

| Demo Step       | Real-World Equivalent                               |
| --------------- | --------------------------------------------------- |
| Start Node A    | Operator sets up SpaceComms at their ops center     |
| Start Node B    | STM provider deploys SpaceComms for distribution    |
| Peer connection | Organizations agree to share data                   |
| CDM propagation | Collision warning automatically reaches all parties |
| Maneuver intent | Operator transparently shares their response plan   |

### Key Takeaways

1. **Automatic propagation**: No manual forwarding or email chains needed
2. **Standardized format**: Both nodes understand the same CDM structure
3. **Bidirectional**: Either node can originate or receive messages
4. **Audit trail**: All messages logged with timestamps and node IDs
5. **Extensible**: Additional nodes could join the mesh

---

## Troubleshooting

### "Connection refused" errors

**Cause**: Node not running or wrong port

**Fix**: Verify node is running and check port in config

### Nodes don't see each other as peers

**Cause**: Firewall or peer configuration issue

**Fix**:

```bash
# Check if nodes can reach each other
curl http://localhost:8081/health  # from Node A's perspective
```

### CDM not propagating

**Cause**: Routing policy or connection issue

**Fix**: Check logs for routing decisions:

```bash
grep "routing" node-a.log
```

---

## Next Steps

After completing this demo:

1. **Read the docs**: [Architecture](architecture.md), [Protocol Spec](protocol-spec.md)
2. **Try modifications**: Edit `sample-cdm.json` with different values
3. **Add more nodes**: Start a third node and create a mesh
4. **Explore the API**: [API Reference](api-reference.md)
5. **Review operations**: [Operations Runbook](operations-and-runbook.md)

---

## Multi-Service Demo

This demo shows SpaceComms integrated with external adapters.

### Components

- **SpaceComms Node A** (port 8080) - Primary node
- **SpaceComms Node B** (port 8081) - Secondary node
- **Space-Track Mock** (port 9000) - Simulates Space-Track API
- **Constellation Hub Mock** (port 9001) - Simulates constellation operator

### Running the Demo

```bash
cd examples
chmod +x demo.sh
./demo.sh
```

### What Happens

1. Space-Track Mock starts with pre-loaded catalog and CDMs
2. Two SpaceComms nodes start and peer with each other
3. Constellation Hub Mock starts monitoring SpaceComms for CDMs
4. A CDM is fetched from Space-Track and injected into SpaceComms
5. Constellation Hub detects the CDM affects a registered satellite
6. An alert is generated in Constellation Hub

### Manual Steps

```bash
# 1. Fetch available CDMs from Space-Track mock
curl http://localhost:9000/cdms | jq

# 2. Inject a CDM into SpaceComms
curl -X POST http://localhost:8080/cdm \
  -H "Content-Type: application/json" \
  -d "$(curl -s http://localhost:9000/cdms | jq '.[0]')"

# 3. Check alerts in Constellation Hub
curl http://localhost:9001/alerts | jq

# 4. Acknowledge an alert
curl -X POST http://localhost:9001/alerts/{alert-id}/acknowledge
```

---

## GUI Demo

SpaceComms includes a web-based dashboard for visualizing node status and CDM data.

### Running the GUI Demo

```bash
cd examples
chmod +x demo-gui.sh
./demo-gui.sh
```

This starts:

- SpaceComms node on port 8080
- Web UI server on port 3000

### Opening the Dashboard

Open your browser to: **http://localhost:3000**

Or if serving manually:

```bash
cd ui
python3 -m http.server 3000
# Then open http://localhost:3000
```

### Dashboard Features

| Panel           | Description                                 |
| --------------- | ------------------------------------------- |
| **Health**      | Node status, uptime, peer count, CDM count  |
| **Topology**    | Visual representation of connected peers    |
| **Peers Table** | List of connected peers with message counts |
| **CDMs Table**  | Active conjunction warnings with key data   |
| **Metrics**     | Announced/withdrawn CDMs, error counts      |

### Connecting to a Different Node

Use the `?node=` query parameter:

```
http://localhost:3000?node=http://localhost:8081
```

### Demo Flow

1. Start the GUI demo: `./demo-gui.sh`
2. Open http://localhost:3000 in your browser
3. In another terminal, inject a CDM:
   ```bash
   curl -X POST http://localhost:8080/cdm \
     -H "Content-Type: application/json" \
     -d @sample-cdm.json
   ```
4. Watch the dashboard update automatically (refreshes every 5 seconds)

### What to Observe

- **Health panel**: CDM count increases
- **CDMs table**: New CDM appears with object IDs and collision probability
- **Topology**: Shows connected peers (none in single-node demo)

---

## Secure Demo (mTLS)

For demonstrations requiring secure peering, see the mTLS configuration in `examples/node-a-tls-config.yaml`.

### Prerequisites

Generate demo certificates:

```bash
cd dev-certs
./generate-certs.sh
```

### Starting Secure Nodes

```bash
# Node A with TLS
./spacecomms start --config examples/node-a-tls-config.yaml

# Node B with TLS
./spacecomms start --config examples/node-b-tls-config.yaml
```

### Verifying Secure Connection

Look for these log messages:

```
INFO  spacecomms::tls > TLS enabled with mTLS
INFO  spacecomms::peer > Secure peer session established
```

---

## Demo Summary

| Demo          | Command           | Duration | Shows                    |
| ------------- | ----------------- | -------- | ------------------------ |
| Quick CLI     | `./demo.sh` (old) | 5 min    | Basic CDM propagation    |
| Multi-Service | `./demo.sh`       | 10 min   | Full adapter integration |
| GUI           | `./demo-gui.sh`   | 5 min    | Visual dashboard         |
| Secure        | Manual + certs    | 15 min   | mTLS peering             |
