//! Service status models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub id: String,
    pub name: String,
    pub status: ServiceState,
    pub message: String,
    pub icon: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceState {
    Open,
    Limited,
    Closed,
    Seasonal,
}

impl ServiceState {
    pub fn color(&self) -> &'static str {
        match self {
            ServiceState::Open => "#22c55e",
            ServiceState::Limited => "#eab308",
            ServiceState::Closed => "#ef4444",
            ServiceState::Seasonal => "#3b82f6",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub tubing: ServiceStatus,
    pub kayak: ServiceStatus,
    pub ebike: ServiceStatus,
    pub bike_rental: ServiceStatus,
    pub bike_repair: ServiceStatus,
}

impl Default for ServicesConfig {
    fn default() -> Self {
        Self {
            tubing: ServiceStatus {
                id: "tubing".to_string(),
                name: "River Tubing".to_string(),
                status: ServiceState::Seasonal,
                message: "Open Memorial Day through Labor Day".to_string(),
                icon: "water".to_string(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            kayak: ServiceStatus {
                id: "kayak".to_string(),
                name: "Kayak & Canoe".to_string(),
                status: ServiceState::Seasonal,
                message: "Available seasonally".to_string(),
                icon: "kayak".to_string(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            ebike: ServiceStatus {
                id: "ebike".to_string(),
                name: "E-Bike Sales & Rentals".to_string(),
                status: ServiceState::Open,
                message: "Authorized Velotric dealer".to_string(),
                icon: "bike".to_string(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            bike_rental: ServiceStatus {
                id: "bike_rental".to_string(),
                name: "Bike Rentals".to_string(),
                status: ServiceState::Open,
                message: "Available by appointment".to_string(),
                icon: "bicycle".to_string(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
            bike_repair: ServiceStatus {
                id: "bike_repair".to_string(),
                name: "Bike Repairs".to_string(),
                status: ServiceState::Open,
                message: "Full service bike shop".to_string(),
                icon: "tools".to_string(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
        }
    }
}
