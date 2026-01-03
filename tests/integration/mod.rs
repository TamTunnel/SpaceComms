//! SpaceComms Integration Tests

use serde_json::json;
use std::time::Duration;

const NODE_A_URL: &str = "http://localhost:8080";
const NODE_B_URL: &str = "http://localhost:8081";

/// Test: Health endpoint returns healthy status
#[tokio::test]
#[ignore] // Requires running node
async fn test_health_endpoint() {
    let client = reqwest::Client::new();
    
    let resp = client
        .get(format!("{}/health", NODE_A_URL))
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "healthy");
}

/// Test: CDM ingestion and retrieval
#[tokio::test]
#[ignore] // Requires running node
async fn test_cdm_lifecycle() {
    let client = reqwest::Client::new();
    
    // Ingest a CDM
    let cdm = json!({
        "cdm_id": "CDM-INT-TEST-001",
        "creation_date": "2024-01-15T14:00:00.000Z",
        "originator": "INTEGRATION-TEST",
        "message_for": "TEST-OPERATOR",
        "tca": "2024-01-17T08:30:00.000Z",
        "miss_distance_m": 200.0,
        "collision_probability": 1.0e-5,
        "object1": {
            "object_id": "TEST-SAT-001",
            "object_name": "Test Satellite 1",
            "object_type": "PAYLOAD",
            "maneuverable": true,
            "state_vector": {
                "reference_frame": "TEME",
                "x_km": 6878.0,
                "y_km": 0.0,
                "z_km": 0.0,
                "vx_km_s": 0.0,
                "vy_km_s": 7.6,
                "vz_km_s": 0.0
            }
        },
        "object2": {
            "object_id": "TEST-DEB-001",
            "object_name": "Test Debris 1",
            "object_type": "DEBRIS",
            "maneuverable": false,
            "state_vector": {
                "reference_frame": "TEME",
                "x_km": 6878.1,
                "y_km": 0.1,
                "z_km": 0.0,
                "vx_km_s": 0.0,
                "vy_km_s": 7.5,
                "vz_km_s": 0.0
            }
        }
    });
    
    let resp = client
        .post(format!("{}/cdm", NODE_A_URL))
        .json(&cdm)
        .send()
        .await
        .expect("Failed to ingest CDM");
    
    assert!(resp.status().is_success(), "CDM ingestion failed: {:?}", resp.text().await);
    
    // Retrieve the CDM
    let resp = client
        .get(format!("{}/cdms/CDM-INT-TEST-001", NODE_A_URL))
        .send()
        .await
        .expect("Failed to get CDM");
    
    assert!(resp.status().is_success());
    
    let retrieved: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(retrieved["cdm_id"], "CDM-INT-TEST-001");
    
    // Withdraw the CDM
    let resp = client
        .delete(format!("{}/cdms/CDM-INT-TEST-001", NODE_A_URL))
        .json(&json!({"reason": "TEST_COMPLETE"}))
        .send()
        .await
        .expect("Failed to withdraw CDM");
    
    assert!(resp.status().is_success());
    
    // Verify it's gone
    let resp = client
        .get(format!("{}/cdms/CDM-INT-TEST-001", NODE_A_URL))
        .send()
        .await
        .expect("Failed to check CDM");
    
    assert_eq!(resp.status(), 404);
}

/// Test: Peer management
#[tokio::test]
#[ignore] // Requires running nodes
async fn test_peer_management() {
    let client = reqwest::Client::new();
    
    // Add peer
    let resp = client
        .post(format!("{}/peers", NODE_A_URL))
        .json(&json!({
            "peer_id": "test-peer",
            "address": NODE_B_URL
        }))
        .send()
        .await
        .expect("Failed to add peer");
    
    assert!(resp.status().is_success());
    
    // List peers
    let resp = client
        .get(format!("{}/peers", NODE_A_URL))
        .send()
        .await
        .expect("Failed to list peers");
    
    let body: serde_json::Value = resp.json().await.unwrap();
    let peers = body["peers"].as_array().unwrap();
    assert!(peers.iter().any(|p| p["id"] == "test-peer"));
    
    // Remove peer
    let resp = client
        .delete(format!("{}/peers/test-peer", NODE_A_URL))
        .send()
        .await
        .expect("Failed to remove peer");
    
    assert!(resp.status().is_success());
}

/// Test: Maneuver announcement
#[tokio::test]
#[ignore] // Requires running node
async fn test_maneuver_announcement() {
    let client = reqwest::Client::new();
    
    let resp = client
        .post(format!("{}/maneuvers", NODE_A_URL))
        .json(&json!({
            "object_id": "TEST-SAT-001",
            "planned_start": "2024-01-20T06:00:00.000Z",
            "planned_duration_s": 30,
            "maneuver_type": "COLLISION_AVOIDANCE"
        }))
        .send()
        .await
        .expect("Failed to announce maneuver");
    
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "announced");
    assert!(body["maneuver_id"].as_str().unwrap().starts_with("MNVR-"));
}

/// Test: CDM validation rejects invalid input
#[tokio::test]
#[ignore] // Requires running node
async fn test_cdm_validation() {
    let client = reqwest::Client::new();
    
    // Missing required field
    let invalid_cdm = json!({
        "cdm_id": "CDM-INVALID",
        "originator": "TEST",
        // Missing: message_for, tca, miss_distance_m, etc.
    });
    
    let resp = client
        .post(format!("{}/cdm", NODE_A_URL))
        .json(&invalid_cdm)
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(resp.status(), 400);
}

/// Test: Multi-node CDM propagation scenario
/// This test requires both nodes to be running and peered
#[tokio::test]
#[ignore] // Requires running nodes
async fn test_multi_node_propagation() {
    let client = reqwest::Client::new();
    
    // Setup: Peer the nodes
    client
        .post(format!("{}/peers", NODE_A_URL))
        .json(&json!({
            "peer_id": "node-b",
            "address": NODE_B_URL
        }))
        .send()
        .await
        .expect("Failed to add peer on Node A");
    
    client
        .post(format!("{}/peers", NODE_B_URL))
        .json(&json!({
            "peer_id": "node-a",
            "address": NODE_A_URL
        }))
        .send()
        .await
        .expect("Failed to add peer on Node B");
    
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Inject CDM on Node A
    let cdm = json!({
        "cdm_id": "CDM-PROPAGATION-TEST",
        "creation_date": "2024-01-15T14:00:00.000Z",
        "originator": "NODE-A",
        "message_for": "ALL",
        "tca": "2024-01-17T08:30:00.000Z",
        "miss_distance_m": 100.0,
        "collision_probability": 5.0e-5,
        "object1": {
            "object_id": "PROP-SAT",
            "object_name": "Propagation Test Sat",
            "object_type": "PAYLOAD",
            "maneuverable": true,
            "state_vector": {
                "reference_frame": "TEME",
                "x_km": 6878.0, "y_km": 0.0, "z_km": 0.0,
                "vx_km_s": 0.0, "vy_km_s": 7.6, "vz_km_s": 0.0
            }
        },
        "object2": {
            "object_id": "PROP-DEB",
            "object_name": "Propagation Test Debris",
            "object_type": "DEBRIS",
            "maneuverable": false,
            "state_vector": {
                "reference_frame": "TEME",
                "x_km": 6878.1, "y_km": 0.0, "z_km": 0.0,
                "vx_km_s": 0.0, "vy_km_s": 7.5, "vz_km_s": 0.0
            }
        }
    });
    
    let resp = client
        .post(format!("{}/cdm", NODE_A_URL))
        .json(&cdm)
        .send()
        .await
        .expect("Failed to inject CDM");
    
    assert!(resp.status().is_success());
    
    // Note: In the reference implementation, propagation would need
    // to be triggered via the protocol layer. This test demonstrates
    // the structure for when that's implemented.
    
    println!("CDM propagation test completed. Check node logs for propagation details.");
}
