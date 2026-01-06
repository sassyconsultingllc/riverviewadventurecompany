//! Public settings handler

use worker::*;
use crate::models::{SiteSettings, PublicSettings};
use crate::utils::cache;

pub async fn get_public_settings(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Try to get from KV settings
    let settings = if let Ok(kv) = ctx.kv("SETTINGS") {
        cache::get_cached::<SiteSettings>(&kv, "site_settings").await
            .unwrap_or_default()
    } else {
        SiteSettings::default()
    };
    
    let public_settings = PublicSettings::from(&settings);
    
    let json = serde_json::to_string(&public_settings).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "public, max-age=300")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::ok(json)?.with_headers(headers))
}
