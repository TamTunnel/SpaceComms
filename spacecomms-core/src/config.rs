//! Configuration handling

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::Level;

/// SpaceComms configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Node identity configuration
    pub node: NodeConfig,
    
    /// Server configuration
    pub server: ServerConfig,
    
    /// API configuration
    #[serde(default)]
    pub api: ApiConfig,
    
    /// Peer configurations
    #[serde(default)]
    pub peers: Vec<PeerConfig>,
    
    /// Storage configuration
    #[serde(default)]
    pub storage: StorageConfig,
    
    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
    
    /// Protocol settings
    #[serde(default)]
    pub protocol: ProtocolConfig,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    fn validate(&self) -> Result<()> {
        if self.node.id.is_empty() {
            return Err(Error::Config("node.id is required".into()));
        }
        if self.server.port == 0 {
            return Err(Error::Config("server.port must be non-zero".into()));
        }
        Ok(())
    }

    /// Get the logging level
    pub fn logging_level(&self) -> Level {
        match self.logging.level.to_lowercase().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        }
    }
}

/// Node identity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Unique node identifier
    pub id: String,
    
    /// Human-readable node name
    #[serde(default)]
    pub name: String,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    #[serde(default = "default_host")]
    pub host: String,
    
    /// Port to listen on
    #[serde(default = "default_port")]
    pub port: u16,
    
    /// TLS configuration
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            tls: None,
        }
    }
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Path to TLS certificate
    pub cert_path: String,
    
    /// Path to TLS key
    pub key_path: String,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiConfig {
    /// Authentication configuration
    #[serde(default)]
    pub auth: AuthConfig,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    /// Whether authentication is enabled
    #[serde(default)]
    pub enabled: bool,
    
    /// Configured tokens
    #[serde(default)]
    pub tokens: Vec<TokenConfig>,
}

/// Token configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    /// Token identifier
    pub id: String,
    
    /// Token secret
    pub secret: String,
    
    /// Token permissions
    #[serde(default)]
    pub permissions: Vec<String>,
}

/// Peer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConfig {
    /// Peer identifier
    pub id: String,
    
    /// Peer address (URL)
    pub address: String,
    
    /// Authentication token for this peer
    #[serde(default)]
    pub auth_token: Option<String>,
    
    /// Routing policies for this peer
    #[serde(default)]
    pub policies: PeerPolicies,
}

/// Peer routing policies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeerPolicies {
    /// Accept CDM messages from this peer
    #[serde(default = "default_true")]
    pub accept_cdm: bool,
    
    /// Accept object state messages from this peer
    #[serde(default = "default_true")]
    pub accept_object_state: bool,
    
    /// Accept maneuver messages from this peer
    #[serde(default = "default_true")]
    pub accept_maneuver: bool,
    
    /// Forward CDMs received from this peer to other peers
    #[serde(default = "default_true")]
    pub forward_cdm: bool,
}

fn default_true() -> bool {
    true
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage type: "memory" or "file"
    #[serde(default = "default_storage_type")]
    pub storage_type: String,
    
    /// File path for file-based storage
    #[serde(default)]
    pub file_path: Option<String>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            storage_type: default_storage_type(),
            file_path: None,
        }
    }
}

fn default_storage_type() -> String {
    "memory".to_string()
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level: trace, debug, info, warn, error
    #[serde(default = "default_log_level")]
    pub level: String,
    
    /// Log format: json or pretty
    #[serde(default = "default_log_format")]
    pub format: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "pretty".to_string()
}

/// Protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    /// Heartbeat interval in seconds
    #[serde(default = "default_heartbeat_interval")]
    pub heartbeat_interval_seconds: u64,
    
    /// Session timeout in seconds
    #[serde(default = "default_session_timeout")]
    pub session_timeout_seconds: u64,
    
    /// Maximum hop count for message propagation
    #[serde(default = "default_max_hop_count")]
    pub max_hop_count: u32,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval_seconds: default_heartbeat_interval(),
            session_timeout_seconds: default_session_timeout(),
            max_hop_count: default_max_hop_count(),
        }
    }
}

fn default_heartbeat_interval() -> u64 {
    30
}

fn default_session_timeout() -> u64 {
    120
}

fn default_max_hop_count() -> u32 {
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_load_config() {
        let config_content = r#"
node:
  id: "test-node"
  name: "Test Node"

server:
  host: "127.0.0.1"
  port: 8080

peers: []
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(config_content.as_bytes()).unwrap();
        
        let config = Config::load(file.path()).unwrap();
        assert_eq!(config.node.id, "test-node");
        assert_eq!(config.server.port, 8080);
    }

    #[test]
    fn test_invalid_config_missing_node_id() {
        let config_content = r#"
node:
  id: ""
  name: "Test"

server:
  port: 8080
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(config_content.as_bytes()).unwrap();
        
        let result = Config::load(file.path());
        assert!(result.is_err());
    }
}
