//! Protocol message envelope

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Protocol version
pub const PROTOCOL_VERSION: &str = "1.0.0";

/// Message envelope wrapping all protocol messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    /// Protocol version
    pub protocol_version: String,
    
    /// Unique message identifier
    pub message_id: String,
    
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Source node identifier
    pub source_node_id: String,
    
    /// Message type
    pub message_type: MessageType,
    
    /// Number of hops from origin
    pub hop_count: u32,
    
    /// Time to live (max remaining hops)
    pub ttl: u32,
    
    /// Message payload
    pub payload: serde_json::Value,
}

impl Envelope {
    /// Create a new envelope for a message
    pub fn new(
        source_node_id: String,
        message_type: MessageType,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            protocol_version: PROTOCOL_VERSION.to_string(),
            message_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            source_node_id,
            message_type,
            hop_count: 0,
            ttl: 10,
        payload,
        }
    }

    /// Create a forwarded copy of this envelope
    pub fn forwarded(&self) -> Option<Self> {
        if self.ttl == 0 {
            return None;
        }
        
        Some(Self {
            protocol_version: self.protocol_version.clone(),
            message_id: self.message_id.clone(),
            timestamp: self.timestamp,
            source_node_id: self.source_node_id.clone(),
            message_type: self.message_type.clone(),
            hop_count: self.hop_count + 1,
            ttl: self.ttl - 1,
            payload: self.payload.clone(),
        })
    }

    /// Check if this message can be forwarded
    pub fn can_forward(&self) -> bool {
        self.ttl > 0
    }
}

/// Message type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    Hello,
    ObjectStateAnnounce,
    ObjectStateWithdraw,
    CdmAnnounce,
    CdmWithdraw,
    ManeuverIntent,
    ManeuverStatus,
    Heartbeat,
    Error,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Hello => write!(f, "HELLO"),
            MessageType::ObjectStateAnnounce => write!(f, "OBJECT_STATE_ANNOUNCE"),
            MessageType::ObjectStateWithdraw => write!(f, "OBJECT_STATE_WITHDRAW"),
            MessageType::CdmAnnounce => write!(f, "CDM_ANNOUNCE"),
            MessageType::CdmWithdraw => write!(f, "CDM_WITHDRAW"),
            MessageType::ManeuverIntent => write!(f, "MANEUVER_INTENT"),
            MessageType::ManeuverStatus => write!(f, "MANEUVER_STATUS"),
            MessageType::Heartbeat => write!(f, "HEARTBEAT"),
            MessageType::Error => write!(f, "ERROR"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_creation() {
        let env = Envelope::new(
            "node-1".to_string(),
            MessageType::Hello,
            serde_json::json!({"test": true}),
        );
        
        assert_eq!(env.protocol_version, PROTOCOL_VERSION);
        assert_eq!(env.source_node_id, "node-1");
        assert_eq!(env.hop_count, 0);
        assert_eq!(env.ttl, 10);
    }

    #[test]
    fn test_envelope_forwarding() {
        let env = Envelope::new(
            "node-1".to_string(),
            MessageType::CdmAnnounce,
            serde_json::json!({}),
        );
        
        let forwarded = env.forwarded().unwrap();
        assert_eq!(forwarded.hop_count, 1);
        assert_eq!(forwarded.ttl, 9);
        assert_eq!(forwarded.message_id, env.message_id);
    }

    #[test]
    fn test_ttl_exhausted() {
        let mut env = Envelope::new(
            "node-1".to_string(),
            MessageType::CdmAnnounce,
            serde_json::json!({}),
        );
        env.ttl = 0;
        
        assert!(!env.can_forward());
        assert!(env.forwarded().is_none());
    }
}
