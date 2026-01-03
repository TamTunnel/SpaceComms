# SpaceComms Protocol

**An open, CDM-centric, BGP-like protocol for space traffic coordination**

[![CI](https://github.com/your-org/spacecomms/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/spacecomms/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

---

## What is SpaceComms?

SpaceComms is a vendor-neutral protocol enabling interoperable exchange of:

- **Conjunction Data Messages (CDMs)** - collision risk warnings
- **Object state/ephemeris data** - satellite positions and trajectories
- **Maneuver intent and status** - planned orbital adjustments

Think of it as **BGP for space traffic management** – nodes peer with each other to share routing information about space objects and conjunction events.

### Key Features

- 🛰️ **CCSDS-aligned** - CDM format compatible with CCSDS 508.0-B-1
- 🔗 **Peer-to-peer** - Decentralized mesh topology, no single point of failure
- 🔌 **Pluggable adapters** - Easy integration with existing infrastructure
- 📡 **Protocol-first** - Clear message specifications for interoperability
- 🔒 **Security-ready** - Hooks for mTLS and token-based authentication

---

## Quick Start

### Prerequisites

- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Build and Run

```bash
# Clone the repository
git clone https://github.com/your-org/spacecomms.git
cd spacecomms

# Build the core service
cd spacecomms-core
cargo build --release

# Start a node with example config
cargo run -- start --config ../examples/config.yaml
```

### Run the Demo

```bash
cd examples
./demo.sh
```

This starts two SpaceComms nodes and demonstrates CDM propagation between them.

---

## Documentation

| Document                                                   | Audience     | Description                                      |
| ---------------------------------------------------------- | ------------ | ------------------------------------------------ |
| [Executive Overview](docs/overview-exec.md)                | Executives   | Plain-language benefits and deployment scenarios |
| [Architecture](docs/architecture.md)                       | Architects   | Technical design and component diagrams          |
| [Protocol Specification](docs/protocol-spec.md)            | Developers   | Message formats and routing model                |
| [API Reference](docs/api-reference.md)                     | Developers   | REST endpoints and schemas                       |
| [Operations Runbook](docs/operations-and-runbook.md)       | Operations   | Deployment, monitoring, troubleshooting          |
| [Regulatory Compliance](docs/regulatory-and-compliance.md) | Legal/Policy | Standards alignment and FAQ                      |
| [Demo Guide](docs/demo-guide.md)                           | Anyone       | Step-by-step demo walkthrough                    |

---

## Project Structure

```
SpaceComms/
├── spacecomms-core/        # Core protocol service (Rust)
├── spacecomms-adapters/    # Integration adapters
│   ├── space-track-mock/   # Mock Space-Track API
│   └── constellation-hub-mock/  # Mock constellation ops
├── examples/               # Demo scripts and sample data
├── tests/                  # Integration tests
├── docs/                   # Documentation
└── .github/workflows/      # CI/CD
```

---

## Protocol Overview

SpaceComms nodes connect as peers and exchange messages:

```
┌─────────────┐                    ┌─────────────┐
│   Node A    │◄──── HELLO ───────►│   Node B    │
│ (Operator)  │                    │ (STM Prov)  │
│             │── CDM_ANNOUNCE ───►│             │
│             │◄─ CDM_ANNOUNCE ────│             │
│             │── MANEUVER_INTENT ►│             │
└─────────────┘                    └─────────────┘
```

**Message Types:**

- `HELLO` - Capability negotiation
- `OBJECT_STATE_ANNOUNCE/WITHDRAW` - Object tracking updates
- `CDM_ANNOUNCE/WITHDRAW` - Conjunction data
- `MANEUVER_INTENT/STATUS` - Orbital maneuver coordination
- `HEARTBEAT` - Connection health

See [Protocol Specification](docs/protocol-spec.md) for details.

---

## Contributing

We welcome contributions! Please:

1. Read the [agent.md](agent.md) for development guidelines
2. Open issues for bugs or feature requests
3. Submit PRs with tests and documentation updates

---

## License

Licensed under Apache 2.0. See [LICENSE](LICENSE) for details.

---

## Disclaimer

This is a reference implementation using **mock data only**. It does not integrate with proprietary catalogs or classified data sources. Operators must layer their own compliance requirements as appropriate for their jurisdiction and operational context.
