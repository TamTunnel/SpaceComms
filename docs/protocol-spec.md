# SpaceComms Protocol Specification

_Version 1.0.0_

---

## Overview

The SpaceComms Protocol enables peer-to-peer exchange of space traffic coordination data. This specification defines message formats, routing behavior, and session management for interoperable implementations.

### Design Goals

1. **Interoperability**: Clear message schemas enable independent implementations
2. **Extensibility**: Versioned envelopes allow backward-compatible evolution
3. **Reliability**: Acknowledged delivery with retry semantics
4. **Security**: Authentication and encryption at transport and message levels

---

## Protocol Transport

### Transport Layer

SpaceComms uses HTTP/2 over TLS 1.3:

- **Endpoint**: `/spacecomms/v1/messages`
- **Method**: POST for all protocol messages
- **Content-Type**: `application/json`
- **Connection**: Long-lived with multiplexed streams

### Message Envelope

All messages use a versioned envelope:

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-uuid-here",
  "timestamp": "2024-01-15T14:30:00.000Z",
  "source_node_id": "node-alpha-01",
  "message_type": "CDM_ANNOUNCE",
  "hop_count": 1,
  "ttl": 10,
  "payload": { ... }
}
```

| Field              | Type    | Required | Description                          |
| ------------------ | ------- | -------- | ------------------------------------ |
| `protocol_version` | string  | Yes      | Semantic version of protocol         |
| `message_id`       | string  | Yes      | Unique message identifier (UUID)     |
| `timestamp`        | string  | Yes      | ISO 8601 timestamp with milliseconds |
| `source_node_id`   | string  | Yes      | Originating node identifier          |
| `message_type`     | string  | Yes      | One of defined message types         |
| `hop_count`        | integer | Yes      | Number of hops from origin           |
| `ttl`              | integer | Yes      | Maximum remaining hops               |
| `payload`          | object  | Yes      | Message-type-specific content        |

---

## Message Types

### HELLO

Capability negotiation when establishing peer session.

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-hello-001",
  "timestamp": "2024-01-15T14:30:00.000Z",
  "source_node_id": "node-alpha-01",
  "message_type": "HELLO",
  "hop_count": 0,
  "ttl": 1,
  "payload": {
    "node_name": "Alpha Operations",
    "capabilities": ["CDM", "OBJECT_STATE", "MANEUVER"],
    "supported_versions": ["1.0.0"],
    "auth_token": "bearer-token-here"
  }
}
```

**Payload Fields**:

| Field                | Type   | Required | Description                  |
| -------------------- | ------ | -------- | ---------------------------- |
| `node_name`          | string | Yes      | Human-readable node name     |
| `capabilities`       | array  | Yes      | Supported message categories |
| `supported_versions` | array  | Yes      | Protocol versions supported  |
| `auth_token`         | string | No       | Authentication credential    |

**Response**: Peer responds with their own HELLO.

---

### OBJECT_STATE_ANNOUNCE

Announce a tracked space object or update its state.

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-obj-001",
  "timestamp": "2024-01-15T14:30:00.000Z",
  "source_node_id": "node-alpha-01",
  "message_type": "OBJECT_STATE_ANNOUNCE",
  "hop_count": 1,
  "ttl": 10,
  "payload": {
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
    "covariance": {
      "reference_frame": "RTN",
      "cr_r": 1.0e-6,
      "ct_t": 1.0e-6,
      "cn_n": 1.0e-6
    },
    "metadata": {
      "source": "operator-ephemeris",
      "quality": "high"
    }
  }
}
```

**Payload Fields**:

| Field            | Type   | Required | Description                               |
| ---------------- | ------ | -------- | ----------------------------------------- |
| `object_id`      | string | Yes      | Unique object identifier (e.g., NORAD ID) |
| `object_name`    | string | Yes      | Human-readable name                       |
| `object_type`    | enum   | Yes      | PAYLOAD, DEBRIS, ROCKET_BODY, UNKNOWN     |
| `owner_operator` | string | No       | Operating organization                    |
| `epoch`          | string | Yes      | State vector epoch (ISO 8601)             |
| `state_vector`   | object | Yes      | Position and velocity                     |
| `covariance`     | object | No       | Uncertainty covariance matrix             |
| `metadata`       | object | No       | Additional context                        |

---

### OBJECT_STATE_WITHDRAW

Withdraw a previously announced object (no longer tracking).

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-obj-withdraw-001",
  "timestamp": "2024-01-15T14:35:00.000Z",
  "source_node_id": "node-alpha-01",
  "message_type": "OBJECT_STATE_WITHDRAW",
  "hop_count": 1,
  "ttl": 10,
  "payload": {
    "object_id": "NORAD-12345",
    "reason": "DECAYED",
    "effective_time": "2024-01-15T14:30:00.000Z"
  }
}
```

**Payload Fields**:

| Field            | Type   | Required | Description                                   |
| ---------------- | ------ | -------- | --------------------------------------------- |
| `object_id`      | string | Yes      | Object being withdrawn                        |
| `reason`         | enum   | Yes      | DECAYED, MANEUVER_COMPLETE, SUPERSEDED, ERROR |
| `effective_time` | string | Yes      | When withdrawal takes effect                  |

---

### CDM_ANNOUNCE

Announce a Conjunction Data Message.

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-cdm-001",
  "timestamp": "2024-01-15T14:30:00.000Z",
  "source_node_id": "node-stm-provider",
  "message_type": "CDM_ANNOUNCE",
  "hop_count": 1,
  "ttl": 10,
  "payload": {
    "cdm": {
      "cdm_id": "CDM-2024-00001234",
      "creation_date": "2024-01-15T14:00:00.000Z",
      "originator": "STM-PROVIDER-A",
      "message_for": "OPERATOR-ALPHA",
      "tca": "2024-01-17T08:30:00.000Z",
      "miss_distance_m": 150.5,
      "collision_probability": 1.2e-4,
      "object1": {
        "object_id": "NORAD-12345",
        "object_name": "STARLINK-1234",
        "object_type": "PAYLOAD",
        "owner_operator": "SpaceX",
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
        },
        "covariance_rtm": {
          "cr_r": 1.0e-4,
          "ct_r": 0.0,
          "ct_t": 1.0e-4,
          "cn_r": 0.0,
          "cn_t": 0.0,
          "cn_n": 1.0e-4
        }
      },
      "object2": {
        "object_id": "NORAD-99999",
        "object_name": "DEBRIS-FENGYUN-1C",
        "object_type": "DEBRIS",
        "owner_operator": "UNKNOWN",
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
        },
        "covariance_rtm": {
          "cr_r": 1.0e-2,
          "ct_r": 0.0,
          "ct_t": 1.0e-2,
          "cn_r": 0.0,
          "cn_t": 0.0,
          "cn_n": 1.0e-2
        }
      },
      "relative_state": {
        "relative_position_r_m": 50.0,
        "relative_position_t_m": 100.0,
        "relative_position_n_m": 25.0,
        "relative_velocity_r_m_s": 0.5,
        "relative_velocity_t_m_s": 15000.0,
        "relative_velocity_n_m_s": 0.1
      },
      "screening_data": {
        "screen_type": "ROUTINE",
        "screen_volume_shape": "ELLIPSOID",
        "hard_body_radius_m": 15.0
      }
    }
  }
}
```

**CDM Payload Fields** (aligned with CCSDS 508.0-B-1):

| Field                   | Type   | Required | CCSDS Alignment            |
| ----------------------- | ------ | -------- | -------------------------- |
| `cdm_id`                | string | Yes      | MESSAGE_ID                 |
| `creation_date`         | string | Yes      | CREATION_DATE              |
| `originator`            | string | Yes      | ORIGINATOR                 |
| `message_for`           | string | Yes      | MESSAGE_FOR                |
| `tca`                   | string | Yes      | TCA                        |
| `miss_distance_m`       | number | Yes      | MISS_DISTANCE              |
| `collision_probability` | number | Yes      | COLLISION_PROBABILITY      |
| `object1`               | object | Yes      | OBJECT1 block              |
| `object2`               | object | Yes      | OBJECT2 block              |
| `relative_state`        | object | No       | Relative position/velocity |
| `screening_data`        | object | No       | Screening configuration    |

**TraCSS Extended Fields** (optional):

| Field                  | Type   | Description                      |
| ---------------------- | ------ | -------------------------------- |
| `data_quality_score`   | number | Provider-assigned quality metric |
| `conjunction_category` | string | HIGH, MEDIUM, LOW risk tier      |
| `recommended_action`   | string | MONITOR, PREPARE, MANEUVER       |

---

### CDM_WITHDRAW

Withdraw a previously announced CDM (superseded or no longer valid).

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-cdm-withdraw-001",
  "timestamp": "2024-01-15T16:00:00.000Z",
  "source_node_id": "node-stm-provider",
  "message_type": "CDM_WITHDRAW",
  "hop_count": 1,
  "ttl": 10,
  "payload": {
    "cdm_id": "CDM-2024-00001234",
    "reason": "SUPERSEDED",
    "superseded_by": "CDM-2024-00001235",
    "effective_time": "2024-01-15T15:55:00.000Z"
  }
}
```

**Payload Fields**:

| Field            | Type   | Required | Description                                   |
| ---------------- | ------ | -------- | --------------------------------------------- |
| `cdm_id`         | string | Yes      | CDM being withdrawn                           |
| `reason`         | enum   | Yes      | SUPERSEDED, TCA_PASSED, FALSE_POSITIVE, ERROR |
| `superseded_by`  | string | No       | Replacement CDM ID                            |
| `effective_time` | string | Yes      | When withdrawal takes effect                  |

---

### MANEUVER_INTENT

Announce intention to perform an orbital maneuver.

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-mnvr-intent-001",
  "timestamp": "2024-01-15T15:00:00.000Z",
  "source_node_id": "node-operator-alpha",
  "message_type": "MANEUVER_INTENT",
  "hop_count": 1,
  "ttl": 10,
  "payload": {
    "maneuver_id": "MNVR-2024-ALPHA-001",
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
    },
    "predicted_post_maneuver_state": {
      "reference_frame": "TEME",
      "epoch": "2024-01-16T06:01:00.000Z",
      "x_km": 6880.0,
      "y_km": 50.0,
      "z_km": 0.0,
      "vx_km_s": 0.0,
      "vy_km_s": 7.615,
      "vz_km_s": 0.0
    }
  }
}
```

**Payload Fields**:

| Field                           | Type   | Required | Description                                          |
| ------------------------------- | ------ | -------- | ---------------------------------------------------- |
| `maneuver_id`                   | string | Yes      | Unique maneuver identifier                           |
| `object_id`                     | string | Yes      | Object being maneuvered                              |
| `related_cdm_id`                | string | No       | CDM triggering maneuver                              |
| `planned_start`                 | string | Yes      | Planned burn start time                              |
| `planned_duration_s`            | number | Yes      | Expected burn duration                               |
| `maneuver_type`                 | enum   | Yes      | COLLISION_AVOIDANCE, STATION_KEEPING, DEORBIT, OTHER |
| `delta_v`                       | object | No       | Planned velocity change                              |
| `predicted_post_maneuver_state` | object | No       | Expected state after maneuver                        |

---

### MANEUVER_STATUS

Report status of a previously announced maneuver.

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-mnvr-status-001",
  "timestamp": "2024-01-16T06:05:00.000Z",
  "source_node_id": "node-operator-alpha",
  "message_type": "MANEUVER_STATUS",
  "hop_count": 1,
  "ttl": 10,
  "payload": {
    "maneuver_id": "MNVR-2024-ALPHA-001",
    "object_id": "NORAD-12345",
    "status": "COMPLETED",
    "actual_start": "2024-01-16T06:00:05.000Z",
    "actual_duration_s": 28,
    "achieved_delta_v": {
      "reference_frame": "VNB",
      "dv_v_m_s": 0.0,
      "dv_n_m_s": 0.48,
      "dv_b_m_s": 0.01
    },
    "post_maneuver_state": {
      "reference_frame": "TEME",
      "epoch": "2024-01-16T06:00:35.000Z",
      "x_km": 6879.8,
      "y_km": 48.5,
      "z_km": 0.1,
      "vx_km_s": 0.0,
      "vy_km_s": 7.614,
      "vz_km_s": 0.0
    }
  }
}
```

**Payload Fields**:

| Field                 | Type   | Required | Description                                        |
| --------------------- | ------ | -------- | -------------------------------------------------- |
| `maneuver_id`         | string | Yes      | Maneuver being reported                            |
| `object_id`           | string | Yes      | Maneuvered object                                  |
| `status`              | enum   | Yes      | PLANNED, IN_PROGRESS, COMPLETED, CANCELLED, FAILED |
| `actual_start`        | string | No       | Actual burn start time                             |
| `actual_duration_s`   | number | No       | Actual burn duration                               |
| `achieved_delta_v`    | object | No       | Achieved velocity change                           |
| `post_maneuver_state` | object | No       | Observed post-maneuver state                       |

---

### HEARTBEAT

Connection health check.

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-hb-001",
  "timestamp": "2024-01-15T14:30:00.000Z",
  "source_node_id": "node-alpha-01",
  "message_type": "HEARTBEAT",
  "hop_count": 0,
  "ttl": 1,
  "payload": {
    "sequence": 12345,
    "objects_tracked": 1250,
    "cdms_active": 42
  }
}
```

**Payload Fields**:

| Field             | Type    | Required | Description               |
| ----------------- | ------- | -------- | ------------------------- |
| `sequence`        | integer | Yes      | Monotonic sequence number |
| `objects_tracked` | integer | No       | Current object count      |
| `cdms_active`     | integer | No       | Current active CDM count  |

---

### ERROR

Error response to invalid message.

```json
{
  "protocol_version": "1.0.0",
  "message_id": "msg-error-001",
  "timestamp": "2024-01-15T14:30:01.000Z",
  "source_node_id": "node-beta-02",
  "message_type": "ERROR",
  "hop_count": 0,
  "ttl": 1,
  "payload": {
    "error_code": "INVALID_MESSAGE",
    "error_message": "Missing required field: tca",
    "related_message_id": "msg-cdm-001"
  }
}
```

**Error Codes**:

| Code                  | Description                      |
| --------------------- | -------------------------------- |
| `INVALID_MESSAGE`     | Message schema validation failed |
| `UNSUPPORTED_VERSION` | Protocol version not supported   |
| `UNAUTHORIZED`        | Authentication failed            |
| `RATE_LIMITED`        | Too many messages                |
| `INTERNAL_ERROR`      | Node internal error              |

---

## Routing Model

### Peer Sessions

Nodes establish peer sessions via HELLO exchange:

1. Initiator sends HELLO
2. Responder validates and sends HELLO
3. Both nodes enable message exchange
4. HEARTBEAT maintains session health
5. Session terminates on timeout or explicit close

### Message Propagation

Messages propagate through the mesh:

```
1. Node A originates CDM_ANNOUNCE (hop_count=0, ttl=10)
2. Node A sends to peers B and C
3. Node B receives, increments hop_count to 1
4. Node B checks policies, decides to forward
5. Node B sends to peers D and E (but not A)
6. Process continues until ttl exhausted or no more peers
```

**Loop Prevention**:

- `message_id` deduplication
- `hop_count` tracking
- `ttl` enforcement
- Don't forward back to source

### Routing Policies

Nodes configure per-peer policies:

```yaml
routing_policies:
  - peer_id: "peer-operator-b"
    action: accept
    filters:
      - type: object_owner
        values: ["SpaceX", "OneWeb"]

  - peer_id: "peer-regulator"
    action: accept
    # Accept all from regulator

  - peer_id: "*"
    action: reject
    # Reject unknown peers by default
```

---

## CCSDS CDM Alignment

SpaceComms CDM format aligns with CCSDS 508.0-B-1 Recommended Standard.

### Core Fields Mapping

| SpaceComms Field        | CCSDS Field           | Notes                    |
| ----------------------- | --------------------- | ------------------------ |
| `cdm_id`                | MESSAGE_ID            | Direct mapping           |
| `creation_date`         | CREATION_DATE         | ISO 8601 format          |
| `originator`            | ORIGINATOR            | STM provider ID          |
| `message_for`           | MESSAGE_FOR           | Target operator          |
| `tca`                   | TCA                   | Time of closest approach |
| `miss_distance_m`       | MISS_DISTANCE         | In meters                |
| `collision_probability` | COLLISION_PROBABILITY | Decimal                  |

### Object Block Mapping

| SpaceComms Field      | CCSDS Field       | Notes              |
| --------------------- | ----------------- | ------------------ |
| `object_id`           | OBJECT_DESIGNATOR | NORAD ID preferred |
| `object_name`         | OBJECT_NAME       | Catalog name       |
| `object_type`         | OBJECT_TYPE       | Enumerated         |
| `maneuverable`        | MANEUVERABLE      | YES/NO             |
| `state_vector.epoch`  | EPOCH             | State epoch        |
| `state_vector.x_km`   | X                 | Position X         |
| `covariance_rtm.cr_r` | CR_R              | Radial variance    |

### TraCSS Extensions

SpaceComms supports TraCSS-recommended transparency fields:

| Field                  | Description                 | TraCSS Reference      |
| ---------------------- | --------------------------- | --------------------- |
| `data_quality_score`   | Provider quality metric     | Enhanced transparency |
| `conjunction_category` | Risk tier classification    | Tiered alerting       |
| `recommended_action`   | Suggested operator response | Decision support      |

---

## Security Considerations

### Authentication

- mTLS recommended for production
- Token-based auth via HELLO message
- Node identity tied to certificate

### Authorization

- Per-peer message filtering
- Object-level access control possible
- Audit trail for all messages

### Encryption

- TLS 1.3 minimum for transport
- Forward secrecy required
- Message-level signatures (future extension)

### Audit Logging

All messages should be logged with:

- Full message envelope
- Peer identity
- Processing decision (accept/reject/forward)
- Timestamp with microsecond precision

---

## Versioning

### Protocol Version

- Semantic versioning: MAJOR.MINOR
- MAJOR: Breaking changes (incompatible)
- MINOR: New message types or optional fields (backward compatible)

### Version Negotiation

During HELLO exchange, nodes negotiate a common protocol version:

**HELLO Payload includes:**

```json
{
  "protocol_version": "1.0",
  "supported_versions": ["1.0", "1.1"]
}
```

**Negotiation Rules:**

| Local | Remote | Result    | Negotiated         |
| ----- | ------ | --------- | ------------------ |
| 1.0   | 1.0    | ✅ OK     | 1.0                |
| 1.0   | 1.1    | ✅ OK     | 1.0 (lower minor)  |
| 1.1   | 1.0    | ✅ OK     | 1.0 (lower minor)  |
| 1.x   | 2.x    | ❌ Reject | - (major mismatch) |

**On Incompatible Version:**

1. Node sends ERROR message with code `UNSUPPORTED_VERSION`
2. Connection is closed
3. Event is logged with both version strings

```json
{
  "message_type": "ERROR",
  "payload": {
    "error_code": "UNSUPPORTED_VERSION",
    "error_message": "Major version mismatch: local v1.x vs remote v2.x",
    "related_message_id": "msg-hello-002"
  }
}
```

### Compatibility

- **Same major, different minor**: Compatible, use lower minor version
- **Different major**: Incompatible, reject connection
- Nodes advertise supported versions in HELLO for future negotiation
- Unknown fields must be preserved (forward compatibility)
- Unknown message types trigger ERROR response

---

## Interoperability Expectations

To interoperate at the "basic" level, an independent implementation MUST:

- Support the current **protocol version** and `HELLO` negotiation.
- Implement core message types (`HELLO`, `OBJECT_STATE_ANNOUNCE`, `CDM_ANNOUNCE`, `_WITHDRAW`, `HEARTBEAT`).
- Respect the **JSON schema** for CDMs (see [`schemas/cdm.schema.json`](../schemas/cdm.schema.json)).
- **Ignore** unknown optional fields (forward compatibility).
- Use the standard envelope format for all messages.

### Conformance Levels

| Level       | Role              | Capabilities                                                                     |
| ----------- | ----------------- | -------------------------------------------------------------------------------- |
| **Level 0** | Observer          | Read-only; subscribes to CDMs/Objects, maintains state, does not originate data. |
| **Level 1** | Producer/Consumer | Originate CDMs/Object States, receive and forward messages, basic routing.       |
| **Level 2** | Full Node         | Full routing logic, policy enforcement, mTLS security, history/query support.    |

---

## Related Documents

- [CCSDS 508.0-B-1](https://public.ccsds.org/Pubs/508x0b1e2.pdf) - CDM Recommended Standard
- [Architecture](architecture.md) - System design
- [API Reference](api-reference.md) - REST endpoints
- [Interop Guide](interop-guide.md) - Guide for third-party implementers
