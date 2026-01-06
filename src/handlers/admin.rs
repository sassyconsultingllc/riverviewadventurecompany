//! Admin dashboard handler with TOTP authentication

use worker::*;
use crate::models::*;
use crate::utils::{cache, auth};

// Admin HTML pages
pub async fn dashboard(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/admin/index.html"))
}

pub async fn login_page(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/admin/login.html"))
}

pub async fn settings_page(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/admin/settings.html"))
}

pub async fn services_page(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/admin/services.html"))
}

pub async fn analytics_page(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/admin/analytics.html"))
}

pub async fn content_page(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/admin/content.html"))
}

// Admin API endpoints
pub async fn verify_login(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: LoginRequest = match req.json().await {
        Ok(b) => b,
        Err(_) => return Response::error("Invalid request body", 400),
    };
    
    // Get stored password hash
    let stored_hash = match ctx.secret("ADMIN_PASSWORD_HASH") {
        Ok(h) => h.to_string(),
        Err(_) => return Response::error("Admin not configured", 500),
    };
    
    // Verify password
    let input_hash = auth::hash_password(&body.password);
    if input_hash != stored_hash || body.username != "admin" {
        return json_response(&LoginResponse {
            success: false,
            message: "Invalid credentials".to_string(),
            token: None,
            requires_totp: false,
        });
    }
    
    // Generate temporary token for TOTP verification
    let token = auth::generate_session_token();
    
    // Store token temporarily (5 minutes)
    if let Ok(kv) = ctx.kv("CACHE") {
        let _ = cache::set_cached(&kv, &format!("totp_pending:{}", token), &true, 300).await;
    }
    
    json_response(&LoginResponse {
        success: true,
        message: "Password verified. Enter TOTP code.".to_string(),
        token: Some(token),
        requires_totp: true,
    })
}

pub async fn verify_totp(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: TotpVerifyRequest = match req.json().await {
        Ok(b) => b,
        Err(_) => return Response::error("Invalid request body", 400),
    };
    
    // Verify pending token exists
    if let Some(token) = &body.token {
        if let Ok(kv) = ctx.kv("CACHE") {
            let pending: Option<bool> = cache::get_cached(&kv, &format!("totp_pending:{}", token)).await;
            if pending.is_none() {
                return json_response(&TotpVerifyResponse {
                    valid: false,
                    message: "Session expired. Please login again.".to_string(),
                    session_token: None,
                });
            }
        }
    } else {
        return json_response(&TotpVerifyResponse {
            valid: false,
            message: "Missing token".to_string(),
            session_token: None,
        });
    }
    
    // Get TOTP secret
    let totp_secret = match ctx.secret("TOTP_SECRET") {
        Ok(s) => s.to_string(),
        Err(_) => return Response::error("TOTP not configured", 500),
    };
    
    // Verify TOTP code
    if !auth::verify_totp(&totp_secret, &body.code) {
        return json_response(&TotpVerifyResponse {
            valid: false,
            message: "Invalid TOTP code".to_string(),
            session_token: None,
        });
    }
    
    // Generate session token
    let session_token = auth::generate_session_token();
    
    // Store session (24 hours)
    if let Ok(kv) = ctx.kv("CACHE") {
        let session = AdminSession {
            user_id: "admin".to_string(),
            username: "admin".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            expires_at: (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
            ip_address: req.headers().get("CF-Connecting-IP").unwrap_or_default().unwrap_or_default(),
        };
        let _ = cache::set_cached(&kv, &format!("session:{}", session_token), &session, 86400).await;
        
        // Clean up pending token
        if let Some(token) = &body.token {
            let _ = kv.delete(&format!("totp_pending:{}", token)).await;
        }
    }
    
    json_response(&TotpVerifyResponse {
        valid: true,
        message: "Login successful".to_string(),
        session_token: Some(session_token),
    })
}

pub async fn get_settings(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if !verify_session(&req, &ctx).await {
        return Response::error("Unauthorized", 401);
    }
    
    let settings = if let Ok(kv) = ctx.kv("SETTINGS") {
        cache::get_cached::<SiteSettings>(&kv, "site_settings").await
            .unwrap_or_default()
    } else {
        SiteSettings::default()
    };
    
    json_response(&settings)
}

pub async fn update_settings(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if !verify_session(&req, &ctx).await {
        return Response::error("Unauthorized", 401);
    }
    
    let settings: SiteSettings = match req.json().await {
        Ok(s) => s,
        Err(_) => return Response::error("Invalid settings data", 400),
    };
    
    if let Ok(kv) = ctx.kv("SETTINGS") {
        match cache::set_cached(&kv, "site_settings", &settings, 0).await {
            Ok(_) => json_response(&serde_json::json!({"success": true})),
            Err(e) => Response::error(format!("Failed to save settings: {}", e), 500),
        }
    } else {
        Response::error("Settings storage not available", 500)
    }
}

pub async fn update_services(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if !verify_session(&req, &ctx).await {
        return Response::error("Unauthorized", 401);
    }
    
    let services: ServicesConfig = match req.json().await {
        Ok(s) => s,
        Err(_) => return Response::error("Invalid services data", 400),
    };
    
    if let Ok(kv) = ctx.kv("SETTINGS") {
        match cache::set_cached(&kv, "services", &services, 0).await {
            Ok(_) => json_response(&serde_json::json!({"success": true})),
            Err(e) => Response::error(format!("Failed to save services: {}", e), 500),
        }
    } else {
        Response::error("Settings storage not available", 500)
    }
}

pub async fn update_thresholds(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if !verify_session(&req, &ctx).await {
        return Response::error("Unauthorized", 401);
    }
    
    let thresholds: FlowThresholds = match req.json().await {
        Ok(t) => t,
        Err(_) => return Response::error("Invalid thresholds data", 400),
    };
    
    if let Ok(kv) = ctx.kv("SETTINGS") {
        match cache::set_cached(&kv, "flow_thresholds", &thresholds, 0).await {
            Ok(_) => json_response(&serde_json::json!({"success": true})),
            Err(e) => Response::error(format!("Failed to save thresholds: {}", e), 500),
        }
    } else {
        Response::error("Settings storage not available", 500)
    }
}

pub async fn update_content(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if !verify_session(&req, &ctx).await {
        return Response::error("Unauthorized", 401);
    }
    
    let content: ContentUpdate = match req.json().await {
        Ok(c) => c,
        Err(_) => return Response::error("Invalid content data", 400),
    };
    
    if let Ok(kv) = ctx.kv("SETTINGS") {
        let key = format!("content:{}:{}", content.page, content.section);
        match cache::set_cached(&kv, &key, &content.content, 0).await {
            Ok(_) => json_response(&serde_json::json!({"success": true})),
            Err(e) => Response::error(format!("Failed to save content: {}", e), 500),
        }
    } else {
        Response::error("Settings storage not available", 500)
    }
}

pub async fn get_analytics(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if !verify_session(&req, &ctx).await {
        return Response::error("Unauthorized", 401);
    }
    
    // Return placeholder analytics (would integrate with Cloudflare Analytics in production)
    let analytics = AnalyticsData {
        page_views: PageViewStats {
            today: 0,
            week: 0,
            month: 0,
            total: 0,
        },
        api_calls: ApiCallStats {
            flow_api: 0,
            weather_api: 0,
            conditions_api: 0,
        },
        popular_pages: vec![],
        conditions_checks: 0,
        contact_submissions: 0,
    };
    
    json_response(&analytics)
}

// Helper functions
async fn verify_session(req: &Request, ctx: &RouteContext<()>) -> bool {
    let auth_header = match req.headers().get("Authorization").ok().flatten() {
        Some(h) => h,
        None => return false,
    };
    
    let token = auth_header.strip_prefix("Bearer ").unwrap_or(&auth_header);
    
    if let Ok(kv) = ctx.kv("CACHE") {
        let session: Option<AdminSession> = cache::get_cached(&kv, &format!("session:{}", token)).await;
        session.is_some()
    } else {
        false
    }
}

fn html_response(content: &str) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Content-Type", "text/html; charset=utf-8")?;
    headers.set("X-Frame-Options", "DENY")?;
    headers.set("X-Content-Type-Options", "nosniff")?;
    headers.set("Referrer-Policy", "strict-origin-when-cross-origin")?;
    Ok(Response::ok(content)?.with_headers(headers))
}

fn json_response<T: serde::Serialize>(data: &T) -> Result<Response> {
    let json = serde_json::to_string(data).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", "no-store")?;
    Ok(Response::ok(json)?.with_headers(headers))
}
