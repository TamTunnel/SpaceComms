# SpaceComms Architecture

_Technical design for software architects and integrators_

---

## System Overview

SpaceComms is a peer-to-peer protocol for exchanging space traffic coordination data. Each participant runs a **node** that connects to other nodes (**peers**) to exchange messages about space objects, conjunction events, and maneuver coordination.

### Design Principles

1. **Protocol-first**: Clear message specifications enable independent implementations
2. **Stateless messaging, stateful nodes**: Each message is self-contained; nodes maintain routing state
3. **Pluggable architecture**: Adapters abstract external data sources
4. **Eventual consistency**: Nodes converge on shared state through announcements
5. **Defense in depth**: Auth, validation, and audit at multiple layers

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              SpaceComms Node                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                  │
│  │   REST API   │    │   Protocol   │    │   Adapters   │                  │
│  │   (HTTP/2)   │    │   Layer      │    │              │                  │
│  │              │    │              │    │ ┌──────────┐ │                  │
│  │ /cdm         │    │ Messages     │    │ │Space-Trk │ │                  │
│  │ /peers       │    │ Routing      │    │ └──────────┘ │                  │
│  │ /objects     │    │ Sessions     │    │ ┌──────────┐ │                  │
│  │ /health      │    │              │    │ │Const Hub │ │                  │
│  └──────┬───────┘    └──────┬───────┘    │ └──────────┘ │                  │
│         │                   │            └──────┬───────┘                  │
│         │                   │                   │                          │
│  ┌──────▼───────────────────▼───────────────────▼───────┐                  │
│  │                      Core Engine                      │                  │
│  │                                                       │                  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │                  │
│  │  │   Storage   │  │  CDM Proc.  │  │   Routing   │   │                  │
│  │  │  (Memory)   │  │  & Valid.   │  │   Engine    │   │                  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘   │                  │
│  └───────────────────────────────────────────────────────┘                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Peer Connections
                                    ▼
                    ┌───────────────────────────────┐
                    │        Other Nodes            │
                    │   (Operators, Providers,      │
                    │    Regulators)                │
                    └───────────────────────────────┘
```

---

## Component Details

### REST API Layer

Exposes HTTP/2 endpoints for external interaction:

| Endpoint        | Method | Purpose                      |
| --------------- | ------ | ---------------------------- |
| `/health`       | GET    | Health check and node status |
| `/cdm`          | POST   | Ingest CDM from local source |
| `/cdms`         | GET    | List active CDMs             |
| `/cdms/{id}`    | GET    | Retrieve specific CDM        |
| `/objects`      | GET    | List tracked space objects   |
| `/objects/{id}` | GET    | Retrieve specific object     |
| `/peers`        | GET    | List configured peers        |
| `/peers`        | POST   | Add new peer                 |
| `/peers/{id}`   | DELETE | Remove peer                  |

**Request/Response Format**: JSON with versioned envelope

### Protocol Layer

Handles peer-to-peer communication:

#### Message Types

```rust
enum MessageType {
    Hello,              // Capability negotiation
    ObjectStateAnnounce,// New/updated object
    ObjectStateWithdraw,// Object no longer tracked
    CdmAnnounce,        // New conjunction data
    CdmWithdraw,        // CDM no longer valid
    ManeuverIntent,     // Planned maneuver
    ManeuverStatus,     // Maneuver execution status
    Heartbeat,          // Connection health
    Error,              // Error response
}
```

#### Session Management

```
┌─────────────┐                              ┌─────────────┐
│   Node A    │                              │   Node B    │
└──────┬──────┘                              └──────┬──────┘
       │                                            │
       │─────────── HELLO (capabilities) ──────────►│
       │                                            │
       │◄────────── HELLO (capabilities) ───────────│
       │                                            │
       │          [Session Established]             │
       │                                            │
       │◄───────────── HEARTBEAT ───────────────────│
       │─────────────── HEARTBEAT ─────────────────►│
       │                                            │
       │            [Session Active]                │
       │                                            │
```

### Core Engine

#### Storage Layer

```rust
trait Storage: Send + Sync {
    fn store_object(&self, obj: ObjectRecord) -> Result<()>;
    fn get_object(&self, id: &str) -> Result<Option<ObjectRecord>>;
    fn list_objects(&self) -> Result<Vec<ObjectRecord>>;

    fn store_cdm(&self, cdm: CdmRecord) -> Result<()>;
    fn get_cdm(&self, id: &str) -> Result<Option<CdmRecord>>;
    fn list_cdms(&self) -> Result<Vec<CdmRecord>>;
    fn withdraw_cdm(&self, id: &str) -> Result<()>;
}
```

Reference implementation uses in-memory storage with file-based persistence hooks.

#### CDM Processing

1. **Parse**: Validate JSON against schema
2. **Normalize**: Convert to internal `CdmRecord`
3. **Validate**: Check required fields, value ranges
4. **Store**: Persist to storage layer
5. **Route**: Forward to peers per routing policy

#### Routing Engine

Inspired by BGP:

```rust
struct RoutingPolicy {
    peer_id: String,
    action: Action,        // Accept, Reject, Modify
    priority: u32,
    filters: Vec<Filter>,  // Object type, source, etc.
}

enum Action {
    Accept,
    Reject,
    ModifyAndAccept { modifications: Vec<Modification> },
}
```

---

## Adapters Architecture

Adapters abstract external data sources and sinks:

```
┌─────────────────────────────────────────────────────────────────┐
│                        Adapter Interface                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  trait Adapter: Send + Sync {                                   │
│      fn name(&self) -> &str;                                    │
│      fn poll_objects(&self) -> Result<Vec<ObjectRecord>>;       │
│      fn poll_cdms(&self) -> Result<Vec<CdmRecord>>;             │
│      fn on_cdm_received(&self, cdm: &CdmRecord);                │
│      fn on_maneuver_intent(&self, intent: &ManeuverIntent);     │
│  }                                                              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
        │                           │                      │
        ▼                           ▼                      ▼
┌───────────────┐         ┌───────────────┐      ┌───────────────┐
│  Space-Track  │         │ Constellation │      │    Custom     │
│     Mock      │         │   Hub Mock    │      │   Adapter     │
│               │         │               │      │               │
│ Static JSON   │         │ Satellite     │      │ Your system   │
│ fixtures      │         │ registry +    │      │ integration   │
│               │         │ maneuver recs │      │               │
└───────────────┘         └───────────────┘      └───────────────┘
```

### Implementing a Custom Adapter

1. Create new crate in `spacecomms-adapters/`
2. Implement `Adapter` trait
3. Register in node configuration
4. Adapter lifecycle managed by core engine

---

## Multi-Node Topology

SpaceComms supports arbitrary mesh topologies:

```
         ┌─────────────┐
         │   STM       │
         │  Provider   │
         └──────┬──────┘
                │
        ┌───────┴───────┐
        │               │
        ▼               ▼
┌─────────────┐   ┌─────────────┐
│  Operator A │   │  Operator B │
│  (100 sats) │   │  (50 sats)  │
└──────┬──────┘   └──────┬──────┘
       │                 │
       │    ┌────────────┘
       │    │
       ▼    ▼
┌─────────────┐
│  Regulator  │
│  (observer) │
└─────────────┘
```

**Routing behavior**:

- CDMs propagate through mesh based on policies
- Loops prevented via message IDs and hop counts
- Nodes can filter what they forward

---

## Technology Choices

### Why Rust?

| Requirement | Rust Advantage                                  |
| ----------- | ----------------------------------------------- |
| Reliability | Memory safety without GC pauses                 |
| Performance | Zero-cost abstractions, low latency             |
| Concurrency | Async/await with tokio, fearless concurrency    |
| Correctness | Strong type system catches bugs at compile time |
| Adoption    | Growing use in aerospace (NASA, SpaceX tooling) |

### Why HTTP/2?

| Requirement  | HTTP/2 Advantage                         |
| ------------ | ---------------------------------------- |
| Multiplexing | Multiple streams on single connection    |
| Encryption   | Native TLS support                       |
| Tooling      | Excellent debugging and monitoring tools |
| Adoption     | Universal firewall/proxy compatibility   |

### Why JSON?

| Requirement    | JSON Advantage                            |
| -------------- | ----------------------------------------- |
| Human readable | Easy debugging and manual testing         |
| Tooling        | Universal parser availability             |
| Schema support | JSON Schema for validation                |
| Extensibility  | Forward compatibility with unknown fields |

---

## Security Architecture

### Authentication

```
┌─────────────┐                              ┌─────────────┐
│   Node A    │                              │   Node B    │
└──────┬──────┘                              └──────┬──────┘
       │                                            │
       │──────── TLS Handshake (mTLS) ─────────────►│
       │                                            │
       │────── Token in HELLO message ─────────────►│
       │                                            │
       │◄─────── Token validation ──────────────────│
       │                                            │
       │         [Authenticated Session]            │
```

### Authorization

- Per-peer policies control message acceptance
- Object-level filters restrict propagation
- Audit logging for all message exchanges

### Encryption

- TLS 1.3 for transport
- Optional message-level signatures (future)

---

## Observability

### Health Endpoints

```json
GET /health

{
  "status": "healthy",
  "node_id": "node-alpha-01",
  "uptime_seconds": 86400,
  "peers": {
    "connected": 3,
    "total": 5
  },
  "objects_tracked": 1250,
  "cdms_active": 42
}
```

### Logging

Structured JSON logs via `tracing`:

```json
{
  "timestamp": "2024-01-15T14:30:00Z",
  "level": "INFO",
  "target": "spacecomms::node::routing",
  "message": "CDM propagated to peer",
  "cdm_id": "cdm-2024-001234",
  "peer_id": "peer-operator-b",
  "request_id": "req-abc123"
}
```

### Metrics (Future)

Prometheus-compatible metrics endpoint:

- Message counts by type
- Latency histograms
- Peer connection status
- Storage utilization

---

## Deployment Options

### Development

```bash
cargo run -- start --config config.yaml
```

### Container

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/spacecomms /usr/local/bin/
CMD ["spacecomms", "start", "--config", "/etc/spacecomms/config.yaml"]
```

### Production Considerations

- Run behind load balancer for high availability
- Configure persistent storage (database adapter)
- Enable mTLS for peer connections
- Set up monitoring and alerting
- Plan capacity for expected message volume

---

## Extensibility

### Adding New Message Types

1. Define in `protocol/messages.rs`
2. Add codec support
3. Implement handler in routing engine
4. Update peer negotiation (HELLO capabilities)
5. Document in protocol spec

### Adding New Adapters

1. Create crate with `Adapter` trait impl
2. Add to configuration schema
3. Register in node startup
4. Document integration pattern

### Adding New Storage Backends

1. Implement `Storage` trait
2. Add configuration option
3. Document migration path

---

## Related Documents

- [Protocol Specification](protocol-spec.md) - Message formats and schemas
- [API Reference](api-reference.md) - REST endpoint details
- [Operations Runbook](operations-and-runbook.md) - Deployment and troubleshooting
