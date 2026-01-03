//! SpaceComms - An open, CDM-centric, BGP-like protocol for space traffic coordination
//!
//! This crate provides the core protocol implementation including:
//! - Protocol message types and encoding
//! - CDM parsing and validation
//! - Peer session management
//! - Routing engine
//! - REST API server

pub mod api;
pub mod cdm;
pub mod config;
pub mod error;
pub mod node;
pub mod protocol;
pub mod storage;

pub use config::Config;
pub use error::{Error, Result};
