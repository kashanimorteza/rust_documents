//--------------------------------------------------------------------------------- Location
// src/logics/general.rs

//--------------------------------------------------------------------------------- Description
// General response models

//--------------------------------------------------------------------------------- Import
use serde::{Deserialize, Serialize};

//--------------------------------------------------------------------------------- Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelOutput<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ModelOutput<T> {
    pub fn success(data: T, message: String) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
            error: None,
        }
    }

    pub fn success_with_message(message: String) -> Self {
        Self {
            success: true,
            message,
            data: None,
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message: "Error".to_string(),
            data: None,
            error: Some(message),
        }
    }
}
