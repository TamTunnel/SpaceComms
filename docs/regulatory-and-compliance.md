# SpaceComms Regulatory & Compliance

_Standards alignment and regulatory considerations_

---

## Overview

SpaceComms is designed to align with international space communication standards while remaining vendor-neutral and open. This document outlines compliance considerations for operators, providers, and regulators.

> [!IMPORTANT]
> SpaceComms is a **reference implementation** using mock data. Production deployments must implement appropriate compliance measures for their jurisdiction.

---

## CCSDS CDM Alignment

### CCSDS 508.0-B-1 Compliance

The SpaceComms CDM message format is **schema-compatible** with CCSDS 508.0-B-1 "Conjunction Data Message Recommended Standard."

| CCSDS Requirement   | SpaceComms Implementation  | Status       |
| ------------------- | -------------------------- | ------------ |
| MESSAGE_ID field    | `cdm_id`                   | ✅ Compliant |
| CREATION_DATE field | `creation_date` (ISO 8601) | ✅ Compliant |
| ORIGINATOR field    | `originator`               | ✅ Compliant |
| TCA field           | `tca` (ISO 8601)           | ✅ Compliant |
| MISS_DISTANCE       | `miss_distance_m` (meters) | ✅ Compliant |
| State vector fields | Mapped directly            | ✅ Compliant |
| Covariance matrix   | RTN frame supported        | ✅ Compliant |

**Note**: SpaceComms uses JSON encoding rather than KVN or XML. The semantic content is equivalent and can be converted.

### CCSDS OCM Compatibility

SpaceComms object state messages can represent OCM-equivalent data:

- State vector with epoch
- Covariance information
- Object metadata

Full OCM format support is a planned extension.

---

## U.S. TraCSS Alignment

### Office of Space Commerce Requirements

SpaceComms aligns with publicly available TraCSS (Traffic Coordination System for Space) recommendations:

| TraCSS Concept                   | SpaceComms Implementation            |
| -------------------------------- | ------------------------------------ |
| CDM format standardization       | CCSDS-aligned messages               |
| Enhanced data quality indicators | `data_quality_score` extension field |
| Tiered conjunction alerting      | `conjunction_category` field         |
| Maneuver coordination            | MANEUVER_INTENT/STATUS messages      |
| Multi-provider interoperability  | Peer-to-peer protocol design         |

### Transparency Fields

SpaceComms supports recommended transparency extensions:

```json
{
  "cdm": {
    "data_quality_score": 0.95,
    "conjunction_category": "HIGH",
    "recommended_action": "PREPARE",
    "source_data_age_hours": 2.5
  }
}
```

---

## International Standards

### ISO TC20/SC14

SpaceComms is designed for potential alignment with ISO Technical Committee 20, Subcommittee 14 (Space Systems and Operations) standards:

- **ISO 26900**: Space systems — Conjunction assessment
- **ISO 16158**: Space systems — Mission disposal

**Current Status**: No formal ISO certification. Protocol design supports future standardization path.

### UN COPUOS

SpaceComms supports UN Committee on the Peaceful Uses of Outer Space (COPUOS) objectives:

| COPUOS Objective                        | SpaceComms Support                |
| --------------------------------------- | --------------------------------- |
| Long-term sustainability of outer space | Collision avoidance coordination  |
| International cooperation               | Vendor-neutral, open protocol     |
| Information sharing                     | Standardized message formats      |
| Transparency                            | Audit logging, traceable messages |

---

## Data Handling

### Open Data Only

SpaceComms reference implementation uses **only open or synthetic data**:

- ❌ No Space-Track credentials
- ❌ No classified data sources
- ❌ No proprietary catalog data
- ✅ Mock data fixtures
- ✅ Synthetic CDM generator

### Data Classification

SpaceComms nodes can handle different data classifications, but operators must implement appropriate controls:

| Data Type                     | Handling Requirement                      |
| ----------------------------- | ----------------------------------------- |
| Unclassified                  | Standard TLS encryption                   |
| Controlled Unclassified (CUI) | Enhanced access controls, audit logging   |
| Classified                    | Not supported in reference implementation |

### Data Retention

Operators should configure retention policies consistent with:

- Operational requirements
- Regulatory obligations
- Storage constraints

Default reference implementation: 7 days for CDMs, 30 days for object states.

---

## Operator Compliance Checklist

### Before Production Deployment

- [ ] Review jurisdiction-specific regulations
- [ ] Implement appropriate data classification controls
- [ ] Configure audit logging per requirements
- [ ] Establish data retention policies
- [ ] Document peer relationships
- [ ] Define incident response procedures

### Ongoing Compliance

- [ ] Regular security assessments
- [ ] Audit log review
- [ ] Peer policy updates
- [ ] Version currency (security patches)
- [ ] Staff training on procedures

---

## Regulatory FAQ

### Q: Is SpaceComms certified by any regulatory body?

**A**: No. SpaceComms is an open-source reference implementation. Operators are responsible for certifying their deployments meet applicable regulations.

### Q: Can SpaceComms be used with classified data?

**A**: The reference implementation is designed for unclassified data only. Organizations handling classified data should adapt the protocol for their security environments with appropriate certifications.

### Q: Does SpaceComms replace Space-Track?

**A**: No. SpaceComms is a protocol for sharing data between systems. It can consume data from Space-Track (via adapters) and distribute it, but does not replace Space-Track's data generation or authoritative catalog functions.

### Q: How does SpaceComms relate to SDA's TraCSS?

**A**: SpaceComms uses similar concepts and aligns with publicly available TraCSS recommendations. It is not an SDA product and is not endorsed by SDA. Operators could potentially connect SpaceComms nodes to TraCSS infrastructure via adapters.

### Q: What about ITAR/EAR compliance?

**A**: The SpaceComms protocol specification and reference implementation do not contain export-controlled technical data. Operators integrating SpaceComms with export-controlled systems must ensure their deployments comply with ITAR/EAR as applicable.

### Q: Can operators in different countries share data via SpaceComms?

**A**: The protocol supports international data exchange. Operators must ensure compliance with:

- Export control regulations
- Data sharing agreements
- National space regulations
- Organizational policies

### Q: How are disputes about CDM accuracy handled?

**A**: SpaceComms is a transport protocol and does not adjudicate data accuracy. Each CDM includes originator identification for traceability. Operators should establish bilateral or multilateral agreements for dispute resolution.

### Q: Is there liability for collision warnings?

**A**: This is a legal question outside SpaceComms scope. Operators should consult legal counsel regarding liability. SpaceComms provides audit trails for traceability but does not define liability frameworks.

---

## Future Standardization

### Proposed Path

1. **Community feedback** on reference implementation
2. **Pilot programs** with willing operators
3. **Industry consortium** for governance
4. **CCSDS submission** for protocol standardization
5. **ISO TC20/SC14** liaison for international adoption

### Governance Considerations

Long-term protocol governance may include:

- Technical steering committee
- Version management process
- Interoperability certification
- Dispute resolution framework

---

## Contact

For regulatory questions or compliance discussions, please open an issue in the repository or contact project maintainers.

---

## Disclaimer

This document provides general guidance and does not constitute legal advice. Operators are responsible for ensuring their SpaceComms deployments comply with all applicable laws, regulations, and organizational policies.
