//! River flow data handler

use worker::*;
use crate::api::usgs;
use crate::models::{FlowData, FlowStatus, FlowThresholds};
use crate::utils::cache;

pub async fn get_flow_data(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let station_id = ctx.var("USGS_STATION_ID")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "05406000".to_string());
    
    // Try to get from cache first
    let cache_key = format!("flow:{}", station_id);
    
    if let Ok(kv) = ctx.kv("CACHE") {
        if let Some(cached) = cache::get_cached::<FlowData>(&kv, &cache_key).await {
            return json_response(&cached);
        }
    }
    
    // Fetch fresh data
    match usgs::fetch_current_flow(&station_id).await {
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
                                "00060" => flow_cfs = Some(value), // Discharge
                                "00010" => water_temp = Some(value * 9.0 / 5.0 + 32.0), // Temp C to F
                                _ => {}
                            }
                        }
                    }
                }
            }
            
            let cfs = flow_cfs.unwrap_or(0.0);
            let thresholds = FlowThresholds::default();
            
            let flow_data = FlowData {
                flow_cfs: cfs,
                water_temp_f: water_temp,
                timestamp,
                station_id: station_id.clone(),
                station_name,
                status: FlowStatus::from_cfs(cfs, &thresholds),
                change_4h: None,
                change_12h: None,
            };
            
            // Cache for 5 minutes
            if let Ok(kv) = ctx.kv("CACHE") {
                let _ = cache::set_cached(&kv, &cache_key, &flow_data, 300).await;
            }
            
            json_response(&flow_data)
        }
        Err(e) => Response::error(format!("Failed to fetch flow data: {}", e), 500),
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
