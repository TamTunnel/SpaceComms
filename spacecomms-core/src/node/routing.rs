//! Routing engine

use crate::config::Config;
use crate::protocol::MessageType;

/// Routing decision
#[derive(Debug, Clone)]
pub enum RoutingDecision {
    /// Accept and store locally
    Accept,
    /// Reject the message
    Reject { reason: String },
    /// Accept and forward to peers
    AcceptAndForward { peer_ids: Vec<String> },
}

/// Routing engine
pub struct RoutingEngine {
    node_id: String,
    max_hop_count: u32,
}

impl RoutingEngine {
    /// Create a new routing engine
    pub fn new(config: Config) -> Self {
        Self {
            node_id: config.node.id,
            max_hop_count: config.protocol.max_hop_count,
        }
    }

    /// Decide how to route a message
    pub fn decide(
        &self,
        message_type: &MessageType,
        source_node_id: &str,
        hop_count: u32,
        ttl: u32,
        peer_ids: &[String],
    ) -> RoutingDecision {
        // Don't process our own messages
        if source_node_id == self.node_id {
            return RoutingDecision::Reject {
                reason: "Own message".to_string(),
            };
        }

        // Check hop count limit
        if hop_count > self.max_hop_count {
            return RoutingDecision::Reject {
                reason: "Max hop count exceeded".to_string(),
            };
        }

        // Check TTL
        if ttl == 0 {
            return RoutingDecision::Accept;
        }

        // Determine which peers to forward to
        match message_type {
            MessageType::Hello | MessageType::Heartbeat | MessageType::Error => {
                // Don't forward session messages
                RoutingDecision::Accept
            }
            MessageType::CdmAnnounce
            | MessageType::CdmWithdraw
            | MessageType::ObjectStateAnnounce
            | MessageType::ObjectStateWithdraw
            | MessageType::ManeuverIntent
            | MessageType::ManeuverStatus => {
                // Forward to all peers except source
                let forward_to: Vec<String> = peer_ids
                    .iter()
                    .filter(|&id| id != source_node_id)
                    .cloned()
                    .collect();

                if forward_to.is_empty() {
                    RoutingDecision::Accept
                } else {
                    RoutingDecision::AcceptAndForward { peer_ids: forward_to }
                }
            }
        }
    }

    /// Check if a peer should receive a message type
    pub fn should_forward_to_peer(
        &self,
        message_type: &MessageType,
        accept_cdm: bool,
        accept_object_state: bool,
        accept_maneuver: bool,
    ) -> bool {
        match message_type {
            MessageType::CdmAnnounce | MessageType::CdmWithdraw => accept_cdm,
            MessageType::ObjectStateAnnounce | MessageType::ObjectStateWithdraw => accept_object_state,
            MessageType::ManeuverIntent | MessageType::ManeuverStatus => accept_maneuver,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{NodeConfig, ProtocolConfig, ServerConfig, StorageConfig, LoggingConfig, ApiConfig};

    fn test_config() -> Config {
        Config {
            node: NodeConfig {
                id: "node-1".to_string(),
                name: "Test Node".to_string(),
            },
            server: ServerConfig::default(),
            api: ApiConfig::default(),
            peers: vec![],
            storage: StorageConfig::default(),
            logging: LoggingConfig::default(),
            protocol: ProtocolConfig::default(),
        }
    }

    #[test]
    fn test_reject_own_message() {
        let engine = RoutingEngine::new(test_config());
        let decision = engine.decide(
            &MessageType::CdmAnnounce,
            "node-1", // Same as our node
            0,
            10,
            &["peer-1".to_string()],
        );
        
        assert!(matches!(decision, RoutingDecision::Reject { .. }));
    }

    #[test]
    fn test_forward_cdm() {
        let engine = RoutingEngine::new(test_config());
        let decision = engine.decide(
            &MessageType::CdmAnnounce,
            "node-2",
            0,
            10,
            &["peer-1".to_string(), "peer-2".to_string()],
        );
        
        match decision {
            RoutingDecision::AcceptAndForward { peer_ids } => {
                assert_eq!(peer_ids.len(), 2);
            }
            _ => panic!("Expected AcceptAndForward"),
        }
    }

    #[test]
    fn test_no_forward_hello() {
        let engine = RoutingEngine::new(test_config());
        let decision = engine.decide(
            &MessageType::Hello,
            "node-2",
            0,
            10,
            &["peer-1".to_string()],
        );
        
        assert!(matches!(decision, RoutingDecision::Accept));
    }
}
