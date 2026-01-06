//! Moon phase handler

use worker::*;
use chrono::Utc;
use crate::models::{MoonData, SunData};

#[derive(serde::Serialize)]
struct MoonResponse {
    moon: MoonData,
    sun: SunData,
}

pub async fn get_moon_phase(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let lat: f64 = ctx.var("LOCATION_LAT")
        .map(|v| v.to_string().parse().unwrap_or(43.2722))
        .unwrap_or(43.2722);
    let lon: f64 = ctx.var("LOCATION_LON")
        .map(|v| v.to_string().parse().unwrap_or(-89.7208))
        .unwrap_or(-89.7208);
    
    let now = Utc::now();
    let year = now.format("%Y").to_string().parse().unwrap_or(2026);
    let month = now.format("%m").to_string().parse().unwrap_or(1);
    let day = now.format("%d").to_string().parse().unwrap_or(1);
    
    let moon = MoonData::calculate(year, month, day);
    let sun = SunData::calculate(year, month, day, lat, lon);
    
    let response = MoonResponse { moon, sun };
    
    let json = serde_json::to_string(&response).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "public, max-age=3600")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::ok(json)?.with_headers(headers))
}
