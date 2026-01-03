//! CDM generator for testing and demos

use crate::cdm::{CdmObject, CdmRecord, RelativeState, ScreenType, ScreeningData};
use crate::protocol::{ObjectType, StateVector};
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

/// Generate a synthetic CDM for testing
pub fn generate_synthetic_cdm(
    object1_id: &str,
    object1_name: &str,
    object2_id: &str,
    object2_name: &str,
    tca: DateTime<Utc>,
    miss_distance_m: f64,
    collision_probability: f64,
) -> CdmRecord {
    let now = Utc::now();
    let cdm_id = format!("CDM-{}-{}", 
        now.format("%Y%m%d"),
        &Uuid::new_v4().to_string()[..8].to_uppercase()
    );
    
    CdmRecord {
        cdm_id,
        creation_date: now,
        originator: "SYNTHETIC-GENERATOR".to_string(),
        message_for: "DEMO-OPERATOR".to_string(),
        tca,
        miss_distance_m,
        collision_probability,
        object1: generate_object(object1_id, object1_name, ObjectType::Payload, true, now),
        object2: generate_object(object2_id, object2_name, ObjectType::Debris, false, now),
        relative_state: Some(RelativeState {
            relative_position_r_m: miss_distance_m * 0.3,
            relative_position_t_m: miss_distance_m * 0.6,
            relative_position_n_m: miss_distance_m * 0.1,
            relative_velocity_r_m_s: 0.5,
            relative_velocity_t_m_s: 15000.0,
            relative_velocity_n_m_s: 0.1,
        }),
        screening_data: Some(ScreeningData {
            screen_type: ScreenType::Routine,
            screen_volume_shape: Some("ELLIPSOID".to_string()),
            hard_body_radius_m: Some(15.0),
        }),
        data_quality_score: Some(0.95),
        conjunction_category: if collision_probability > 1e-3 {
            Some(crate::cdm::ConjunctionCategory::High)
        } else if collision_probability > 1e-5 {
            Some(crate::cdm::ConjunctionCategory::Medium)
        } else {
            Some(crate::cdm::ConjunctionCategory::Low)
        },
        recommended_action: if collision_probability > 1e-4 {
            Some(crate::cdm::RecommendedAction::Prepare)
        } else {
            Some(crate::cdm::RecommendedAction::Monitor)
        },
    }
}

fn generate_object(
    id: &str,
    name: &str,
    object_type: ObjectType,
    maneuverable: bool,
    epoch: DateTime<Utc>,
) -> CdmObject {
    // Generate reasonable LEO state vector
    let altitude_km = 550.0_f64;
    let radius_km = 6378.137 + altitude_km;
    let velocity_km_s = (398600.4418_f64 / radius_km).sqrt(); // Circular orbit velocity
    
    CdmObject {
        object_id: id.to_string(),
        object_name: name.to_string(),
        object_type,
        owner_operator: if maneuverable { Some("Demo Operator".to_string()) } else { None },
        maneuverable,
        state_vector: StateVector {
            reference_frame: "TEME".to_string(),
            epoch: Some(epoch),
            x_km: radius_km,
            y_km: 0.0,
            z_km: 0.0,
            vx_km_s: 0.0,
            vy_km_s: velocity_km_s,
            vz_km_s: 0.0,
        },
        covariance_rtm: Some(crate::protocol::CovarianceRtn {
            reference_frame: "RTN".to_string(),
            cr_r: 1.0e-4,
            ct_r: 0.0,
            ct_t: 1.0e-4,
            cn_r: 0.0,
            cn_t: 0.0,
            cn_n: 1.0e-4,
        }),
    }
}

/// Generate a demo CDM for the examples
pub fn generate_demo_cdm() -> CdmRecord {
    let tca = Utc::now() + Duration::days(2);
    generate_synthetic_cdm(
        "NORAD-12345",
        "STARLINK-1234",
        "NORAD-99999",
        "FENGYUN-1C-DEB",
        tca,
        150.5,
        1.2e-4,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cdm::validate_cdm;

    #[test]
    fn test_generate_demo_cdm() {
        let cdm = generate_demo_cdm();
        assert!(validate_cdm(&cdm).is_ok());
        assert!(cdm.cdm_id.starts_with("CDM-"));
    }

    #[test]
    fn test_generate_synthetic_cdm() {
        let tca = Utc::now() + Duration::days(1);
        let cdm = generate_synthetic_cdm(
            "SAT-001", "Test Satellite",
            "DEB-001", "Test Debris",
            tca, 100.0, 5e-5,
        );
        
        assert!(validate_cdm(&cdm).is_ok());
        assert_eq!(cdm.object1.object_id, "SAT-001");
        assert_eq!(cdm.object2.object_id, "DEB-001");
        assert!(cdm.object1.maneuverable);
        assert!(!cdm.object2.maneuverable);
    }
}
