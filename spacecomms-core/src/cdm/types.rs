//! CDM types aligned with CCSDS 508.0-B-1

use crate::protocol::{CovarianceRtn, ObjectType, StateVector};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Conjunction Data Message record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdmRecord {
    /// Unique CDM identifier
    pub cdm_id: String,
    
    /// Creation timestamp
    pub creation_date: DateTime<Utc>,
    
    /// Originator (STM provider)
    pub originator: String,
    
    /// Message recipient operator
    pub message_for: String,
    
    /// Time of closest approach
    pub tca: DateTime<Utc>,
    
    /// Miss distance in meters
    pub miss_distance_m: f64,
    
    /// Collision probability (0.0 to 1.0)
    pub collision_probability: f64,
    
    /// Primary object
    pub object1: CdmObject,
    
    /// Secondary object
    pub object2: CdmObject,
    
    /// Relative state at TCA (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_state: Option<RelativeState>,
    
    /// Screening data (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screening_data: Option<ScreeningData>,
    
    // TraCSS extension fields
    /// Data quality score (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_score: Option<f64>,
    
    /// Risk tier classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conjunction_category: Option<ConjunctionCategory>,
    
    /// Suggested operator response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<RecommendedAction>,
}

/// Object within a CDM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdmObject {
    /// Object identifier (e.g., NORAD ID)
    pub object_id: String,
    
    /// Human-readable name
    pub object_name: String,
    
    /// Object type
    pub object_type: ObjectType,
    
    /// Owner/operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_operator: Option<String>,
    
    /// Whether object can maneuver
    #[serde(default)]
    pub maneuverable: bool,
    
    /// State vector at TCA
    pub state_vector: StateVector,
    
    /// Covariance in RTN frame
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covariance_rtm: Option<CovarianceRtn>,
}

/// Relative state at TCA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeState {
    /// Relative position in radial direction (meters)
    pub relative_position_r_m: f64,
    
    /// Relative position in transverse direction (meters)
    pub relative_position_t_m: f64,
    
    /// Relative position in normal direction (meters)
    pub relative_position_n_m: f64,
    
    /// Relative velocity in radial direction (m/s)
    pub relative_velocity_r_m_s: f64,
    
    /// Relative velocity in transverse direction (m/s)
    pub relative_velocity_t_m_s: f64,
    
    /// Relative velocity in normal direction (m/s)
    pub relative_velocity_n_m_s: f64,
}

/// Screening configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningData {
    /// Type of screening performed
    pub screen_type: ScreenType,
    
    /// Shape of screening volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_volume_shape: Option<String>,
    
    /// Combined hard body radius in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_body_radius_m: Option<f64>,
}

/// Screening type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScreenType {
    Routine,
    Special,
    Emergency,
}

/// Conjunction category (TraCSS extension)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConjunctionCategory {
    High,
    Medium,
    Low,
}

/// Recommended action (TraCSS extension)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecommendedAction {
    Monitor,
    Prepare,
    Maneuver,
}

/// Object record for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRecord {
    /// Object identifier
    pub object_id: String,
    
    /// Object name
    pub object_name: String,
    
    /// Object type
    pub object_type: ObjectType,
    
    /// Owner/operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_operator: Option<String>,
    
    /// State vector epoch
    pub epoch: DateTime<Utc>,
    
    /// Current state vector
    pub state_vector: StateVector,
    
    /// Covariance (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covariance: Option<CovarianceRtn>,
    
    /// Source node ID
    pub source_node: String,
    
    /// Last update time
    pub last_updated: DateTime<Utc>,
}
