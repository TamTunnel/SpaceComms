//! Node module - server and session management

mod peer;
mod routing;
mod server;

pub use peer::*;
pub use routing::*;
pub use server::*;

use crate::config::Config;
use crate::storage::{create_storage, Storage};
use crate::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// SpaceComms node
pub struct Node {
    config: Config,
    storage: Arc<dyn Storage>,
    peers: Arc<RwLock<PeerManager>>,
    routing: Arc<RoutingEngine>,
}

impl Node {
    /// Create a new node from configuration
    pub async fn new(config: Config) -> Result<Self> {
        let storage = create_storage(&config.storage.storage_type);
        let peers = Arc::new(RwLock::new(PeerManager::new()));
        let routing = Arc::new(RoutingEngine::new(config.clone()));
        
        Ok(Self {
            config,
            storage,
            peers,
            routing,
        })
    }

    /// Run the node
    pub async fn run(self) -> Result<()> {
        info!("Node {} starting...", self.config.node.id);
        
        // Initialize configured peers
        {
            let mut peers = self.peers.write().await;
            for peer_config in &self.config.peers {
                peers.add_peer(PeerInfo {
                    id: peer_config.id.clone(),
                    address: peer_config.address.clone(),
                    status: PeerStatus::Disconnected,
                    last_heartbeat: None,
                    messages_sent: 0,
                    messages_received: 0,
                    policies: peer_config.policies.clone(),
                });
            }
        }
        
        // Start HTTP server
        let server = NodeServer::new(
            self.config.clone(),
            self.storage.clone(),
            self.peers.clone(),
            self.routing.clone(),
        );
        
        server.run().await
    }
}
