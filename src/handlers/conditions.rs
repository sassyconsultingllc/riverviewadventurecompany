//! Combined conditions handler

use worker::*;
use serde::{Serialize, Deserialize};
use crate::api::{usgs, weather as weather_api};
use crate::models::*;
use crate::utils::cache;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
pub struct ConditionsResponse {
    pub flow: Option<FlowData>,
    pub weather: Option<WeatherData>,
    pub alerts: Vec<WeatherAlert>,
    pub moon: MoonData,
    pub sun: SunData,
    pub services: ServicesConfig,
    pub timestamp: String,
}

pub async fn get_all_conditions(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let station_id = ctx.var("USGS_STATION_ID")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "05406000".to_string());
    let lat = ctx.var("LOCATION_LAT")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "43.2722".to_string());
    let lon = ctx.var("LOCATION_LON")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "-89.7208".to_string());
    let zone = ctx.var("NWS_ZONE")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "WIZ061".to_string());
    
    // Try cache first
    let cache_key = "conditions:all";
    
    if let Ok(kv) = ctx.kv("CACHE") {
        if let Some(cached) = cache::get_cached::<ConditionsResponse>(&kv, cache_key).await {
            return json_response(&cached);
        }
    }
    
    // Fetch flow data
    let flow_data = match usgs::fetch_current_flow(&station_id).await {
        Ok(usgs_data) => {
            let mut flow_cfs: Option<f64> = None;
            let mut water_temp: Option<f64> = None;
            let mut timestamp = String::new();
            let mut station_name = String::new();
            
            for ts in &usgs_data.value.time_series {
                station_name = ts.source_info.site_name.clone();
                
                if let Some(var_code) = ts.variable.variable_code.first() {
                    if let Some(values) = ts.values.first() {
                        if let Some(data_value) = values.value.first() {
                            let value: f64 = data_value.value.parse().unwrap_or(0.0);
                            timestamp = data_value.date_time.clone();
                            
                            match var_code.value.as_str() {
                                "00060" => flow_cfs = Some(value),
                                "00010" => water_temp = Some(value * 9.0 / 5.0 + 32.0),
                                _ => {}
                            }
                        }
                    }
                }
            }
            
            flow_cfs.map(|cfs| {
                let thresholds = FlowThresholds::default();
                FlowData {
                    flow_cfs: cfs,
                    water_temp_f: water_temp,
                    timestamp,
                    station_id: station_id.clone(),
                    station_name,
                    status: FlowStatus::from_cfs(cfs, &thresholds),
                    change_4h: None,
                    change_12h: None,
                }
            })
        }
        Err(_) => None,
    };
    
    // Fetch weather data
    let weather_data = if let Ok(api_key) = ctx.secret("TOMORROW_IO_API_KEY") {
        match weather_api::fetch_tomorrow_io(&api_key.to_string(), &lat, &lon).await {
            Ok(response) => {
                response.data.timelines.first()
                    .and_then(|t| t.intervals.first())
                    .map(|interval| {
                        let values = &interval.values;
                        WeatherData {
                            temperature_f: values.temperature.unwrap_or(0.0),
                            feels_like_f: values.temperature_apparent.unwrap_or(0.0),
                            humidity: values.humidity.unwrap_or(0.0),
                            wind_speed_mph: values.wind_speed.unwrap_or(0.0),
                            wind_direction: degrees_to_compass(values.wind_direction.unwrap_or(0.0)),
                            wind_gust_mph: values.wind_gust,
                            precipitation_in: values.precipitation_intensity.unwrap_or(0.0),
                            visibility_mi: values.visibility.unwrap_or(10.0),
                            uv_index: values.uv_index.unwrap_or(0.0),
                            conditions: weather_code_to_conditions(values.weather_code.unwrap_or(0)),
                            conditions_code: values.weather_code.unwrap_or(0),
                            timestamp: interval.start_time.clone(),
                        }
                    })
            }
            Err(_) => None,
        }
    } else {
        None
    };
    
    // Fetch alerts
    let alerts = match weather_api::fetch_nws_alerts(&zone).await {
        Ok(response) => {
            response.features.iter().map(|f| {
                WeatherAlert {
                    id: f.id.clone(),
                    event: f.properties.event.clone(),
                    headline: f.properties.headline.clone().unwrap_or_default(),
                    description: f.properties.description.clone().unwrap_or_default(),
                    severity: AlertSeverity::from(f.properties.severity.as_deref().unwrap_or("unknown")),
                    urgency: f.properties.urgency.clone().unwrap_or_default(),
                    effective: f.properties.effective.clone().unwrap_or_default(),
                    expires: f.properties.expires.clone().unwrap_or_default(),
                    sender: f.properties.sender_name.clone().unwrap_or_default(),
                }
            }).collect()
        }
        Err(_) => vec![],
    };
    
    // Calculate moon and sun
    let now = Utc::now();
    let year = now.format("%Y").to_string().parse().unwrap_or(2026);
    let month = now.format("%m").to_string().parse().unwrap_or(1);
    let day = now.format("%d").to_string().parse().unwrap_or(1);
    let lat_f: f64 = lat.parse().unwrap_or(43.2722);
    let lon_f: f64 = lon.parse().unwrap_or(-89.7208);
    
    let moon = MoonData::calculate(year, month, day);
    let sun = SunData::calculate(year, month, day, lat_f, lon_f);
    
    // Get services from KV or use defaults
    let services = if let Ok(kv) = ctx.kv("SETTINGS") {
        cache::get_cached::<ServicesConfig>(&kv, "services").await
            .unwrap_or_default()
    } else {
        ServicesConfig::default()
    };
    
    let response = ConditionsResponse {
        flow: flow_data,
        weather: weather_data,
        alerts,
        moon,
        sun,
        services,
        timestamp: Utc::now().to_rfc3339(),
    };
    
    // Cache for 5 minutes
    if let Ok(kv) = ctx.kv("CACHE") {
        let _ = cache::set_cached(&kv, cache_key, &response, 300).await;
    }
    
    json_response(&response)
}

fn json_response<T: serde::Serialize>(data: &T) -> Result<Response> {
    let json = serde_json::to_string(data).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "public, max-age=300")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::ok(json)?.with_headers(headers))
}
