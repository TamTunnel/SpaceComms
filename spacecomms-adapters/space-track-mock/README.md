# Space-Track Mock Adapter

A mock implementation of a Space-Track-like API for testing and demonstrating SpaceComms integration without requiring access to the real Space-Track system.

## Overview

This service simulates the public-facing aspects of Space-Track's API, providing:

- A satellite catalog with basic orbital parameters
- Conjunction Data Messages (CDMs) in a format compatible with SpaceComms

## Running

```bash
# From the space-track-mock directory
cargo run

# Or with a custom port
PORT=9000 cargo run
```

The server starts on `http://localhost:9000` by default.

## Endpoints

### Health Check

```bash
curl http://localhost:9000/health
# Returns: OK
```

### Statistics

```bash
curl http://localhost:9000/stats
```

```json
{
  "catalog_count": 12,
  "cdm_count": 4,
  "status": "running"
}
```

### Catalog

List all catalog entries:

```bash
curl http://localhost:9000/catalog
```

Filter by NORAD ID:

```bash
curl "http://localhost:9000/catalog?norad_id=12345"
```

Filter by object type (PAYLOAD, DEBRIS, ROCKET_BODY):

```bash
curl "http://localhost:9000/catalog?object_type=DEBRIS"
```

Filter by owner:

```bash
curl "http://localhost:9000/catalog?owner=SpaceX"
```

### CDMs (Conjunction Data Messages)

List all CDMs:

```bash
curl http://localhost:9000/cdms
```

Filter by object ID (returns CDMs where the object is either object1 or object2):

```bash
curl "http://localhost:9000/cdms?object_id=12345"
```

Filter by minimum collision probability:

```bash
curl "http://localhost:9000/cdms?min_probability=1e-5"
```

Get a specific CDM by ID:

```bash
curl http://localhost:9000/cdms/CDM-2024-001-STARLINK-FY1C
```

## Fixtures

Static data is loaded from JSON files in the `fixtures/` directory:

- `fixtures/catalog.json` - Satellite catalog entries
- `fixtures/cdms.json` - Conjunction Data Messages

The CDM format follows the CCSDS 508.0-B-1 standard structure as implemented in SpaceComms.

## Integration with SpaceComms

To integrate with a SpaceComms node:

1. Start the mock:

   ```bash
   cd spacecomms-adapters/space-track-mock
   cargo run
   ```

2. Fetch CDMs and inject into SpaceComms:

   ```bash
   # Get a CDM from the mock
   curl http://localhost:9000/cdms/CDM-2024-001-STARLINK-FY1C > cdm.json

   # Inject into SpaceComms node
   curl -X POST http://localhost:8080/cdm \
     -H "Content-Type: application/json" \
     -d @cdm.json
   ```

## Environment Variables

| Variable   | Default | Description      |
| ---------- | ------- | ---------------- |
| `PORT`     | `9000`  | HTTP server port |
| `RUST_LOG` | `info`  | Log level        |
