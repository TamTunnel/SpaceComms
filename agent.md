# SpaceComms Agent Instructions

> AI agents working on this repository should follow these guidelines.

## Project Summary

SpaceComms is an open, CDM-centric, BGP-like protocol and reference service for space traffic coordination. It enables interoperable exchange of Conjunction Data Messages (CDMs), ephemeris data, and maneuver intent between satellite operators, STM providers, and regulators.

**Tech Stack**: Rust, HTTP/2 REST API, JSON protocol messages, in-memory + file storage  
**License**: Apache 2.0

## Key Directories

- `spacecomms-core/` - Core protocol service (Rust)
- `spacecomms-adapters/` - Integration adapters (Space-Track mock, Constellation Hub mock)
- `examples/` - Runnable demos and sample data
- `tests/` - Integration tests
- `docs/` - Human-readable documentation
- `.github/workflows/` - CI/CD definitions

## Commands

### Build

```bash
cd spacecomms-core && cargo build --release
```

### Run Core Service

```bash
cd spacecomms-core && cargo run -- start --config ../examples/config.yaml
```

### Run Tests

```bash
# Unit tests
cd spacecomms-core && cargo test

# Integration tests
cd tests && cargo test
```

### Linting and Formatting

```bash
cd spacecomms-core && cargo clippy --all-targets -- -D warnings
cd spacecomms-core && cargo fmt --check
```

### Run Demo

```bash
cd examples && ./demo.sh
```

## Coding Guidelines

### Error Handling

- Use `Result<T, E>` with custom error types via `thiserror`
- Provide context with `.context()` from `anyhow` for user-facing errors
- Never use `.unwrap()` in production code; use `.expect()` only for truly impossible cases

### Logging

- Use `tracing` crate for structured logging
- Log at appropriate levels: ERROR for failures, WARN for degraded state, INFO for operations, DEBUG for details

### Configuration

- Use YAML config files parsed by `serde_yaml`
- Environment variable overrides via `config` crate
- Document all config options in `docs/operations-and-runbook.md`

### Observability

- Expose `/health` and `/metrics` endpoints
- Include request IDs in all log entries
- Trace peer sessions and message flows

### Code Structure

- Small, composable modules with clear interfaces
- Protocol logic MUST be separate from adapter logic
- Storage layer abstracted behind traits

## Agent Boundaries

### MAY Do

- Edit source code, tests, documentation, CI config
- Add new dependencies via Cargo.toml
- Create new files in existing directory structure
- Refactor for clarity and performance

### MUST NOT Do

- Add secrets or real API credentials
- Depend on non-open or proprietary data sources
- Modify LICENSE to be more restrictive
- Break existing public API contracts without documentation

## Workflows

### Add New Protocol Message

1. Define message struct in `src/protocol/messages.rs`
2. Add to `MessageType` enum
3. Implement encode/decode in `src/protocol/codec.rs`
4. Add handler in `src/node/server.rs`
5. Add tests in `src/protocol/tests.rs`
6. Update `docs/protocol-spec.md`

### Add New Adapter

1. Create directory under `spacecomms-adapters/`
2. Initialize Rust crate with shared protocol dependency
3. Implement adapter trait from `spacecomms-core`
4. Add integration tests
5. Document in `docs/architecture.md`

### Update Docs and Tests for Change X

1. Identify affected documentation files
2. Update protocol spec if message format changed
3. Update API reference if endpoints changed
4. Add/update unit tests for new logic
5. Add/update integration tests for user-facing behavior
6. Run full test suite before committing
