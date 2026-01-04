# Governance and Evolution

This document outlines how the SpaceComms protocol and reference implementation are maintained, how decisions are made, and the process for evolving the standard.

## Governance Model

The SpaceComms project is currently maintained by the core project team with contributions from the open-source community.

### Maintainers

Maintainers are responsible for:

- Reviewing and merging pull requests.
- Managing releases and tags.
- Steering the technical roadmap.
- Ensuring security issues are addressed.

**Future Intent**: As the ecosystem grows, we intend to invite key stakeholders (e.g., STM operators, space agencies, commercial providers) to join as co-maintainers.

## Change Process

We strive for stability but recognize the need to evolve.

### 1. Proposal

To propose a change (especially to the protocol), open an issue labeled `protocol-change` or submit a design document in the `docs/proposals/` directory.

### 2. Discussion

Changes are discussed openly on GitHub. Major changes require broad consensus.

### 3. Implementation

Once approved, changes are implemented in a feature branch.

- Protocol changes MUST update `docs/protocol-spec.md`.
- Schema changes MUST be reflected in `schemas/`.
- Tests MUST be added to verify the new behavior.

### 4. Release

Changes are merged and included in the next scheduled release.

## Versioning Policy

### Protocol Versioning

SpaceComms uses **Semantic Versioning** (`MAJOR.MINOR`):

- **MAJOR**: Breaking changes that prevent interoperability with older nodes.
- **MINOR**: Backward-compatible features (e.g., new optional fields, new message types).

_See the [Protocol Specification](protocol-spec.md#protocol-version) for negotiation details._

### Implementation Versioning

The Rust reference node follows `MAJOR.MINOR.PATCH` to track software stability, distinct from the protocol version it implements.

## Standardization Pathway

SpaceComms is a community-driven initiative. However, we aim to align with international standards:

- **CCSDS**: We strictly follow CCSDS 508.0-B-1 for CDM payloads.
- **TraCSS**: We monitor NOAA's Traffic Coordination System for Space requirements.
- **ISO**: We aim for compatibility with ISO TC20/SC14 standards where applicable.

We welcome collaboration with standardization bodies to use SpaceComms as a reference or input for future interoperability standards.
