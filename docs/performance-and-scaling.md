# SpaceComms Performance & Scaling

This document outlines the performance characteristics, scaling strategies, and benchmarking methodology for the SpaceComms reference implementation.

## Scale Assumptions

The reference implementation (Rust) is designed for the following scale:

- **Network Size**: ~10–100 directly peered nodes (mesh topology).
- **Throughput**: Processing tens of thousands of CDMs/day per node.
- **Environment**: Commodity cloud instances (e.g., 2 vCPU, 4GB RAM) or standard workstations.
- **Latency**: End-to-end propagation < 2 seconds across a 5-hop diameter network (under normal load).

## Scaling Levers

### 1. Horizontal Scaling (Mesh Growth)

SpaceComms is decentralized. Adding more nodes distributes the load of _querying_ data, but message propagation (flood/gossip) increases with node count.

- **Strategy**: Use careful peering policies to limit the "blast radius" of broadcasts.
- **Optimization**: Tune `ttl` and `hop_count` limits in `config.yaml`.

### 2. Message Batching

Wait for a small window to bundle multiple object announcements.

- _Current Status_: Messages are sent individually in v1.0. Future versions may implement batching.

### 3. Storage Partitioning

- The reference node uses a pluggable storage backend.
- For high volume, replace the in-memory/embedded DB with an external PostgreSQL instance or sharded data store.

## Benchmarking Guide

### How to Benchmark SpaceComms

You can run a local load test using the provided tools:

1.  **Setup**:
    Spin up 3 nodes (Node A, Node B, Node C) in a chain topology (A ↔ B ↔ C).

    ```bash
    # (Using docker-compose is recommended for isolation)
    docker-compose up -d --scale node=3
    ```

2.  **Generate Load**:
    Use a script to inject synthetic CDMs into Node A.

    ```bash
    # Pseudo-code example
    for i in {1..1000}; do
       curl -X POST http://localhost:8080/cdm -d @fixture.json
    done
    ```

3.  **Measure**:
    - **Throughput**: Requests per second handled by Node A.
    - **Propagation Delay**: Time from Node A inject → Node C processing (check logs).
    - **Resource Usage**: `docker stats` for CPU/Memory.

### Key Metrics to Watch

| Metric                   | Description                              | Health Indicator              |
| ------------------------ | ---------------------------------------- | ----------------------------- |
| `cdm_processing_time_ms` | Time to validate and store a CDM         | Should be < 50ms              |
| `message_queue_depth`    | Pending messages in the internal channel | Should drain quickly (near 0) |
| `active_peers`           | Number of connected nodes                | Stable count                  |
| `errors`                 | Failed validations or drops              | Should be < 1% of traffic     |

## Example Performance Data

_(Example only - specific to dev environment)_

- **Hardware**: MacBook Pro M3
- **Scenario**: 2 Nodes, 1000 CDMs back-to-back
- **Result**:
  - Avg Injection Rate: ~1200 msgs/sec
  - Avg Propagation Latency: ~15ms
  - Memory Usage: ~45MB RSS per node
