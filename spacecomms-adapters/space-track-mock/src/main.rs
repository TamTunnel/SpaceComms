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
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Serialize)]
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
}

#[derive(Serialize)]
struct CdmEntry {
    cdm_id: String,
    tca: String,
    object1_norad: String,
    object1_name: String,
    object2_norad: String,
    object2_name: String,
    miss_distance_km: f64,
    pc: f64,
}

#[derive(Deserialize)]
struct CatalogQuery {
    #[serde(default)]
    norad_id: Option<String>,
    #[serde(default)]
    object_type: Option<String>,
}

fn mock_catalog() -> Vec<CatalogEntry> {
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
        },
        CatalogEntry {
            norad_id: "54321".to_string(),
            object_name: "ONEWEB-0123".to_string(),
            object_type: "PAYLOAD".to_string(),
            owner: "OneWeb".to_string(),
            launch_date: "2022-10-20".to_string(),
            decay_date: None,
            period_minutes: 127.8,
            inclination_deg: 87.4,
            apogee_km: 1200.0,
            perigee_km: 1195.0,
        },
        CatalogEntry {
            norad_id: "99999".to_string(),
            object_name: "FENGYUN-1C-DEB".to_string(),
            object_type: "DEBRIS".to_string(),
            owner: "PRC".to_string(),
            launch_date: "1999-05-10".to_string(),
            decay_date: None,
            period_minutes: 97.2,
            inclination_deg: 98.6,
            apogee_km: 850.0,
            perigee_km: 780.0,
        },
        CatalogEntry {
            norad_id: "88888".to_string(),
            object_name: "COSMOS-2251-DEB".to_string(),
            object_type: "DEBRIS".to_string(),
            owner: "CIS".to_string(),
            launch_date: "1993-06-16".to_string(),
            decay_date: None,
            period_minutes: 96.8,
            inclination_deg: 74.0,
            apogee_km: 800.0,
            perigee_km: 750.0,
        },
    ]
}

fn mock_cdms() -> Vec<CdmEntry> {
    vec![
        CdmEntry {
            cdm_id: "CDM-MOCK-001".to_string(),
            tca: "2024-01-20T08:30:00Z".to_string(),
            object1_norad: "12345".to_string(),
            object1_name: "STARLINK-1234".to_string(),
            object2_norad: "99999".to_string(),
            object2_name: "FENGYUN-1C-DEB".to_string(),
            miss_distance_km: 0.150,
            pc: 1.2e-4,
        },
        CdmEntry {
            cdm_id: "CDM-MOCK-002".to_string(),
            tca: "2024-01-22T14:15:00Z".to_string(),
            object1_norad: "54321".to_string(),
            object1_name: "ONEWEB-0123".to_string(),
            object2_norad: "88888".to_string(),
            object2_name: "COSMOS-2251-DEB".to_string(),
            miss_distance_km: 0.500,
            pc: 5.0e-6,
        },
    ]
}

async fn get_catalog(Query(params): Query<CatalogQuery>) -> Json<Vec<CatalogEntry>> {
    let catalog = mock_catalog();
    
    let filtered: Vec<CatalogEntry> = catalog
        .into_iter()
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
            true
        })
        .collect();
    
    Json(filtered)
}

async fn get_cdms() -> Json<Vec<CdmEntry>> {
    Json(mock_cdms())
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/catalog", get(get_catalog))
        .route("/cdms", get(get_cdms));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Space-Track Mock running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
