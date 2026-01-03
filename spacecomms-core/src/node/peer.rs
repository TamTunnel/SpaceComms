//! Peer management

use crate::config::PeerPolicies;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Peer connection status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PeerStatus {
    Connected,
    Connecting,
    Disconnected,
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer identifier
    pub id: String,
    
    /// Peer address (URL)
    pub address: String,
    
    /// Connection status
    pub status: PeerStatus,
    
    /// Last heartbeat received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_heartbeat: Option<DateTime<Utc>>,
    
    /// Messages sent to peer
    pub messages_sent: u64,
    
    /// Messages received from peer
    pub messages_received: u64,
    
    /// Routing policies
    #[serde(skip)]
    pub policies: PeerPolicies,
}

/// Peer manager
pub struct PeerManager {
    peers: Vec<PeerInfo>,
}

impl PeerManager {
    /// Create a new peer manager
    pub fn new() -> Self {
        Self { peers: Vec::new() }
    }

    /// Add a peer
    pub fn add_peer(&mut self, peer: PeerInfo) {
        // Check if peer already exists
        if let Some(existing) = self.peers.iter_mut().find(|p| p.id == peer.id) {
            existing.address = peer.address;
            existing.policies = peer.policies;
        } else {
            self.peers.push(peer);
        }
    }

    /// Remove a peer
    pub fn remove_peer(&mut self, id: &str) -> bool {
        let len_before = self.peers.len();
        self.peers.retain(|p| p.id != id);
        self.peers.len() < len_before
    }

    /// Get a peer by ID
    pub fn get_peer(&self, id: &str) -> Option<&PeerInfo> {
        self.peers.iter().find(|p| p.id == id)
    }

    /// Get a mutable peer by ID
    pub fn get_peer_mut(&mut self, id: &str) -> Option<&mut PeerInfo> {
        self.peers.iter_mut().find(|p| p.id == id)
    }

    /// List all peers
    pub fn list_peers(&self) -> &[PeerInfo] {
        &self.peers
    }

    /// Get connected peer count
    pub fn connected_count(&self) -> usize {
        self.peers.iter().filter(|p| p.status == PeerStatus::Connected).count()
    }

    /// Get total peer count
    pub fn total_count(&self) -> usize {
        self.peers.len()
    }

    /// Update peer status
    pub fn set_peer_status(&mut self, id: &str, status: PeerStatus) {
        if let Some(peer) = self.get_peer_mut(id) {
            peer.status = status;
        }
    }

    /// Record message sent
    pub fn record_sent(&mut self, id: &str) {
        if let Some(peer) = self.get_peer_mut(id) {
            peer.messages_sent += 1;
        }
    }

    /// Record message received
    pub fn record_received(&mut self, id: &str) {
        if let Some(peer) = self.get_peer_mut(id) {
            peer.messages_received += 1;
        }
    }

    /// Update heartbeat
    pub fn update_heartbeat(&mut self, id: &str) {
        if let Some(peer) = self.get_peer_mut(id) {
            peer.last_heartbeat = Some(Utc::now());
            peer.status = PeerStatus::Connected;
        }
    }
}

impl Default for PeerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_peer() -> PeerInfo {
        PeerInfo {
            id: "peer-1".to_string(),
            address: "http://localhost:8081".to_string(),
            status: PeerStatus::Disconnected,
            last_heartbeat: None,
            messages_sent: 0,
            messages_received: 0,
            policies: PeerPolicies::default(),
        }
    }

    #[test]
    fn test_add_peer() {
        let mut mgr = PeerManager::new();
        mgr.add_peer(test_peer());
        assert_eq!(mgr.total_count(), 1);
    }

    #[test]
    fn test_remove_peer() {
        let mut mgr = PeerManager::new();
        mgr.add_peer(test_peer());
        assert!(mgr.remove_peer("peer-1"));
        assert_eq!(mgr.total_count(), 0);
    }

    #[test]
    fn test_update_heartbeat() {
        let mut mgr = PeerManager::new();
        mgr.add_peer(test_peer());
        mgr.update_heartbeat("peer-1");
        
        let peer = mgr.get_peer("peer-1").unwrap();
        assert_eq!(peer.status, PeerStatus::Connected);
        assert!(peer.last_heartbeat.is_some());
    }
}
