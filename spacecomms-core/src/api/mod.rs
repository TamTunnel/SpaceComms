//! API module

// The main API functionality is in node/server.rs
// This module provides additional utilities

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

/// Standard API error response
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.error.as_str() {
            "validation_failed" => StatusCode::BAD_REQUEST,
            "not_found" => StatusCode::NOT_FOUND,
            "unauthorized" => StatusCode::UNAUTHORIZED,
            "forbidden" => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}

impl ApiError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self {
            error: "validation_failed".to_string(),
            message: message.into(),
            field: None,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            error: "not_found".to_string(),
            message: message.into(),
            field: None,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            error: "internal_error".to_string(),
            message: message.into(),
            field: None,
        }
    }
}
