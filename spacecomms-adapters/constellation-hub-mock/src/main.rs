//! Constellation Hub Mock Adapter
//!
//! Simulates a constellation operations platform that:
//! - Manages registered satellites
//! - Watches for CDMs from SpaceComms nodes
//! - Generates alerts when registered satellites are involved in conjunctions

use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tracing::{info, warn, error};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ============================================================================
// Types
// ============================================================================

#[derive(Clone, Serialize, Deserialize)]
struct Satellite {
    id: String,
    norad_id: String,
    name: String,
    constellation: String,
    status: String,
    registered_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Alert {
    id: String,
    satellite_id: String,
    satellite_name: String,
    cdm_id: String,
    tca: String,
    miss_distance_m: f64,
    collision_probability: f64,
    other_object_id: String,
    other_object_name: String,
    severity: String,
    created_at: DateTime<Utc>,
    acknowledged: bool,
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

#[derive(Serialize)]
struct AlertListResponse {
    alerts: Vec<Alert>,
    total: usize,
    unacknowledged: usize,
}

#[derive(Clone)]
struct AppState {
    satellites: Arc<RwLock<HashMap<String, Satellite>>>,
    alerts: Arc<RwLock<Vec<Alert>>>,
    spacecomms_url: String,
}

// ============================================================================
// CDM Poller Types (for fetching from SpaceComms node)
// ============================================================================

#[derive(Deserialize)]
struct CdmListResponse {
    cdms: Vec<CdmSummary>,
    total: usize,
}

#[derive(Clone, Deserialize)]
struct CdmSummary {
    cdm_id: String,
    tca: DateTime<Utc>,
    miss_distance_m: f64,
    collision_probability: f64,
    object1_id: String,
    object2_id: String,
}

// ============================================================================
// Handlers
// ============================================================================

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
        norad_id: request.norad_id.clone(),
        name: request.name.clone(),
        constellation: request.constellation,
        status: "ACTIVE".to_string(),
        registered_at: Utc::now(),
    };

    {
        let mut satellites = state.satellites.write().unwrap();
        satellites.insert(id.clone(), satellite);
    }

    info!(
        satellite_id = %id,
        norad_id = %request.norad_id,
        name = %request.name,
        "Satellite registered"
    );

    Json(RegisterSatelliteResponse {
        id,
        status: "registered".to_string(),
    })
}

async fn unregister_satellite(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let mut satellites = state.satellites.write().unwrap();
    if satellites.remove(&id).is_some() {
        info!(satellite_id = %id, "Satellite unregistered");
        Ok(Json(serde_json::json!({
            "id": id,
            "status": "unregistered"
        })))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn list_alerts(State(state): State<AppState>) -> Json<AlertListResponse> {
    let alerts = state.alerts.read().unwrap();
    let unacknowledged = alerts.iter().filter(|a| !a.acknowledged).count();
    
    Json(AlertListResponse {
        total: alerts.len(),
        unacknowledged,
        alerts: alerts.clone(),
    })
}

async fn get_alerts_for_satellite(
    State(state): State<AppState>,
    Path(satellite_id): Path<String>,
) -> Json<Vec<Alert>> {
    let alerts = state.alerts.read().unwrap();
    let filtered: Vec<Alert> = alerts
        .iter()
        .filter(|a| a.satellite_id == satellite_id)
        .cloned()
        .collect();
    
    Json(filtered)
}

async fn acknowledge_alert(
    State(state): State<AppState>,
    Path(alert_id): Path<String>,
) -> Result<Json<Alert>, axum::http::StatusCode> {
    let mut alerts = state.alerts.write().unwrap();
    
    if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
        alert.acknowledged = true;
        info!(alert_id = %alert_id, "Alert acknowledged");
        Ok(Json(alert.clone()))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn get_maneuver_recommendation(
    Json(request): Json<ManeuverRecommendationRequest>,
) -> Json<ManeuverRecommendationResponse> {
    let recommendation_id = format!("REC-{}", &Uuid::new_v4().to_string()[..8].to_uppercase());

    info!(
        recommendation_id = %recommendation_id,
        satellite_id = %request.satellite_id,
        cdm_id = %request.cdm_id,
        "Maneuver recommendation generated"
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

#[derive(Serialize)]
struct StatsResponse {
    satellites_registered: usize,
    total_alerts: usize,
    unacknowledged_alerts: usize,
    spacecomms_url: String,
}

async fn stats(State(state): State<AppState>) -> Json<StatsResponse> {
    let satellites = state.satellites.read().unwrap();
    let alerts = state.alerts.read().unwrap();
    let unacknowledged = alerts.iter().filter(|a| !a.acknowledged).count();

    Json(StatsResponse {
        satellites_registered: satellites.len(),
        total_alerts: alerts.len(),
        unacknowledged_alerts: unacknowledged,
        spacecomms_url: state.spacecomms_url.clone(),
    })
}

// ============================================================================
// CDM Poller (Background Task)
// ============================================================================

fn calculate_severity(probability: f64) -> String {
    if probability >= 1e-4 {
        "CRITICAL".to_string()
    } else if probability >= 1e-5 {
        "HIGH".to_string()
    } else if probability >= 1e-6 {
        "MEDIUM".to_string()
    } else {
        "LOW".to_string()
    }
}

async fn poll_cdms(state: AppState) {
    let client = reqwest::Client::new();
    let mut known_cdms: std::collections::HashSet<String> = std::collections::HashSet::new();
    
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;

        // Fetch CDMs from SpaceComms node
        let url = format!("{}/cdms", state.spacecomms_url);
        
        match client.get(&url).send().await {
            Ok(response) => {
                if let Ok(cdm_list) = response.json::<CdmListResponse>().await {
                    let satellites = state.satellites.read().unwrap();
                    let satellite_norad_ids: Vec<String> = satellites
                        .values()
                        .map(|s| s.norad_id.clone())
                        .collect();
                    drop(satellites);

                    for cdm in cdm_list.cdms {
                        // Skip if we've already processed this CDM
                        if known_cdms.contains(&cdm.cdm_id) {
                            continue;
                        }
                        known_cdms.insert(cdm.cdm_id.clone());

                        // Check if either object is one of our registered satellites
                        let satellites = state.satellites.read().unwrap();
                        
                        let matching_sat = satellites.values().find(|s| {
                            s.norad_id == cdm.object1_id || s.norad_id == cdm.object2_id
                        });

                        if let Some(satellite) = matching_sat {
                            let other_object_id = if satellite.norad_id == cdm.object1_id {
                                cdm.object2_id.clone()
                            } else {
                                cdm.object1_id.clone()
                            };

                            let alert = Alert {
                                id: Uuid::new_v4().to_string(),
                                satellite_id: satellite.id.clone(),
                                satellite_name: satellite.name.clone(),
                                cdm_id: cdm.cdm_id.clone(),
                                tca: cdm.tca.to_rfc3339(),
                                miss_distance_m: cdm.miss_distance_m,
                                collision_probability: cdm.collision_probability,
                                other_object_id,
                                other_object_name: "Unknown".to_string(),
                                severity: calculate_severity(cdm.collision_probability),
                                created_at: Utc::now(),
                                acknowledged: false,
                            };

                            drop(satellites);

                            info!(
                                alert_id = %alert.id,
                                satellite = %alert.satellite_name,
                                cdm_id = %alert.cdm_id,
                                severity = %alert.severity,
                                "New CDM alert created for registered satellite"
                            );

                            let mut alerts = state.alerts.write().unwrap();
                            alerts.push(alert);
                        }
                    }
                }
            }
            Err(e) => {
                // Only log occasionally to avoid spam
                warn!("Could not connect to SpaceComms at {}: {}", state.spacecomms_url, e);
            }
        }
    }
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    // Pre-seed some satellites for demo
    let mut satellites = HashMap::new();
    satellites.insert(
        "sat-001".to_string(),
        Satellite {
            id: "sat-001".to_string(),
            norad_id: "12345".to_string(),
            name: "STARLINK-1234".to_string(),
            constellation: "STARLINK".to_string(),
            status: "ACTIVE".to_string(),
            registered_at: Utc::now(),
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
            registered_at: Utc::now(),
        },
    );
    satellites.insert(
        "sat-003".to_string(),
        Satellite {
            id: "sat-003".to_string(),
            norad_id: "54321".to_string(),
            name: "ONEWEB-0123".to_string(),
            constellation: "ONEWEB".to_string(),
            status: "ACTIVE".to_string(),
            registered_at: Utc::now(),
        },
    );

    let spacecomms_url = std::env::var("SPACECOMMS_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    let state = AppState {
        satellites: Arc::new(RwLock::new(satellites)),
        alerts: Arc::new(RwLock::new(Vec::new())),
        spacecomms_url: spacecomms_url.clone(),
    };

    // Start background CDM poller
    let poller_state = state.clone();
    tokio::spawn(async move {
        poll_cdms(poller_state).await;
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/stats", get(stats))
        .route("/satellites", get(list_satellites))
        .route("/satellites", post(register_satellite))
        .route("/satellites/:id", get(get_satellite))
        .route("/satellites/:id", delete(unregister_satellite))
        .route("/alerts", get(list_alerts))
        .route("/alerts/satellite/:id", get(get_alerts_for_satellite))
        .route("/alerts/:id/acknowledge", post(acknowledge_alert))
        .route("/maneuver-recommendation", post(get_maneuver_recommendation))
        .with_state(state);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9001);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Constellation Hub Mock running on http://{}", addr);
    info!("Watching SpaceComms at {}", spacecomms_url);
    info!("Endpoints:");
    info!("  GET  /health                    - Health check");
    info!("  GET  /stats                     - Statistics");
    info!("  GET  /satellites                - List satellites");
    info!("  POST /satellites                - Register satellite");
    info!("  GET  /satellites/:id            - Get satellite");
    info!("  DELETE /satellites/:id          - Unregister satellite");
    info!("  GET  /alerts                    - List all alerts");
    info!("  GET  /alerts/satellite/:id      - Get alerts for satellite");
    info!("  POST /alerts/:id/acknowledge    - Acknowledge alert");
    info!("  POST /maneuver-recommendation   - Get maneuver recommendation");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
