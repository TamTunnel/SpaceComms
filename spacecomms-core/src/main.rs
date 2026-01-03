//! SpaceComms CLI Entry Point

use clap::{Parser, Subcommand};
use spacecomms::{Config, Result};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "spacecomms")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the SpaceComms node
    Start {
        /// Path to configuration file
        #[arg(short, long, default_value = "config.yaml")]
        config: PathBuf,
    },
    /// Validate configuration file
    ValidateConfig {
        /// Path to configuration file
        #[arg(short, long, default_value = "config.yaml")]
        config: PathBuf,
    },
    /// Add a peer to a running node
    Peer {
        #[command(subcommand)]
        command: PeerCommands,
    },
    /// CDM operations
    Cdm {
        #[command(subcommand)]
        command: CdmCommands,
    },
    /// List tracked objects
    Objects {
        /// Node API address
        #[arg(short, long, default_value = "http://localhost:8080")]
        address: String,
    },
}

#[derive(Subcommand)]
enum PeerCommands {
    /// Add a new peer
    Add {
        /// Node API address
        #[arg(short, long, default_value = "http://localhost:8080")]
        address: String,
        /// Peer ID
        #[arg(long)]
        peer_id: String,
        /// Peer address
        #[arg(long)]
        peer_address: String,
    },
    /// List configured peers
    List {
        /// Node API address
        #[arg(short, long, default_value = "http://localhost:8080")]
        address: String,
    },
}

#[derive(Subcommand)]
enum CdmCommands {
    /// Inject a CDM from file
    Inject {
        /// Node API address
        #[arg(short, long, default_value = "http://localhost:8080")]
        address: String,
        /// Path to CDM JSON file
        file: PathBuf,
    },
    /// List active CDMs
    List {
        /// Node API address
        #[arg(short, long, default_value = "http://localhost:8080")]
        address: String,
    },
}

fn setup_logging(level: Level) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level.as_str()));

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(true))
        .with(filter)
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { config } => {
            let cfg = Config::load(&config)?;
            setup_logging(cfg.logging_level());
            
            info!("Starting SpaceComms node: {}", cfg.node.id);
            
            let node = spacecomms::node::Node::new(cfg).await?;
            node.run().await?;
        }
        Commands::ValidateConfig { config } => {
            setup_logging(Level::INFO);
            
            match Config::load(&config) {
                Ok(cfg) => {
                    info!("Configuration valid");
                    info!("  Node ID: {}", cfg.node.id);
                    info!("  Server: {}:{}", cfg.server.host, cfg.server.port);
                    info!("  Peers configured: {}", cfg.peers.len());
                }
                Err(e) => {
                    eprintln!("Configuration invalid: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Peer { command } => {
            setup_logging(Level::INFO);
            
            match command {
                PeerCommands::Add { address, peer_id, peer_address } => {
                    let client = reqwest::Client::new();
                    let resp = client
                        .post(format!("{}/peers", address))
                        .json(&serde_json::json!({
                            "peer_id": peer_id,
                            "address": peer_address
                        }))
                        .send()
                        .await?;
                    
                    if resp.status().is_success() {
                        info!("Peer added successfully");
                        println!("{}", resp.text().await?);
                    } else {
                        eprintln!("Failed to add peer: {}", resp.text().await?);
                        std::process::exit(1);
                    }
                }
                PeerCommands::List { address } => {
                    let client = reqwest::Client::new();
                    let resp = client
                        .get(format!("{}/peers", address))
                        .send()
                        .await?;
                    
                    if resp.status().is_success() {
                        let json: serde_json::Value = resp.json().await?;
                        println!("{}", serde_json::to_string_pretty(&json)?);
                    } else {
                        eprintln!("Failed to list peers: {}", resp.text().await?);
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Cdm { command } => {
            setup_logging(Level::INFO);
            
            match command {
                CdmCommands::Inject { address, file } => {
                    let content = std::fs::read_to_string(&file)?;
                    let cdm: serde_json::Value = serde_json::from_str(&content)?;
                    
                    let client = reqwest::Client::new();
                    let resp = client
                        .post(format!("{}/cdm", address))
                        .json(&cdm)
                        .send()
                        .await?;
                    
                    if resp.status().is_success() {
                        info!("CDM injected successfully");
                        println!("{}", resp.text().await?);
                    } else {
                        eprintln!("Failed to inject CDM: {}", resp.text().await?);
                        std::process::exit(1);
                    }
                }
                CdmCommands::List { address } => {
                    let client = reqwest::Client::new();
                    let resp = client
                        .get(format!("{}/cdms", address))
                        .send()
                        .await?;
                    
                    if resp.status().is_success() {
                        let json: serde_json::Value = resp.json().await?;
                        println!("{}", serde_json::to_string_pretty(&json)?);
                    } else {
                        eprintln!("Failed to list CDMs: {}", resp.text().await?);
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Objects { address } => {
            setup_logging(Level::INFO);
            
            let client = reqwest::Client::new();
            let resp = client
                .get(format!("{}/objects", address))
                .send()
                .await?;
            
            if resp.status().is_success() {
                let json: serde_json::Value = resp.json().await?;
                println!("{}", serde_json::to_string_pretty(&json)?);
            } else {
                eprintln!("Failed to list objects: {}", resp.text().await?);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
