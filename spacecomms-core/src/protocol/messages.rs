//! Protocol message types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// HELLO Message
// ============================================================================

/// HELLO message for capability negotiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloPayload {
    /// Human-readable node name
    pub node_name: String,
    
    /// Protocol version this node is using
    pub protocol_version: String,
    
    /// Supported protocol versions (for negotiation)
    pub supported_versions: Vec<String>,
    
    /// Supported capabilities
    pub capabilities: Vec<String>,
    
    /// Optional authentication token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
}

impl Default for HelloPayload {
    fn default() -> Self {
        Self {
            node_name: "SpaceComms Node".to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities: vec!["CDM".to_string(), "OBJECT_STATE".to_string(), "MANEUVER".to_string()],
            supported_versions: vec!["1.0".to_string(), "1.1".to_string()],
            auth_token: None,
        }
    }
}

/// Current protocol version
pub const PROTOCOL_VERSION: &str = "1.0";

/// Result of version negotiation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionNegotiationResult {
    /// Versions are compatible, use the negotiated version
    Compatible(String),
    /// Versions are incompatible
    Incompatible { local: String, remote: String, reason: String },
}

/// Negotiate protocol version between two nodes
pub fn negotiate_version(local: &HelloPayload, remote: &HelloPayload) -> VersionNegotiationResult {
    // Parse major.minor from version strings
    let parse_version = |v: &str| -> Option<(u32, u32)> {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() >= 2 {
            Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
        } else if parts.len() == 1 {
            Some((parts[0].parse().ok()?, 0))
        } else {
            None
        }
    };

    let local_version = parse_version(&local.protocol_version);
    let remote_version = parse_version(&remote.protocol_version);

    match (local_version, remote_version) {
        (Some((local_major, local_minor)), Some((remote_major, remote_minor))) => {
            // Different major versions are incompatible
            if local_major != remote_major {
                return VersionNegotiationResult::Incompatible {
                    local: local.protocol_version.clone(),
                    remote: remote.protocol_version.clone(),
                    reason: format!(
                        "Major version mismatch: local v{}.x vs remote v{}.x",
                        local_major, remote_major
                    ),
                };
            }

            // Same major, use the lower minor version for compatibility
            let negotiated_minor = local_minor.min(remote_minor);
            let negotiated = format!("{}.{}", local_major, negotiated_minor);

            VersionNegotiationResult::Compatible(negotiated)
        }
        _ => VersionNegotiationResult::Incompatible {
            local: local.protocol_version.clone(),
            remote: remote.protocol_version.clone(),
            reason: "Could not parse version strings".to_string(),
        },
    }
}

#[cfg(test)]
mod version_tests {
    use super::*;

    #[test]
    fn test_same_version_compatible() {
        let local = HelloPayload {
            protocol_version: "1.0".to_string(),
            ..Default::default()
        };
        let remote = HelloPayload {
            protocol_version: "1.0".to_string(),
            ..Default::default()
        };

        match negotiate_version(&local, &remote) {
            VersionNegotiationResult::Compatible(v) => assert_eq!(v, "1.0"),
            _ => panic!("Expected compatible"),
        }
    }

    #[test]
    fn test_compatible_minor_difference() {
        let local = HelloPayload {
            protocol_version: "1.0".to_string(),
            ..Default::default()
        };
        let remote = HelloPayload {
            protocol_version: "1.1".to_string(),
            ..Default::default()
        };

        match negotiate_version(&local, &remote) {
            VersionNegotiationResult::Compatible(v) => assert_eq!(v, "1.0"),
            _ => panic!("Expected compatible"),
        }
    }

    #[test]
    fn test_incompatible_major_difference() {
        let local = HelloPayload {
            protocol_version: "1.0".to_string(),
            ..Default::default()
        };
        let remote = HelloPayload {
            protocol_version: "2.0".to_string(),
            ..Default::default()
        };

        match negotiate_version(&local, &remote) {
            VersionNegotiationResult::Incompatible { reason, .. } => {
                assert!(reason.contains("Major version mismatch"));
            }
            _ => panic!("Expected incompatible"),
        }
    }
}

// ============================================================================
// OBJECT_STATE Messages
// ============================================================================

/// State vector in a reference frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateVector {
    /// Reference frame (e.g., "TEME", "ITRF")
    pub reference_frame: String,
    
    /// Epoch of state vector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<DateTime<Utc>>,
    
    /// X position in km
    pub x_km: f64,
    
    /// Y position in km
    pub y_km: f64,
    
    /// Z position in km
    pub z_km: f64,
    
    /// X velocity in km/s
    pub vx_km_s: f64,
    
    /// Y velocity in km/s
    pub vy_km_s: f64,
    
    /// Z velocity in km/s
    pub vz_km_s: f64,
}

/// Covariance matrix in RTN frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovarianceRtn {
    /// Reference frame
    #[serde(default = "default_rtn")]
    pub reference_frame: String,
    
    /// Radial variance
    pub cr_r: f64,
    
    /// Transverse-radial covariance
    #[serde(default)]
    pub ct_r: f64,
    
    /// Transverse variance
    pub ct_t: f64,
    
    /// Normal-radial covariance
    #[serde(default)]
    pub cn_r: f64,
    
    /// Normal-transverse covariance
    #[serde(default)]
    pub cn_t: f64,
    
    /// Normal variance
    pub cn_n: f64,
}

fn default_rtn() -> String {
    "RTN".to_string()
}

/// Object type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjectType {
    Payload,
    Debris,
    RocketBody,
    Unknown,
}

/// Object state announcement payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectStateAnnouncePayload {
    /// Unique object identifier (e.g., NORAD ID)
    pub object_id: String,
    
    /// Human-readable object name
    pub object_name: String,
    
    /// Object type
    pub object_type: ObjectType,
    
    /// Owner/operator organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_operator: Option<String>,
    
    /// State vector epoch
    pub epoch: DateTime<Utc>,
    
    /// State vector
    pub state_vector: StateVector,
    
    /// Covariance matrix (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covariance: Option<CovarianceRtn>,
    
    /// Additional metadata
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// Reason for object state withdrawal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WithdrawReason {
    Decayed,
    ManeuverComplete,
    Superseded,
    Error,
}

/// Object state withdrawal payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectStateWithdrawPayload {
    /// Object being withdrawn
    pub object_id: String,
    
    /// Reason for withdrawal
    pub reason: WithdrawReason,
    
    /// When withdrawal takes effect
    pub effective_time: DateTime<Utc>,
}

// ============================================================================
// CDM Messages
// ============================================================================

/// CDM withdrawal reason
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CdmWithdrawReason {
    Superseded,
    TcaPassed,
    FalsePositive,
    Error,
}

/// CDM withdrawal payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdmWithdrawPayload {
    /// CDM being withdrawn
    pub cdm_id: String,
    
    /// Reason for withdrawal
    pub reason: CdmWithdrawReason,
    
    /// Replacement CDM ID (if superseded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded_by: Option<String>,
    
    /// When withdrawal takes effect
    pub effective_time: DateTime<Utc>,
}

// ============================================================================
// MANEUVER Messages
// ============================================================================

/// Maneuver type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ManeuverType {
    CollisionAvoidance,
    StationKeeping,
    Deorbit,
    Other,
}

/// Delta-V in VNB frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaV {
    /// Reference frame
    #[serde(default = "default_vnb")]
    pub reference_frame: String,
    
    /// Velocity component (km/s)
    pub dv_v_m_s: f64,
    
    /// Normal component (km/s)
    pub dv_n_m_s: f64,
    
    /// Binormal component (km/s)
    pub dv_b_m_s: f64,
}

fn default_vnb() -> String {
    "VNB".to_string()
}

/// Maneuver intent payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManeuverIntentPayload {
    /// Unique maneuver identifier
    pub maneuver_id: String,
    
    /// Object being maneuvered
    pub object_id: String,
    
    /// Related CDM ID (if collision avoidance)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_cdm_id: Option<String>,
    
    /// Planned burn start time
    pub planned_start: DateTime<Utc>,
    
    /// Planned burn duration in seconds
    pub planned_duration_s: f64,
    
    /// Maneuver type
    pub maneuver_type: ManeuverType,
    
    /// Planned delta-V
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_v: Option<DeltaV>,
    
    /// Predicted post-maneuver state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_post_maneuver_state: Option<StateVector>,
}

/// Maneuver status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ManeuverStatusType {
    Planned,
    InProgress,
    Completed,
    Cancelled,
    Failed,
}

/// Maneuver status payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManeuverStatusPayload {
    /// Maneuver being reported
    pub maneuver_id: String,
    
    /// Maneuvered object
    pub object_id: String,
    
    /// Current status
    pub status: ManeuverStatusType,
    
    /// Actual burn start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_start: Option<DateTime<Utc>>,
    
    /// Actual burn duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_duration_s: Option<f64>,
    
    /// Achieved delta-V
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achieved_delta_v: Option<DeltaV>,
    
    /// Observed post-maneuver state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_maneuver_state: Option<StateVector>,
}

// ============================================================================
// HEARTBEAT Message
// ============================================================================

/// Heartbeat payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatPayload {
    /// Monotonic sequence number
    pub sequence: u64,
    
    /// Number of objects tracked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects_tracked: Option<u64>,
    
    /// Number of active CDMs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdms_active: Option<u64>,
}

// ============================================================================
// ERROR Message
// ============================================================================

/// Error code enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    InvalidMessage,
    UnsupportedVersion,
    Unauthorized,
    RateLimited,
    InternalError,
}

/// Error payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    /// Error code
    pub error_code: ErrorCode,
    
    /// Human-readable error message
    pub error_message: String,
    
    /// Related message ID (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_message_id: Option<String>,
}
