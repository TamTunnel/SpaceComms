//! CDM parser and validator

use crate::cdm::CdmRecord;
use crate::{Error, Result};

/// Validate a CDM record
pub fn validate_cdm(cdm: &CdmRecord) -> Result<()> {
    // Required field validations
    if cdm.cdm_id.is_empty() {
        return Err(Error::CdmValidation("cdm_id is required".into()));
    }
    
    if cdm.originator.is_empty() {
        return Err(Error::CdmValidation("originator is required".into()));
    }
    
    if cdm.message_for.is_empty() {
        return Err(Error::CdmValidation("message_for is required".into()));
    }
    
    // Validate miss distance is non-negative
    if cdm.miss_distance_m < 0.0 {
        return Err(Error::CdmValidation("miss_distance_m must be non-negative".into()));
    }
    
    // Validate collision probability is in range [0, 1]
    if cdm.collision_probability < 0.0 || cdm.collision_probability > 1.0 {
        return Err(Error::CdmValidation(
            "collision_probability must be between 0.0 and 1.0".into()
        ));
    }
    
    // Validate objects
    validate_cdm_object(&cdm.object1, "object1")?;
    validate_cdm_object(&cdm.object2, "object2")?;
    
    // Validate TCA is after creation date
    if cdm.tca < cdm.creation_date {
        return Err(Error::CdmValidation(
            "tca must be after creation_date".into()
        ));
    }
    
    Ok(())
}

fn validate_cdm_object(obj: &crate::cdm::CdmObject, field_name: &str) -> Result<()> {
    if obj.object_id.is_empty() {
        return Err(Error::CdmValidation(format!("{}.object_id is required", field_name)));
    }
    
    if obj.object_name.is_empty() {
        return Err(Error::CdmValidation(format!("{}.object_name is required", field_name)));
    }
    
    Ok(())
}

/// Parse CDM from JSON value
pub fn parse_cdm(value: serde_json::Value) -> Result<CdmRecord> {
    let cdm: CdmRecord = serde_json::from_value(value)?;
    validate_cdm(&cdm)?;
    Ok(cdm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cdm::{CdmObject, ScreenType, ScreeningData};
    use crate::protocol::{ObjectType, StateVector};
    use chrono::Utc;

    fn create_test_cdm() -> CdmRecord {
        let now = Utc::now();
        let tca = now + chrono::Duration::days(2);
        
        CdmRecord {
            cdm_id: "CDM-TEST-001".to_string(),
            creation_date: now,
            originator: "TEST-PROVIDER".to_string(),
            message_for: "TEST-OPERATOR".to_string(),
            tca,
            miss_distance_m: 150.0,
            collision_probability: 1.2e-4,
            object1: CdmObject {
                object_id: "NORAD-12345".to_string(),
                object_name: "SAT-1".to_string(),
                object_type: ObjectType::Payload,
                owner_operator: Some("Operator A".to_string()),
                maneuverable: true,
                state_vector: StateVector {
                    reference_frame: "TEME".to_string(),
                    epoch: Some(now),
                    x_km: 6878.137,
                    y_km: 0.0,
                    z_km: 0.0,
                    vx_km_s: 0.0,
                    vy_km_s: 7.612,
                    vz_km_s: 0.0,
                },
                covariance_rtm: None,
            },
            object2: CdmObject {
                object_id: "NORAD-99999".to_string(),
                object_name: "DEBRIS-1".to_string(),
                object_type: ObjectType::Debris,
                owner_operator: None,
                maneuverable: false,
                state_vector: StateVector {
                    reference_frame: "TEME".to_string(),
                    epoch: Some(now),
                    x_km: 6878.200,
                    y_km: 0.050,
                    z_km: 0.0,
                    vx_km_s: 0.0,
                    vy_km_s: 7.610,
                    vz_km_s: 0.0,
                },
                covariance_rtm: None,
            },
            relative_state: None,
            screening_data: Some(ScreeningData {
                screen_type: ScreenType::Routine,
                screen_volume_shape: Some("ELLIPSOID".to_string()),
                hard_body_radius_m: Some(15.0),
            }),
            data_quality_score: None,
            conjunction_category: None,
            recommended_action: None,
        }
    }

    #[test]
    fn test_valid_cdm() {
        let cdm = create_test_cdm();
        assert!(validate_cdm(&cdm).is_ok());
    }

    #[test]
    fn test_missing_cdm_id() {
        let mut cdm = create_test_cdm();
        cdm.cdm_id = String::new();
        assert!(validate_cdm(&cdm).is_err());
    }

    #[test]
    fn test_invalid_collision_probability() {
        let mut cdm = create_test_cdm();
        cdm.collision_probability = 1.5; // > 1.0
        assert!(validate_cdm(&cdm).is_err());
    }

    #[test]
    fn test_tca_before_creation() {
        let mut cdm = create_test_cdm();
        cdm.tca = cdm.creation_date - chrono::Duration::hours(1);
        assert!(validate_cdm(&cdm).is_err());
    }
}
