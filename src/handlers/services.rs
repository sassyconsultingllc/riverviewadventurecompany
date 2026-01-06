//! Services status handler

use worker::*;
use crate::models::ServicesConfig;
use crate::utils::cache;

pub async fn get_services(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Try to get from KV settings
    let services = if let Ok(kv) = ctx.kv("SETTINGS") {
        cache::get_cached::<ServicesConfig>(&kv, "services").await
            .unwrap_or_default()
    } else {
        ServicesConfig::default()
    };
    
    let json = serde_json::to_string(&services).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "public, max-age=60")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::ok(json)?.with_headers(headers))
}
