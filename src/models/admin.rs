//! Admin authentication and dashboard models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub requires_totp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpVerifyRequest {
    pub code: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpVerifyResponse {
    pub valid: bool,
    pub message: String,
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSession {
    pub user_id: String,
    pub username: String,
    pub created_at: String,
    pub expires_at: String,
    pub ip_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsData {
    pub page_views: PageViewStats,
    pub api_calls: ApiCallStats,
    pub popular_pages: Vec<PageStat>,
    pub conditions_checks: i64,
    pub contact_submissions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageViewStats {
    pub today: i64,
    pub week: i64,
    pub month: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCallStats {
    pub flow_api: i64,
    pub weather_api: i64,
    pub conditions_api: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageStat {
    pub path: String,
    pub views: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentUpdate {
    pub page: String,
    pub section: String,
    pub content: String,
}
