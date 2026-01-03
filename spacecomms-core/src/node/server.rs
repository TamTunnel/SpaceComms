//! HTTP server for SpaceComms node

use crate::cdm::{parse_cdm, CdmRecord};
use crate::config::Config;
use crate::node::{PeerInfo, PeerManager, PeerStatus, RoutingEngine};
use crate::storage::Storage;
use crate::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::info;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    config: Config,
    storage: Arc<dyn Storage>,
    peers: Arc<RwLock<PeerManager>>,
    routing: Arc<RoutingEngine>,
    start_time: chrono::DateTime<Utc>,
}

/// Node HTTP server
pub struct NodeServer {
    state: AppState,
}

impl NodeServer {
    /// Create a new node server
    pub fn new(
        config: Config,
        storage: Arc<dyn Storage>,
        peers: Arc<RwLock<PeerManager>>,
        routing: Arc<RoutingEngine>,
    ) -> Self {
        Self {
            state: AppState {
                config,
                storage,
                peers,
                routing,
                start_time: Utc::now(),
            },
        }
    }

    /// Run the server
    pub async fn run(self) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health))
            .route("/cdm", post(ingest_cdm))
            .route("/cdms", get(list_cdms))
            .route("/cdms/:id", get(get_cdm))
            .route("/cdms/:id", delete(withdraw_cdm))
            .route("/objects", get(list_objects))
            .route("/peers", get(list_peers))
            .route("/peers", post(add_peer))
            .route("/peers/:id", delete(remove_peer))
            .route("/maneuvers", post(announce_maneuver))
            .layer(TraceLayer::new_for_http())
            .with_state(self.state.clone());

        let addr = format!("{}:{}", self.state.config.server.host, self.state.config.server.port);
        info!("Listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

// ============================================================================
// Response types
// ============================================================================

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    node_id: String,
    uptime_seconds: i64,
    peers: PeerStats,
    objects_tracked: usize,
    cdms_active: usize,
    version: String,
}

#[derive(Serialize)]
struct PeerStats {
    connected: usize,
    total: usize,
}

#[derive(Serialize)]
struct CdmIngestResponse {
    cdm_id: String,
    status: String,
    propagated_to: Vec<String>,
}

#[derive(Serialize)]
struct CdmListResponse {
    cdms: Vec<CdmSummary>,
    total: usize,
}

#[derive(Serialize)]
struct CdmSummary {
    cdm_id: String,
    tca: chrono::DateTime<Utc>,
    miss_distance_m: f64,
    collision_probability: f64,
    object1_id: String,
    object2_id: String,
}

#[derive(Serialize)]
struct ObjectListResponse {
    objects: Vec<ObjectSummary>,
    total: usize,
}

#[derive(Serialize)]
struct ObjectSummary {
    object_id: String,
    object_name: String,
    object_type: String,
    last_updated: chrono::DateTime<Utc>,
}

#[derive(Serialize)]
struct PeerListResponse {
    peers: Vec<PeerInfo>,
}

#[derive(Deserialize)]
struct AddPeerRequest {
    peer_id: String,
    address: String,
    #[serde(default)]
    auth_token: Option<String>,
}

#[derive(Serialize)]
struct AddPeerResponse {
    peer_id: String,
    status: String,
}

#[derive(Serialize)]
struct RemovePeerResponse {
    peer_id: String,
    status: String,
}

#[derive(Deserialize)]
struct WithdrawCdmRequest {
    reason: String,
    #[serde(default)]
    superseded_by: Option<String>,
}

#[derive(Serialize)]
struct WithdrawResponse {
    cdm_id: String,
    status: String,
    reason: String,
}

#[derive(Deserialize)]
struct ManeuverRequest {
    object_id: String,
    #[serde(default)]
    related_cdm_id: Option<String>,
    planned_start: chrono::DateTime<Utc>,
    planned_duration_s: f64,
    maneuver_type: String,
}

#[derive(Serialize)]
struct ManeuverResponse {
    maneuver_id: String,
    status: String,
    propagated_to: Vec<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

// ============================================================================
// Handlers
// ============================================================================

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let peers = state.peers.read().await;
    let cdm_count = state.storage.cdm_count().await.unwrap_or(0);
    let object_count = state.storage.object_count().await.unwrap_or(0);
    let uptime = Utc::now() - state.start_time;

    Json(HealthResponse {
        status: "healthy".to_string(),
        node_id: state.config.node.id.clone(),
        uptime_seconds: uptime.num_seconds(),
        peers: PeerStats {
            connected: peers.connected_count(),
            total: peers.total_count(),
        },
        objects_tracked: object_count,
        cdms_active: cdm_count,
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn ingest_cdm(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> std::result::Result<(StatusCode, Json<CdmIngestResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Parse and validate CDM
    let cdm = parse_cdm(body).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "validation_failed".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    let cdm_id = cdm.cdm_id.clone();
    info!("CDM received: {}", cdm_id);
    info!("  TCA: {}", cdm.tca);
    info!("  Miss distance: {}m", cdm.miss_distance_m);
    info!("  Collision probability: {}", cdm.collision_probability);

    // Store CDM
    state.storage.store_cdm(cdm).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "storage_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    // Get peers for propagation (in real implementation, would forward)
    let peers = state.peers.read().await;
    let propagated_to: Vec<String> = peers
        .list_peers()
        .iter()
        .filter(|p| p.status == PeerStatus::Connected)
        .map(|p| p.id.clone())
        .collect();

    info!("CDM accepted, would forward to {} peers", propagated_to.len());

    Ok((
        StatusCode::CREATED,
        Json(CdmIngestResponse {
            cdm_id,
            status: "accepted".to_string(),
            propagated_to,
        }),
    ))
}

async fn list_cdms(State(state): State<AppState>) -> Json<CdmListResponse> {
    let cdms = state.storage.list_cdms().await.unwrap_or_default();
    let summaries: Vec<CdmSummary> = cdms
        .iter()
        .map(|c| CdmSummary {
            cdm_id: c.cdm_id.clone(),
            tca: c.tca,
            miss_distance_m: c.miss_distance_m,
            collision_probability: c.collision_probability,
            object1_id: c.object1.object_id.clone(),
            object2_id: c.object2.object_id.clone(),
        })
        .collect();

    Json(CdmListResponse {
        total: summaries.len(),
        cdms: summaries,
    })
}

async fn get_cdm(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> std::result::Result<Json<CdmRecord>, (StatusCode, Json<ErrorResponse>)> {
    match state.storage.get_cdm(&id).await {
        Ok(Some(cdm)) => Ok(Json(cdm)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("CDM not found: {}", id),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "storage_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

async fn withdraw_cdm(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<WithdrawCdmRequest>,
) -> std::result::Result<Json<WithdrawResponse>, (StatusCode, Json<ErrorResponse>)> {
    state.storage.withdraw_cdm(&id).await.map_err(|e| {
        if e.is_not_found() {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "not_found".to_string(),
                    message: format!("CDM not found: {}", id),
                }),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "storage_error".to_string(),
                    message: e.to_string(),
                }),
            )
        }
    })?;

    info!("CDM withdrawn: {} (reason: {})", id, body.reason);

    Ok(Json(WithdrawResponse {
        cdm_id: id,
        status: "withdrawn".to_string(),
        reason: body.reason,
    }))
}

async fn list_objects(State(state): State<AppState>) -> Json<ObjectListResponse> {
    let objects = state.storage.list_objects().await.unwrap_or_default();
    let summaries: Vec<ObjectSummary> = objects
        .iter()
        .map(|o| ObjectSummary {
            object_id: o.object_id.clone(),
            object_name: o.object_name.clone(),
            object_type: format!("{:?}", o.object_type),
            last_updated: o.last_updated,
        })
        .collect();

    Json(ObjectListResponse {
        total: summaries.len(),
        objects: summaries,
    })
}

async fn list_peers(State(state): State<AppState>) -> Json<PeerListResponse> {
    let peers = state.peers.read().await;
    Json(PeerListResponse {
        peers: peers.list_peers().to_vec(),
    })
}

async fn add_peer(
    State(state): State<AppState>,
    Json(body): Json<AddPeerRequest>,
) -> (StatusCode, Json<AddPeerResponse>) {
    let mut peers = state.peers.write().await;
    
    peers.add_peer(PeerInfo {
        id: body.peer_id.clone(),
        address: body.address,
        status: PeerStatus::Connecting,
        last_heartbeat: None,
        messages_sent: 0,
        messages_received: 0,
        policies: Default::default(),
    });

    info!("Peer added: {}", body.peer_id);

    (
        StatusCode::CREATED,
        Json(AddPeerResponse {
            peer_id: body.peer_id,
            status: "connecting".to_string(),
        }),
    )
}

async fn remove_peer(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> std::result::Result<Json<RemovePeerResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut peers = state.peers.write().await;
    
    if peers.remove_peer(&id) {
        info!("Peer removed: {}", id);
        Ok(Json(RemovePeerResponse {
            peer_id: id,
            status: "removed".to_string(),
        }))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("Peer not found: {}", id),
            }),
        ))
    }
}

async fn announce_maneuver(
    State(state): State<AppState>,
    Json(body): Json<ManeuverRequest>,
) -> (StatusCode, Json<ManeuverResponse>) {
    let maneuver_id = format!("MNVR-{}-{}", 
        Utc::now().format("%Y%m%d"),
        &uuid::Uuid::new_v4().to_string()[..8].to_uppercase()
    );

    info!("Maneuver intent announced: {}", maneuver_id);
    info!("  Object: {}", body.object_id);
    info!("  Planned start: {}", body.planned_start);
    info!("  Type: {}", body.maneuver_type);

    let peers = state.peers.read().await;
    let propagated_to: Vec<String> = peers
        .list_peers()
        .iter()
        .filter(|p| p.status == PeerStatus::Connected)
        .map(|p| p.id.clone())
        .collect();

    (
        StatusCode::CREATED,
        Json(ManeuverResponse {
            maneuver_id,
            status: "announced".to_string(),
            propagated_to,
        }),
    )
}
