# Constellation Hub Mock Adapter

A mock implementation of a constellation operations platform that integrates with SpaceComms to provide satellite management and CDM alert tracking.

## Overview

This service simulates a constellation operator's backend system, providing:

- Satellite fleet registration and management
- Automatic monitoring of SpaceComms for relevant CDMs
- Alert generation when registered satellites are involved in conjunctions
- Maneuver recommendation generation

## Running

```bash
# From the constellation-hub-mock directory
cargo run

# With custom SpaceComms URL and port
SPACECOMMS_URL=http://localhost:8080 PORT=9001 cargo run
```

The server starts on `http://localhost:9001` by default.

## How It Works

1. **Satellite Registration**: Register satellites you want to monitor
2. **CDM Polling**: Background task polls SpaceComms every 10 seconds
3. **Alert Generation**: When a CDM involves a registered satellite, an alert is created
4. **Alert Management**: View, filter, and acknowledge alerts

## Endpoints

### Health & Stats

```bash
# Health check
curl http://localhost:9001/health

# Statistics
curl http://localhost:9001/stats
```

### Satellite Management

List all registered satellites:

```bash
curl http://localhost:9001/satellites
```

Register a new satellite:

```bash
curl -X POST http://localhost:9001/satellites \
  -H "Content-Type: application/json" \
  -d '{
    "norad_id": "12345",
    "name": "MY-SAT-001",
    "constellation": "MY_CONSTELLATION"
  }'
```

Get a specific satellite:

```bash
curl http://localhost:9001/satellites/sat-001
```

Unregister a satellite:

```bash
curl -X DELETE http://localhost:9001/satellites/sat-001
```

### Alert Management

List all alerts:

```bash
curl http://localhost:9001/alerts
```

Returns:

```json
{
  "alerts": [...],
  "total": 5,
  "unacknowledged": 2
}
```

Get alerts for a specific satellite:

```bash
curl http://localhost:9001/alerts/satellite/sat-001
```

Acknowledge an alert:

```bash
curl -X POST http://localhost:9001/alerts/{alert-id}/acknowledge
```

### Maneuver Recommendations

Request a maneuver recommendation:

```bash
curl -X POST http://localhost:9001/maneuver-recommendation \
  -H "Content-Type: application/json" \
  -d '{
    "satellite_id": "sat-001",
    "cdm_id": "CDM-2024-001"
  }'
```

## Pre-seeded Data

The mock starts with 3 pre-registered satellites:

- `sat-001`: STARLINK-1234 (NORAD: 12345)
- `sat-002`: STARLINK-1235 (NORAD: 12346)
- `sat-003`: ONEWEB-0123 (NORAD: 54321)

## Integration Demo

Run a full integration demo:

1. Start SpaceComms node:

   ```bash
   cd spacecomms-core
   cargo run -- start --config ../examples/config.yaml
   ```

2. Start Constellation Hub mock:

   ```bash
   cd spacecomms-adapters/constellation-hub-mock
   SPACECOMMS_URL=http://localhost:8080 cargo run
   ```

3. Inject a CDM into SpaceComms:

   ```bash
   curl -X POST http://localhost:8080/cdm \
     -H "Content-Type: application/json" \
     -d @../space-track-mock/fixtures/cdms.json | head -1
   ```

4. Check for alerts in Constellation Hub:
   ```bash
   curl http://localhost:9001/alerts
   ```

## Alert Severity Levels

Alerts are automatically classified by collision probability:

| Severity | Probability Range |
| -------- | ----------------- |
| CRITICAL | ≥ 1×10⁻⁴          |
| HIGH     | ≥ 1×10⁻⁵          |
| MEDIUM   | ≥ 1×10⁻⁶          |
| LOW      | < 1×10⁻⁶          |

## Environment Variables

| Variable         | Default                 | Description                 |
| ---------------- | ----------------------- | --------------------------- |
| `PORT`           | `9001`                  | HTTP server port            |
| `SPACECOMMS_URL` | `http://localhost:8080` | SpaceComms node URL to poll |
| `RUST_LOG`       | `info`                  | Log level                   |
