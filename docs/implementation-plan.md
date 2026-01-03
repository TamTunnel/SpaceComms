# SpaceComms Implementation Plan

_Project goals, phases, and technical approach_

---

## Purpose

SpaceComms is a neutral, open, CDM-centric protocol for exchanging:

- **Conjunction Data Messages (CDMs)** - Collision risk warnings
- **Ephemeris/OCM-style data** - Satellite positions and trajectories
- **Maneuver intent and status** - Orbital adjustment coordination

### Non-Goals

- Full flight dynamics simulation
- Proprietary conjunction screening algorithms
- Classified data handling
- Real-time telemetry streaming

---

## Stakeholders

| Stakeholder                                 | Interest                                                |
| ------------------------------------------- | ------------------------------------------------------- |
| **Satellite Operators**                     | Receive timely collision warnings, coordinate maneuvers |
| **STM Providers** (SDA, TraCSS, Kayhan)     | Distribute CDMs, receive maneuver feedback              |
| **Regulators** (ESA, NASA, FAA)             | Monitor conjunction activity, ensure compliance         |
| **Standards Bodies** (CCSDS, ISO TC20/SC14) | Protocol standardization pathway                        |
| **Open Source Community**                   | Reference implementation, interoperability testing      |

---

## Phases

### Phase 1: Local Node + CDM Exchange

**Scope**: Single-node operation with CDM ingestion, storage, and basic peering.

**Deliverables**:

- Core protocol message definitions
- CDM parser and validator (CCSDS 508.0-B-1 aligned)
- In-memory storage layer
- REST API for CDM operations
- Single peer connection (manual configuration)
- CLI for node operation

**Interfaces**:

- REST API: `/cdm`, `/cdms`, `/health`
- Protocol: HELLO, CDM_ANNOUNCE, HEARTBEAT

**Testing**:

- Unit tests for message encoding/decoding
- CDM validation tests
- API integration tests

**Demo**: Start node, inject CDM via API, query CDMs

---

### Phase 2: Multi-Node Peering

**Scope**: Multiple nodes connecting as peers with routing policies.

**Deliverables**:

- Peer management (add/remove via API)
- Session management (HELLO exchange, heartbeat)
- Message routing engine
- Configurable routing policies
- Message loop prevention
- Authentication hooks (token-based)

**Interfaces**:

- REST API: `/peers`
- Protocol: All message types
- Policies: Accept/reject filters, TTL enforcement

**Testing**:

- Multi-node integration tests
- CDM propagation scenarios
- Routing policy tests
- Auth/unauthorized tests

**Demo**: Two nodes exchanging CDMs, CDM withdrawal

---

### Phase 3: Hardening & Adapters

**Scope**: Production readiness, external integrations.

**Deliverables**:

- TLS/mTLS support
- Audit logging
- File-based persistence
- Space-Track mock adapter
- Constellation Hub mock adapter
- Performance tuning
- Container packaging

**Interfaces**:

- Adapter trait for pluggable integrations
- Enhanced health/metrics endpoints

**Testing**:

- Security testing
- Performance benchmarks
- Adapter integration tests

**Demo**: Full demo with adapters, multi-party scenario

---

## Risk Assessment

| Risk                         | Likelihood | Impact | Mitigation                              |
| ---------------------------- | ---------- | ------ | --------------------------------------- |
| **Data licensing issues**    | Medium     | High   | Mock data only, clear disclaimers       |
| **CDM format deviations**    | Medium     | Medium | Schema validation, CCSDS reference      |
| **Adoption resistance**      | Medium     | Medium | Clear docs, simple demo, open source    |
| **Latency concerns**         | Low        | Medium | Async protocol design, note limitations |
| **Security vulnerabilities** | Low        | High   | Standard crypto, security review        |

---

## Related Documents

- [Architecture](architecture.md) - Technical design
- [Protocol Specification](protocol-spec.md) - Message formats
- [Demo Guide](demo-guide.md) - Demonstration walkthrough
