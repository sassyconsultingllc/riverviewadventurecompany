//! Historical data handler

use worker::*;
use chrono::{Utc, Duration};
use crate::api::usgs;
use crate::models::FlowData;

pub async fn get_period_data(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let period = ctx.param("period").map(|s| s.as_str()).unwrap_or("yesterday");
    let station_id = ctx.var("USGS_STATION_ID")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "05406000".to_string());
    
    let now = Utc::now();
    let (start_date, end_date) = match period {
        "yesterday" => {
            let yesterday = now - Duration::days(1);
            (yesterday.format("%Y-%m-%d").to_string(), yesterday.format("%Y-%m-%d").to_string())
        }
        "lastweek" => {
            let week_ago = now - Duration::days(7);
            (week_ago.format("%Y-%m-%d").to_string(), week_ago.format("%Y-%m-%d").to_string())
        }
        "lastyear" => {
            let year_ago = now - Duration::days(365);
            (year_ago.format("%Y-%m-%d").to_string(), year_ago.format("%Y-%m-%d").to_string())
        }
        _ => return Response::error("Invalid period. Use: yesterday, lastweek, lastyear", 400),
    };
    
    match usgs::fetch_historical_flow(&station_id, &start_date, &end_date).await {
        Ok(usgs_data) => {
            let mut flow_values: Vec<f64> = vec![];
            
            for ts in &usgs_data.value.time_series {
                if let Some(var_code) = ts.variable.variable_code.first() {
                    if var_code.value == "00060" {
                        if let Some(values) = ts.values.first() {
                            for data_value in &values.value {
                                if let Ok(value) = data_value.value.parse::<f64>() {
                                    flow_values.push(value);
                                }
                            }
                        }
                    }
                }
            }
            
            let avg_flow = if !flow_values.is_empty() {
                flow_values.iter().sum::<f64>() / flow_values.len() as f64
            } else {
                0.0
            };
            
            let response = serde_json::json!({
                "period": period,
                "start_date": start_date,
                "end_date": end_date,
                "average_flow_cfs": avg_flow,
                "data_points": flow_values.len(),
                "min_flow_cfs": flow_values.iter().cloned().fold(f64::INFINITY, f64::min),
                "max_flow_cfs": flow_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            });
            
            let json = serde_json::to_string(&response).map_err(|e| Error::from(e.to_string()))?;
            let mut headers = Headers::new();
            headers.set("Content-Type", "application/json")?;
            headers.set("Cache-Control", "public, max-age=3600")?;
            headers.set("Access-Control-Allow-Origin", "*")?;
            Ok(Response::ok(json)?.with_headers(headers))
        }
        Err(e) => Response::error(format!("Failed to fetch historical data: {}", e), 500),
    }
}
