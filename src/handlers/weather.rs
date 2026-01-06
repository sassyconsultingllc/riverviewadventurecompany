//! Weather data handler

use worker::*;
use crate::api::weather as weather_api;
use crate::models::{WeatherData, WeatherAlert, AlertSeverity, degrees_to_compass, weather_code_to_conditions};
use crate::utils::cache;

pub async fn get_weather_data(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let lat = ctx.var("LOCATION_LAT")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "43.2722".to_string());
    let lon = ctx.var("LOCATION_LON")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "-89.7208".to_string());
    
    // Try cache first
    let cache_key = format!("weather:{}:{}", lat, lon);
    
    if let Ok(kv) = ctx.kv("CACHE") {
        if let Some(cached) = cache::get_cached::<WeatherData>(&kv, &cache_key).await {
            return json_response(&cached);
        }
    }
    
    // Get API key
    let api_key = match ctx.secret("TOMORROW_IO_API_KEY") {
        Ok(key) => key.to_string(),
        Err(_) => return Response::error("Weather API not configured", 500),
    };
    
    // Fetch from Tomorrow.io
    match weather_api::fetch_tomorrow_io(&api_key, &lat, &lon).await {
        Ok(response) => {
            if let Some(timeline) = response.data.timelines.first() {
                if let Some(interval) = timeline.intervals.first() {
                    let values = &interval.values;
                    
                    let weather_data = WeatherData {
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
                    };
                    
                    // Cache for 10 minutes
                    if let Ok(kv) = ctx.kv("CACHE") {
                        let _ = cache::set_cached(&kv, &cache_key, &weather_data, 600).await;
                    }
                    
                    return json_response(&weather_data);
                }
            }
            Response::error("Invalid weather data", 500)
        }
        Err(e) => Response::error(format!("Failed to fetch weather: {}", e), 500),
    }
}

pub async fn get_weather_alerts(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let zone = ctx.var("NWS_ZONE")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "WIZ061".to_string());
    
    // Try cache first
    let cache_key = format!("alerts:{}", zone);
    
    if let Ok(kv) = ctx.kv("CACHE") {
        if let Some(cached) = cache::get_cached::<Vec<WeatherAlert>>(&kv, &cache_key).await {
            return json_response(&cached);
        }
    }
    
    // Fetch from NWS
    match weather_api::fetch_nws_alerts(&zone).await {
        Ok(response) => {
            let alerts: Vec<WeatherAlert> = response.features.iter().map(|f| {
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
            }).collect();
            
            // Cache for 5 minutes
            if let Ok(kv) = ctx.kv("CACHE") {
                let _ = cache::set_cached(&kv, &cache_key, &alerts, 300).await;
            }
            
            json_response(&alerts)
        }
        Err(e) => Response::error(format!("Failed to fetch alerts: {}", e), 500),
    }
}

fn json_response<T: serde::Serialize>(data: &T) -> Result<Response> {
    let json = serde_json::to_string(data).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "public, max-age=300")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::ok(json)?.with_headers(headers))
}
