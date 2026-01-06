//! Contact form handler

use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ContactFormData {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub subject: String,
    pub message: String,
    pub service: Option<String>,
}

#[derive(Serialize)]
pub struct ContactResponse {
    pub success: bool,
    pub message: String,
}

pub async fn submit_form(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Rate limiting check (simple implementation)
    let client_ip = req.headers()
        .get("CF-Connecting-IP")
        .ok()
        .flatten()
        .unwrap_or_else(|| "unknown".to_string());
    
    if let Ok(kv) = ctx.kv("CACHE") {
        let rate_key = format!("rate:contact:{}", client_ip);
        let count: Option<i32> = crate::utils::cache::get_cached(&kv, &rate_key).await;
        
        if let Some(c) = count {
            if c >= 5 {
                return json_response(&ContactResponse {
                    success: false,
                    message: "Too many requests. Please try again later.".to_string(),
                });
            }
            let _ = crate::utils::cache::set_cached(&kv, &rate_key, &(c + 1), 3600).await;
        } else {
            let _ = crate::utils::cache::set_cached(&kv, &rate_key, &1, 3600).await;
        }
    }
    
    // Parse form data
    let form_data: ContactFormData = match req.json().await {
        Ok(d) => d,
        Err(_) => return json_response(&ContactResponse {
            success: false,
            message: "Invalid form data".to_string(),
        }),
    };
    
    // Validate required fields
    if form_data.name.trim().is_empty() || 
       form_data.email.trim().is_empty() || 
       form_data.message.trim().is_empty() {
        return json_response(&ContactResponse {
            success: false,
            message: "Please fill in all required fields".to_string(),
        });
    }
    
    // Validate email format
    if !form_data.email.contains('@') || !form_data.email.contains('.') {
        return json_response(&ContactResponse {
            success: false,
            message: "Please enter a valid email address".to_string(),
        });
    }
    
    // Store submission in KV for later retrieval
    if let Ok(kv) = ctx.kv("SETTINGS") {
        let submission_id = format!("contact:{}:{}", 
            chrono::Utc::now().timestamp(),
            &client_ip[..8.min(client_ip.len())]
        );
        
        let submission = serde_json::json!({
            "name": form_data.name,
            "email": form_data.email,
            "phone": form_data.phone,
            "subject": form_data.subject,
            "message": form_data.message,
            "service": form_data.service,
            "submitted_at": chrono::Utc::now().to_rfc3339(),
            "ip": client_ip,
        });
        
        let _ = kv.put(&submission_id, submission.to_string())
            .map_err(|e| Error::from(e.to_string()))?
            .expiration_ttl(30 * 24 * 3600) // 30 days
            .execute()
            .await;
    }
    
    // In production, you would send an email here using a service like SendGrid or Mailgun
    // For now, we just store the submission
    
    json_response(&ContactResponse {
        success: true,
        message: "Thank you for your message! We'll get back to you soon.".to_string(),
    })
}

fn json_response<T: serde::Serialize>(data: &T) -> Result<Response> {
    let json = serde_json::to_string(data).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::ok(json)?.with_headers(headers))
}
