# SpaceComms API Reference

_REST API and Protocol Message Reference_

---

## REST API Endpoints

Base URL: `http://localhost:8080` (configurable)

### Health & Status

#### GET /health

Health check endpoint.

**Response** `200 OK`

```json
{
  "status": "healthy",
  "node_id": "node-alpha-01",
  "uptime_seconds": 86400,
  "peers": {
    "connected": 3,
    "total": 5
  },
  "objects_tracked": 1250,
  "cdms_active": 42,
  "version": "1.0.0"
}
```

---

### CDM Management

#### POST /cdm

Ingest a new CDM from local source.

**Request**

```json
{
  "cdm_id": "CDM-2024-00001234",
  "creation_date": "2024-01-15T14:00:00.000Z",
  "originator": "LOCAL-SYSTEM",
  "message_for": "OPERATOR-ALPHA",
  "tca": "2024-01-17T08:30:00.000Z",
  "miss_distance_m": 150.5,
  "collision_probability": 1.2e-4,
  "object1": {
    "object_id": "NORAD-12345",
    "object_name": "STARLINK-1234",
    "object_type": "PAYLOAD",
    "maneuverable": true,
    "state_vector": {
      "reference_frame": "TEME",
      "epoch": "2024-01-15T12:00:00.000Z",
      "x_km": 6878.137,
      "y_km": 0.0,
      "z_km": 0.0,
      "vx_km_s": 0.0,
      "vy_km_s": 7.612,
      "vz_km_s": 0.0
    }
  },
  "object2": {
    "object_id": "NORAD-99999",
    "object_name": "DEBRIS-FRAGMENT",
    "object_type": "DEBRIS",
    "maneuverable": false,
    "state_vector": {
      "reference_frame": "TEME",
      "epoch": "2024-01-15T12:00:00.000Z",
      "x_km": 6878.2,
      "y_km": 0.05,
      "z_km": 0.0,
      "vx_km_s": 0.0,
      "vy_km_s": 7.61,
      "vz_km_s": 0.0
    }
  }
}
```

**Response** `201 Created`

```json
{
  "cdm_id": "CDM-2024-00001234",
  "status": "accepted",
  "propagated_to": ["peer-operator-b", "peer-stm-provider"]
}
```

**Error Response** `400 Bad Request`

```json
{
  "error": "validation_failed",
  "message": "Missing required field: tca",
  "field": "tca"
}
```

---

#### GET /cdms

List active CDMs.

**Query Parameters**
| Parameter | Type | Description |
|-----------|------|-------------|
| `object_id` | string | Filter by object ID |
| `min_probability` | number | Minimum collision probability |
| `limit` | integer | Max results (default: 100) |
| `offset` | integer | Pagination offset |

**Response** `200 OK`

```json
{
  "cdms": [
    {
      "cdm_id": "CDM-2024-00001234",
      "tca": "2024-01-17T08:30:00.000Z",
      "miss_distance_m": 150.5,
      "collision_probability": 1.2e-4,
      "object1_id": "NORAD-12345",
      "object2_id": "NORAD-99999",
      "created_at": "2024-01-15T14:00:00.000Z",
      "source_node": "node-stm-provider"
    }
  ],
  "total": 42,
  "limit": 100,
  "offset": 0
}
```

---

#### GET /cdms/{cdm_id}

Retrieve specific CDM by ID.

**Response** `200 OK`

Full CDM object (same schema as POST /cdm request).

**Error Response** `404 Not Found`

```json
{
  "error": "not_found",
  "message": "CDM not found: CDM-2024-00001234"
}
```

---

#### DELETE /cdms/{cdm_id}

Withdraw a CDM.

**Request**

```json
{
  "reason": "SUPERSEDED",
  "superseded_by": "CDM-2024-00001235"
}
```

**Response** `200 OK`

```json
{
  "cdm_id": "CDM-2024-00001234",
  "status": "withdrawn",
  "reason": "SUPERSEDED"
}
```

---

### Object Management

#### GET /objects

List tracked space objects.

**Query Parameters**
| Parameter | Type | Description |
|-----------|------|-------------|
| `type` | string | Filter by object type |
| `owner` | string | Filter by owner/operator |
| `limit` | integer | Max results (default: 100) |

**Response** `200 OK`

```json
{
  "objects": [
    {
      "object_id": "NORAD-12345",
      "object_name": "STARLINK-1234",
      "object_type": "PAYLOAD",
      "owner_operator": "SpaceX",
      "last_updated": "2024-01-15T12:00:00.000Z"
    }
  ],
  "total": 1250,
  "limit": 100,
  "offset": 0
}
```

---

#### GET /objects/{object_id}

Retrieve specific object.

**Response** `200 OK`

```json
{
  "object_id": "NORAD-12345",
  "object_name": "STARLINK-1234",
  "object_type": "PAYLOAD",
  "owner_operator": "SpaceX",
  "epoch": "2024-01-15T12:00:00.000Z",
  "state_vector": {
    "reference_frame": "TEME",
    "x_km": 6878.137,
    "y_km": 0.0,
    "z_km": 0.0,
    "vx_km_s": 0.0,
    "vy_km_s": 7.612,
    "vz_km_s": 0.0
  },
  "active_cdms": 2,
  "source_node": "node-operator-alpha"
}
```

---

### Peer Management

#### GET /peers

List configured peers.

**Response** `200 OK`

```json
{
  "peers": [
    {
      "peer_id": "peer-operator-b",
      "address": "https://operator-b.example.com:8443",
      "status": "connected",
      "last_heartbeat": "2024-01-15T14:29:30.000Z",
      "messages_sent": 1234,
      "messages_received": 5678
    },
    {
      "peer_id": "peer-stm-provider",
      "address": "https://stm.example.com:8443",
      "status": "disconnected",
      "last_heartbeat": "2024-01-15T14:00:00.000Z",
      "error": "connection_timeout"
    }
  ]
}
```

---

#### POST /peers

Add a new peer.

**Request**

```json
{
  "peer_id": "peer-new-operator",
  "address": "https://new-operator.example.com:8443",
  "auth_token": "bearer-token-here",
  "policies": {
    "accept_cdm": true,
    "accept_object_state": true,
    "accept_maneuver": false
  }
}
```

**Response** `201 Created`

```json
{
  "peer_id": "peer-new-operator",
  "status": "connecting"
}
```

---

#### DELETE /peers/{peer_id}

Remove a peer.

**Response** `200 OK`

```json
{
  "peer_id": "peer-new-operator",
  "status": "removed"
}
```

---

### Maneuver Management

#### POST /maneuvers

Announce maneuver intent.

**Request**

```json
{
  "object_id": "NORAD-12345",
  "related_cdm_id": "CDM-2024-00001234",
  "planned_start": "2024-01-16T06:00:00.000Z",
  "planned_duration_s": 30,
  "maneuver_type": "COLLISION_AVOIDANCE",
  "delta_v": {
    "reference_frame": "VNB",
    "dv_v_m_s": 0.0,
    "dv_n_m_s": 0.5,
    "dv_b_m_s": 0.0
  }
}
```

**Response** `201 Created`

```json
{
  "maneuver_id": "MNVR-2024-ALPHA-001",
  "status": "announced",
  "propagated_to": ["peer-operator-b", "peer-stm-provider"]
}
```

---

#### PATCH /maneuvers/{maneuver_id}

Update maneuver status.

**Request**

```json
{
  "status": "COMPLETED",
  "actual_start": "2024-01-16T06:00:05.000Z",
  "actual_duration_s": 28
}
```

**Response** `200 OK`

```json
{
  "maneuver_id": "MNVR-2024-ALPHA-001",
  "status": "COMPLETED"
}
```

---

## HTTP Status Codes

| Code                        | Meaning                  |
| --------------------------- | ------------------------ |
| `200 OK`                    | Request successful       |
| `201 Created`               | Resource created         |
| `400 Bad Request`           | Invalid request body     |
| `401 Unauthorized`          | Authentication required  |
| `403 Forbidden`             | Insufficient permissions |
| `404 Not Found`             | Resource not found       |
| `409 Conflict`              | Resource already exists  |
| `429 Too Many Requests`     | Rate limit exceeded      |
| `500 Internal Server Error` | Server error             |

---

## Authentication

API requests require authentication via Bearer token:

```
Authorization: Bearer <token>
```

Configure tokens in `spacecomms-config.yaml`:

```yaml
api:
  auth:
    enabled: true
    tokens:
      - id: "admin-token"
        secret: "your-secret-here"
        permissions: ["read", "write", "admin"]
```

---

## Protocol Message Schemas

See [Protocol Specification](protocol-spec.md) for complete message schemas including:

- HELLO
- OBJECT_STATE_ANNOUNCE / WITHDRAW
- CDM_ANNOUNCE / WITHDRAW
- MANEUVER_INTENT / STATUS
- HEARTBEAT
- ERROR

---

## Error Responses

All error responses follow this schema:

```json
{
  "error": "error_code",
  "message": "Human-readable description",
  "field": "optional_field_name",
  "details": {}
}
```

| Error Code          | Description                       |
| ------------------- | --------------------------------- |
| `validation_failed` | Request body validation failed    |
| `not_found`         | Requested resource not found      |
| `unauthorized`      | Authentication required or failed |
| `forbidden`         | Insufficient permissions          |
| `conflict`          | Resource already exists           |
| `rate_limited`      | Too many requests                 |
| `internal_error`    | Server error                      |
