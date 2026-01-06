//! River flow data models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowData {
    pub flow_cfs: f64,
    pub water_temp_f: Option<f64>,
    pub timestamp: String,
    pub station_id: String,
    pub station_name: String,
    pub status: FlowStatus,
    pub change_4h: Option<f64>,
    pub change_12h: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FlowStatus {
    Safe,
    Caution,
    Danger,
    Unknown,
}

impl FlowStatus {
    pub fn from_cfs(cfs: f64, thresholds: &FlowThresholds) -> Self {
        if cfs < thresholds.safe_max {
            FlowStatus::Safe
        } else if cfs < thresholds.caution_max {
            FlowStatus::Caution
        } else {
            FlowStatus::Danger
        }
    }
    
    pub fn message(&self) -> &'static str {
        match self {
            FlowStatus::Safe => "Conditions are ideal for all water activities",
            FlowStatus::Caution => "Exercise caution - elevated water levels",
            FlowStatus::Danger => "Water activities suspended due to high flow",
            FlowStatus::Unknown => "Unable to determine current conditions",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            FlowStatus::Safe => "#22c55e",
            FlowStatus::Caution => "#eab308",
            FlowStatus::Danger => "#ef4444",
            FlowStatus::Unknown => "#6b7280",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowThresholds {
    pub safe_max: f64,
    pub caution_max: f64,
}

impl Default for FlowThresholds {
    fn default() -> Self {
        Self {
            safe_max: 8000.0,
            caution_max: 15000.0,
        }
    }
}

// USGS API Response structures
#[derive(Debug, Deserialize)]
pub struct UsgsResponse {
    pub value: UsgsValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsgsValue {
    pub time_series: Vec<UsgsTimeSeries>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsgsTimeSeries {
    pub source_info: UsgsSourceInfo,
    pub variable: UsgsVariable,
    pub values: Vec<UsgsValues>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsgsSourceInfo {
    pub site_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsgsVariable {
    pub variable_code: Vec<UsgsVariableCode>,
}

#[derive(Debug, Deserialize)]
pub struct UsgsVariableCode {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct UsgsValues {
    pub value: Vec<UsgsDataValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsgsDataValue {
    pub value: String,
    pub date_time: String,
}
