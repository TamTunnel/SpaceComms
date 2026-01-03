//! Constellation Hub Mock Adapter
//!
//! Simulates a constellation operations platform API
//! for testing and demonstration purposes.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
struct Satellite {
    id: String,
    norad_id: String,
    name: String,
    constellation: String,
    status: String,
}

#[derive(Deserialize)]
struct RegisterSatelliteRequest {
    norad_id: String,
    name: String,
    constellation: String,
}

#[derive(Serialize)]
struct RegisterSatelliteResponse {
    id: String,
    status: String,
}

#[derive(Deserialize)]
struct ManeuverRecommendationRequest {
    satellite_id: String,
    cdm_id: String,
}

#[derive(Serialize)]
struct ManeuverRecommendationResponse {
    recommendation_id: String,
    satellite_id: String,
    cdm_id: String,
    action: String,
    maneuver_type: String,
    suggested_burn_time: String,
    delta_v_m_s: f64,
    confidence: f64,
}

#[derive(Clone)]
struct AppState {
    satellites: Arc<RwLock<HashMap<String, Satellite>>>,
}

async fn health() -> &'static str {
    "OK"
}

async fn list_satellites(State(state): State<AppState>) -> Json<Vec<Satellite>> {
    let satellites = state.satellites.read().unwrap();
    Json(satellites.values().cloned().collect())
}

async fn get_satellite(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Satellite>, axum::http::StatusCode> {
    let satellites = state.satellites.read().unwrap();
    satellites
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

async fn register_satellite(
    State(state): State<AppState>,
    Json(request): Json<RegisterSatelliteRequest>,
) -> Json<RegisterSatelliteResponse> {
    let id = Uuid::new_v4().to_string();
    
    let satellite = Satellite {
        id: id.clone(),
        norad_id: request.norad_id,
        name: request.name,
        constellation: request.constellation,
        status: "ACTIVE".to_string(),
    };
    
    {
        let mut satellites = state.satellites.write().unwrap();
        satellites.insert(id.clone(), satellite);
    }
    
    info!("Satellite registered: {}", id);
    
    Json(RegisterSatelliteResponse {
        id,
        status: "registered".to_string(),
    })
}

async fn get_maneuver_recommendation(
    Json(request): Json<ManeuverRecommendationRequest>,
) -> Json<ManeuverRecommendationResponse> {
    // Return a canned response for demo purposes
    let recommendation_id = format!("REC-{}", &Uuid::new_v4().to_string()[..8].to_uppercase());
    
    info!(
        "Maneuver recommendation requested for satellite {} / CDM {}",
        request.satellite_id, request.cdm_id
    );
    
    Json(ManeuverRecommendationResponse {
        recommendation_id,
        satellite_id: request.satellite_id,
        cdm_id: request.cdm_id,
        action: "MANEUVER_RECOMMENDED".to_string(),
        maneuver_type: "COLLISION_AVOIDANCE".to_string(),
        suggested_burn_time: "2024-01-19T06:00:00Z".to_string(),
        delta_v_m_s: 0.5,
        confidence: 0.92,
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    // Pre-seed some satellites
    let mut satellites = HashMap::new();
    satellites.insert(
        "sat-001".to_string(),
        Satellite {
            id: "sat-001".to_string(),
            norad_id: "12345".to_string(),
            name: "STARLINK-1234".to_string(),
            constellation: "STARLINK".to_string(),
            status: "ACTIVE".to_string(),
        },
    );
    satellites.insert(
        "sat-002".to_string(),
        Satellite {
            id: "sat-002".to_string(),
            norad_id: "12346".to_string(),
            name: "STARLINK-1235".to_string(),
            constellation: "STARLINK".to_string(),
            status: "ACTIVE".to_string(),
        },
    );

    let state = AppState {
        satellites: Arc::new(RwLock::new(satellites)),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/satellites", get(list_satellites))
        .route("/satellites", post(register_satellite))
        .route("/satellites/:id", get(get_satellite))
        .route("/maneuver-recommendation", post(get_maneuver_recommendation))
        .with_state(state);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9001);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Constellation Hub Mock running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
