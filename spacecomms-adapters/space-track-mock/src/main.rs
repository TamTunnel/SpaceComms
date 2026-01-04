//! Space-Track Mock Adapter
//!
//! Simulates Space-Track-like API responses using static fixtures
//! for testing and demonstration purposes.

use axum::{
    extract::Query,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// Catalog entry matching fixture structure
#[derive(Clone, Serialize, Deserialize)]
struct CatalogEntry {
    norad_id: String,
    object_name: String,
    object_type: String,
    owner: String,
    launch_date: String,
    decay_date: Option<String>,
    period_minutes: f64,
    inclination_deg: f64,
    apogee_km: f64,
    perigee_km: f64,
    #[serde(default)]
    rcs_size: Option<String>,
    #[serde(default)]
    country_code: Option<String>,
}

// Full CDM structure matching our internal format
#[derive(Clone, Serialize, Deserialize)]
struct CdmEntry {
    cdm_id: String,
    creation_date: String,
    originator: String,
    message_for: String,
    tca: String,
    miss_distance_m: f64,
    collision_probability: f64,
    object1: CdmObject,
    object2: CdmObject,
    #[serde(default)]
    relative_state: Option<RelativeState>,
    #[serde(default)]
    screening_data: Option<ScreeningData>,
}

#[derive(Clone, Serialize, Deserialize)]
struct CdmObject {
    object_id: String,
    object_name: String,
    object_type: String,
    operator_organization: Option<String>,
    ephemeris_name: String,
    covariance_method: String,
    maneuverable: bool,
    catalog_source: String,
    position_x_km: f64,
    position_y_km: f64,
    position_z_km: f64,
    velocity_x_km_s: f64,
    velocity_y_km_s: f64,
    velocity_z_km_s: f64,
}

#[derive(Clone, Serialize, Deserialize)]
struct RelativeState {
    relative_speed_m_s: f64,
    relative_position_r_km: f64,
    relative_position_t_km: f64,
    relative_position_n_km: f64,
}

#[derive(Clone, Serialize, Deserialize)]
struct ScreeningData {
    screening_volume_type: String,
    hard_body_radius_m: f64,
}

#[derive(Deserialize)]
struct CatalogQuery {
    #[serde(default)]
    norad_id: Option<String>,
    #[serde(default)]
    object_type: Option<String>,
    #[serde(default)]
    owner: Option<String>,
}

#[derive(Deserialize)]
struct CdmQuery {
    #[serde(default)]
    object_id: Option<String>,
    #[serde(default)]
    min_probability: Option<f64>,
}

// Shared state with loaded fixtures
struct AppData {
    catalog: Vec<CatalogEntry>,
    cdms: Vec<CdmEntry>,
}

fn load_fixtures() -> AppData {
    // Try to load from files, fall back to embedded defaults
    let catalog: Vec<CatalogEntry> = std::fs::read_to_string("fixtures/catalog.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(default_catalog);

    let cdms: Vec<CdmEntry> = std::fs::read_to_string("fixtures/cdms.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(default_cdms);

    info!("Loaded {} catalog entries, {} CDMs from fixtures", catalog.len(), cdms.len());

    AppData { catalog, cdms }
}

fn default_catalog() -> Vec<CatalogEntry> {
    vec![
        CatalogEntry {
            norad_id: "12345".to_string(),
            object_name: "STARLINK-1234".to_string(),
            object_type: "PAYLOAD".to_string(),
            owner: "SpaceX".to_string(),
            launch_date: "2023-05-15".to_string(),
            decay_date: None,
            period_minutes: 95.5,
            inclination_deg: 53.0,
            apogee_km: 560.0,
            perigee_km: 540.0,
            rcs_size: Some("MEDIUM".to_string()),
            country_code: Some("US".to_string()),
        },
    ]
}

fn default_cdms() -> Vec<CdmEntry> {
    vec![]
}

async fn get_catalog(
    data: axum::extract::State<Arc<AppData>>,
    Query(params): Query<CatalogQuery>,
) -> Json<Vec<CatalogEntry>> {
    let filtered: Vec<CatalogEntry> = data.catalog
        .iter()
        .filter(|e| {
            if let Some(ref norad) = params.norad_id {
                if e.norad_id != *norad {
                    return false;
                }
            }
            if let Some(ref obj_type) = params.object_type {
                if e.object_type != *obj_type {
                    return false;
                }
            }
            if let Some(ref owner) = params.owner {
                if e.owner != *owner {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    Json(filtered)
}

async fn get_cdms(
    data: axum::extract::State<Arc<AppData>>,
    Query(params): Query<CdmQuery>,
) -> Json<Vec<CdmEntry>> {
    let filtered: Vec<CdmEntry> = data.cdms
        .iter()
        .filter(|c| {
            if let Some(ref obj_id) = params.object_id {
                if c.object1.object_id != *obj_id && c.object2.object_id != *obj_id {
                    return false;
                }
            }
            if let Some(min_prob) = params.min_probability {
                if c.collision_probability < min_prob {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    Json(filtered)
}

async fn get_cdm_by_id(
    data: axum::extract::State<Arc<AppData>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<CdmEntry>, axum::http::StatusCode> {
    data.cdms
        .iter()
        .find(|c| c.cdm_id == id)
        .cloned()
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

async fn health() -> &'static str {
    "OK"
}

#[derive(Serialize)]
struct StatsResponse {
    catalog_count: usize,
    cdm_count: usize,
    status: String,
}

async fn stats(data: axum::extract::State<Arc<AppData>>) -> Json<StatsResponse> {
    Json(StatsResponse {
        catalog_count: data.catalog.len(),
        cdm_count: data.cdms.len(),
        status: "running".to_string(),
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    let data = Arc::new(load_fixtures());

    let app = Router::new()
        .route("/health", get(health))
        .route("/stats", get(stats))
        .route("/catalog", get(get_catalog))
        .route("/cdms", get(get_cdms))
        .route("/cdms/:id", get(get_cdm_by_id))
        .with_state(data);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Space-Track Mock running on http://{}", addr);
    info!("Endpoints:");
    info!("  GET /health           - Health check");
    info!("  GET /stats            - Statistics");
    info!("  GET /catalog          - List catalog entries");
    info!("  GET /catalog?norad_id=12345");
    info!("  GET /catalog?object_type=DEBRIS");
    info!("  GET /cdms             - List all CDMs");
    info!("  GET /cdms?object_id=12345");
    info!("  GET /cdms/:id         - Get specific CDM");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
