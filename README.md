# SpaceComms Protocol

**An open protocol that lets satellite systems share collision warnings and coordinate maneuvers—like air-traffic control radio, but for space.**

[![CI](https://github.com/TamTunnel/SpaceComms/actions/workflows/ci.yml/badge.svg)](https://github.com/TamTunnel/SpaceComms/actions/workflows/ci.yml)
[![Release](https://github.com/TamTunnel/SpaceComms/actions/workflows/release.yml/badge.svg)](https://github.com/TamTunnel/SpaceComms/actions/workflows/release.yml)
[![Deploy Dev](https://github.com/TamTunnel/SpaceComms/actions/workflows/deploy-dev.yml/badge.svg)](https://github.com/TamTunnel/SpaceComms/actions/workflows/deploy-dev.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

---

## In Plain English

**One-liner**: SpaceComms is an open "universal language" that lets different satellite operators share collision alerts and maneuver plans, regardless of which systems they use.

**Simple analogy**: Imagine air-traffic control—pilots from any airline can communicate with any control tower because they all speak the same radio protocol. SpaceComms does the same thing for satellites: operators, tracking providers, and regulators can all exchange safety information using a common format, without being locked into one vendor's platform.

---

## The Problem

Space is getting crowded. Over 10,000 active satellites orbit Earth, with tens of thousands more planned. Every close approach between objects is a potential collision—and collisions create debris that threatens everything else in orbit.

**Today's challenge:**

- Satellite operators get collision warnings from different sources (government catalogs, commercial providers) in different formats
- There's no standard way for operators to share their planned maneuvers with each other
- When two satellites from different companies might collide, coordination happens through ad-hoc emails and phone calls
- Each new tracking provider creates yet another proprietary system to integrate

**What's needed**: A neutral, open protocol that any system can speak—not another closed platform, but the "common language" layer underneath all of them.

---

## Existing Approaches

SpaceComms isn't the first effort in space traffic management. Here's the landscape:

| Approach                                              | Description                                                                         | How SpaceComms Relates                                                                            |
| ----------------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| **Government STM** (e.g., U.S. Space Command, TraCSS) | National catalogs that track objects and issue collision warnings to operators      | SpaceComms can serve as a distribution protocol; these systems could publish via SpaceComms nodes |
| **Commercial STM providers**                          | Companies offering enhanced tracking, conjunction assessment, and maneuver planning | SpaceComms provides interoperability between providers; it's a protocol layer they can adopt      |
| **Bilateral data sharing**                            | Point-to-point agreements between operators                                         | SpaceComms standardizes these exchanges so they scale to many parties                             |

**SpaceComms is complementary**: It's not a competing operations center or tracking service. It's the protocol layer—like TCP/IP for the internet—that enables all these systems to exchange information in a standard way.

---

## What SpaceComms Does

A vendor-neutral protocol enabling interoperable exchange of:

- **Conjunction Data Messages (CDMs)** — collision risk warnings
- **Object state/ephemeris data** — satellite positions and trajectories
- **Maneuver intent and status** — planned orbital adjustments

Think of it as **BGP for space traffic management**: just as BGP lets internet networks share routing information without a central authority, SpaceComms lets space operators share collision warnings and maneuver plans in a decentralized mesh.

### Key Features

- 🛰️ **CCSDS-aligned** — CDM format compatible with CCSDS 508.0-B-1
- 🔗 **Peer-to-peer** — Decentralized mesh topology, no single point of failure
- 🔌 **Pluggable adapters** — Easy integration with existing infrastructure
- 📡 **Protocol-first** — Clear message specifications for interoperability
- 🔒 **Security-ready** — Hooks for mTLS and token-based authentication

---

## Quick Start

### Prerequisites

- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Build and Run

```bash
# Clone the repository
git clone https://github.com/TamTunnel/SpaceComms.git
cd SpaceComms

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

| Document                                                   | Audience            | Description                                        |
| ---------------------------------------------------------- | ------------------- | -------------------------------------------------- |
| **[Executive Overview](docs/overview-exec.md)**            | Executives, Policy  | Plain-language benefits, deployment scenarios, FAQ |
| **[Architecture](docs/architecture.md)**                   | Software Architects | Technical design, component diagrams, decisions    |
| **[Protocol Specification](docs/protocol-spec.md)**        | Developers          | Message formats, schemas, routing model            |
| **[API Reference](docs/api-reference.md)**                 | Developers          | REST endpoints and request/response schemas        |
| [Operations Runbook](docs/operations-and-runbook.md)       | Operations          | Deployment, monitoring, troubleshooting            |
| [Regulatory Compliance](docs/regulatory-and-compliance.md) | Legal/Policy        | Standards alignment and regulatory FAQ             |
| [Demo Guide](docs/demo-guide.md)                           | Anyone              | Step-by-step demo walkthrough                      |

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

- `HELLO` — Capability negotiation
- `OBJECT_STATE_ANNOUNCE/WITHDRAW` — Object tracking updates
- `CDM_ANNOUNCE/WITHDRAW` — Conjunction data
- `MANEUVER_INTENT/STATUS` — Orbital maneuver coordination
- `HEARTBEAT` — Connection health

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
