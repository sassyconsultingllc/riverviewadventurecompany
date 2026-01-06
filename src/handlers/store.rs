//! Store handler for e-bikes and repair services

use worker::*;
use crate::models::{default_velotric_products, default_repair_services};
use crate::utils::cache;

pub async fn get_bikes(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Try to get from KV settings (for custom pricing)
    let bikes = if let Ok(kv) = ctx.kv("SETTINGS") {
        cache::get_cached(&kv, "bikes").await
            .unwrap_or_else(|| default_velotric_products())
    } else {
        default_velotric_products()
    };
    
    let json = serde_json::to_string(&bikes).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "public, max-age=300")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::ok(json)?.with_headers(headers))
}

pub async fn get_repairs(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Try to get from KV settings (for custom pricing)
    let repairs = if let Ok(kv) = ctx.kv("SETTINGS") {
        cache::get_cached(&kv, "repairs").await
            .unwrap_or_else(|| default_repair_services())
    } else {
        default_repair_services()
    };
    
    let json = serde_json::to_string(&repairs).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "public, max-age=300")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::ok(json)?.with_headers(headers))
}
