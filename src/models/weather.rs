//! Weather data models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature_f: f64,
    pub feels_like_f: f64,
    pub humidity: f64,
    pub wind_speed_mph: f64,
    pub wind_direction: String,
    pub wind_gust_mph: Option<f64>,
    pub precipitation_in: f64,
    pub visibility_mi: f64,
    pub uv_index: f64,
    pub conditions: String,
    pub conditions_code: i32,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherAlert {
    pub id: String,
    pub event: String,
    pub headline: String,
    pub description: String,
    pub severity: AlertSeverity,
    pub urgency: String,
    pub effective: String,
    pub expires: String,
    pub sender: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    Extreme,
    Severe,
    Moderate,
    Minor,
    Unknown,
}

impl From<&str> for AlertSeverity {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "extreme" => AlertSeverity::Extreme,
            "severe" => AlertSeverity::Severe,
            "moderate" => AlertSeverity::Moderate,
            "minor" => AlertSeverity::Minor,
            _ => AlertSeverity::Unknown,
        }
    }
}

// Tomorrow.io API Response
#[derive(Debug, Deserialize)]
pub struct TomorrowIoResponse {
    pub data: TomorrowIoData,
}

#[derive(Debug, Deserialize)]
pub struct TomorrowIoData {
    pub timelines: Vec<TomorrowIoTimeline>,
}

#[derive(Debug, Deserialize)]
pub struct TomorrowIoTimeline {
    pub intervals: Vec<TomorrowIoInterval>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TomorrowIoInterval {
    pub start_time: String,
    pub values: TomorrowIoValues,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TomorrowIoValues {
    pub temperature: Option<f64>,
    pub temperature_apparent: Option<f64>,
    pub humidity: Option<f64>,
    pub wind_speed: Option<f64>,
    pub wind_direction: Option<f64>,
    pub wind_gust: Option<f64>,
    pub precipitation_intensity: Option<f64>,
    pub visibility: Option<f64>,
    pub uv_index: Option<f64>,
    pub weather_code: Option<i32>,
}

// NWS Alert Response
#[derive(Debug, Deserialize)]
pub struct NwsAlertResponse {
    pub features: Vec<NwsAlertFeature>,
}

#[derive(Debug, Deserialize)]
pub struct NwsAlertFeature {
    pub id: String,
    pub properties: NwsAlertProperties,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NwsAlertProperties {
    pub event: String,
    pub headline: Option<String>,
    pub description: Option<String>,
    pub severity: Option<String>,
    pub urgency: Option<String>,
    pub effective: Option<String>,
    pub expires: Option<String>,
    pub sender_name: Option<String>,
}

/// Convert wind direction degrees to compass direction
pub fn degrees_to_compass(degrees: f64) -> String {
    let directions = ["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE",
                      "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW"];
    let index = ((degrees + 11.25) / 22.5) as usize % 16;
    directions[index].to_string()
}

/// Convert Tomorrow.io weather code to human-readable conditions
pub fn weather_code_to_conditions(code: i32) -> String {
    match code {
        0 => "Unknown",
        1000 => "Clear",
        1100 => "Mostly Clear",
        1101 => "Partly Cloudy",
        1102 => "Mostly Cloudy",
        1001 => "Cloudy",
        2000 => "Fog",
        2100 => "Light Fog",
        4000 => "Drizzle",
        4001 => "Rain",
        4200 => "Light Rain",
        4201 => "Heavy Rain",
        5000 => "Snow",
        5001 => "Flurries",
        5100 => "Light Snow",
        5101 => "Heavy Snow",
        6000 => "Freezing Drizzle",
        6001 => "Freezing Rain",
        6200 => "Light Freezing Rain",
        6201 => "Heavy Freezing Rain",
        7000 => "Ice Pellets",
        7101 => "Heavy Ice Pellets",
        7102 => "Light Ice Pellets",
        8000 => "Thunderstorm",
        _ => "Unknown",
    }.to_string()
}
