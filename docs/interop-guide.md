# SpaceComms Interoperability Guide

This guide describes how to build a SpaceComms-compatible node in any programming language (e.g., Go, Java, Python). It defines the minimum requirements for participating in a SpaceComms network.

## Implementation Requirements

To interoperate with the SpaceComms reference implementation, a node MUST:

1.  **Transport**: Support HTTP/2 over TLS 1.3 (or cleartext HTTP for development).
2.  **Protocol**: Implement the [Protocol Specification](protocol-spec.md) version 1.0.
3.  **Serialization**: Parse and generate JSON messages adhering to the defined schemas.
4.  **Handshake**: Perform the `HELLO` exchange and negotiate protocol version.

## Minimum Endpoints

A minimal implementation must expose the following HTTP POST endpoints:

| Endpoint                  | Description                                                       |
| ------------------------- | ----------------------------------------------------------------- |
| `/spacecomms/v1/messages` | Main receiver for all protocol messages (CDM, Object State, etc.) |

_Note: The reference implementation separates some concerns, but the specification allows for a single message ingestion endpoint if preferred, provided it handles the message types correctly._ (The reference implementation currently uses `/cdm`, `/peers` etc for management API, but peer-to-peer traffic is intended to be unified. _Correction_: The current reference implementation exposes specific endpoints like `/cdms` for ingestion. For _peer-to-peer_ replication, it uses the standardized message passing. _Clarification for implementers_: Stick to the wire format described below found in the Protocol Spec).

### Wire Format

All communication between nodes happens via standard HTTP POST requests containing JSON bodies.

## Message Examples

### 1. HELLO (Handshake)

Required to establish a session.

```json
{
  "protocol_version": "1.0",
  "message_id": "msg-hello-001",
  "timestamp": "2024-01-15T14:30:00.000Z",
  "source_node_id": "node-python-impl",
  "message_type": "HELLO",
  "hop_count": 0,
  "ttl": 1,
  "payload": {
    "node_name": "Python Implementation Node",
    "capabilities": ["CDM", "OBJECT_STATE"],
    "supported_versions": ["1.0", "1.1"],
    "protocol_version": "1.0"
  }
}
```

### 2. CDM_ANNOUNCE

Propagating a Conjunction Data Message.

```json
{
  "protocol_version": "1.0",
  "message_id": "msg-cdm-123",
  "timestamp": "2024-01-15T14:35:00.000Z",
  "source_node_id": "node-python-impl",
  "message_type": "CDM_ANNOUNCE",
  "hop_count": 0,
  "ttl": 10,
  "payload": {
    "cdm": {
       "cdm_id": "CDM-2024-TEST",
       "creation_date": "2024-01-15T14:00:00.000Z",
       "originator": "TEST-PROVIDER",
       "message_for": "TEST-OPERATOR",
       "tca": "2024-01-16T12:00:00.000Z",
       "miss_distance_m": 500.0,
       "collision_probability": 0.001,
       "object1": { ... },
       "object2": { ... }
    }
  }
}
```

## Interop Test Recipe

To verify your implementation against the Rust reference node:

### 1. Run the Reference Node

```bash
# In shell A
cd examples
./demo.sh
# (Or just run one node: cargo run -- start --config examples/config.yaml)
```

### 2. Point Your Node at It

Configure your implementation to treat `http://localhost:8080` (or whatever port the reference node uses) as a peer.

### 3. Verification Checklist

- [ ] **Handshake Success**: Valid `HELLO` response received from reference node.
- [ ] **Heartbeat**: Reference node sends `HEARTBEAT` messages periodically (if session maintained).
- [ ] **CDM Exchange**:
  - Inject a CDM into your node.
  - Reference node receives and validates it (check reference node logs).
  - Inject a CDM into reference node (via its CLI/API).
  - Your node receives the `CDM_ANNOUNCE`.

## Conformance Levels

See the [Protocol Specification](protocol-spec.md#conformance-levels) for definitions of Level 0, Level 1, and Level 2 support.
