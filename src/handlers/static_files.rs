//! Static file serving handler

use worker::*;

pub async fn serve_css(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let file = ctx.param("file").map(|s| s.as_str()).unwrap_or("");
    
    let content = match file {
        "main.css" => include_str!("../../static/css/main.css"),
        "dashboard.css" => include_str!("../../static/css/dashboard.css"),
        "animations.css" => include_str!("../../static/css/animations.css"),
        _ => return Response::error("Not Found", 404),
    };
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "text/css; charset=utf-8")?;
    headers.set("Cache-Control", "public, max-age=86400")?;
    Ok(Response::ok(content)?.with_headers(headers))
}

pub async fn serve_js(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let file = ctx.param("file").map(|s| s.as_str()).unwrap_or("");
    
    let content = match file {
        "main.js" => include_str!("../../static/js/main.js"),
        "dashboard.js" => include_str!("../../static/js/dashboard.js"),
        "conditions.js" => include_str!("../../static/js/conditions.js"),
        "animations.js" => include_str!("../../static/js/animations.js"),
        _ => return Response::error("Not Found", 404),
    };
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/javascript; charset=utf-8")?;
    headers.set("Cache-Control", "public, max-age=86400")?;
    Ok(Response::ok(content)?.with_headers(headers))
}

pub async fn serve_admin_css(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let file = ctx.param("file").map(|s| s.as_str()).unwrap_or("");
    
    let content = match file {
        "admin.css" => include_str!("../../static/admin/admin.css"),
        _ => return Response::error("Not Found", 404),
    };
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "text/css; charset=utf-8")?;
    headers.set("Cache-Control", "public, max-age=3600")?;
    Ok(Response::ok(content)?.with_headers(headers))
}

pub async fn serve_admin_js(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let file = ctx.param("file").map(|s| s.as_str()).unwrap_or("");
    
    let content = match file {
        "admin.js" => include_str!("../../static/admin/admin.js"),
        _ => return Response::error("Not Found", 404),
    };
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/javascript; charset=utf-8")?;
    headers.set("Cache-Control", "public, max-age=3600")?;
    Ok(Response::ok(content)?.with_headers(headers))
}

<<<<<<< HEAD
pub async fn serve_image(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let file = ctx.param("file").map(|s| s.as_str()).unwrap_or("");
    
    // Get R2 bucket
    let bucket = ctx.env.bucket("ASSETS")?;
    
    // Try to get the image from R2
    match bucket.get(format!("images/{}", file)).execute().await? {
        Some(object) => {
            let body = object.body().ok_or_else(|| Error::from("No body"))?;
            let bytes = body.bytes().await?;
            
            let content_type = match file.rsplit('.').next() {
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("png") => "image/png",
                Some("gif") => "image/gif",
                Some("webp") => "image/webp",
                Some("svg") => "image/svg+xml",
                _ => "application/octet-stream",
            };
            
            let mut headers = Headers::new();
            headers.set("Content-Type", content_type)?;
            headers.set("Cache-Control", "public, max-age=31536000, immutable")?;
            
            Ok(Response::from_bytes(bytes)?.with_headers(headers))
        }
        None => Response::error("Image not found", 404),
    }
}

pub async fn serve_favicon(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let bucket = ctx.env.bucket("ASSETS")?;
    
    match bucket.get("favicon.ico").execute().await? {
        Some(object) => {
            let body = object.body().ok_or_else(|| Error::from("No body"))?;
            let bytes = body.bytes().await?;
            
            let mut headers = Headers::new();
            headers.set("Content-Type", "image/x-icon")?;
            headers.set("Cache-Control", "public, max-age=604800")?;
            
            Ok(Response::from_bytes(bytes)?.with_headers(headers))
        }
        None => {
            // Fallback: try logo.png as favicon
            match bucket.get("images/logo.png").execute().await? {
                Some(object) => {
                    let body = object.body().ok_or_else(|| Error::from("No body"))?;
                    let bytes = body.bytes().await?;
                    
                    let mut headers = Headers::new();
                    headers.set("Content-Type", "image/png")?;
                    headers.set("Cache-Control", "public, max-age=604800")?;
                    
                    Ok(Response::from_bytes(bytes)?.with_headers(headers))
                }
                None => Response::error("Favicon not found", 404),
            }
        }
    }
=======
pub async fn serve_image(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // For images, we'd typically use R2 or external CDN
    // This is a placeholder - images should be served from R2 bucket
    Response::error("Images served from CDN", 302)
}

pub async fn serve_favicon(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Content-Type", "image/x-icon")?;
    headers.set("Cache-Control", "public, max-age=604800")?;
    // Return a simple placeholder or redirect to CDN
    Response::error("Favicon served from CDN", 302)
>>>>>>> 69f4b5dbcdd74d012c2ebfcef975cf9cdc95e2b2
}

pub async fn serve_robots(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let content = r#"User-agent: *
Allow: /

Sitemap: https://riverviewadventurecompany.com/sitemap.xml
"#;
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "text/plain")?;
    headers.set("Cache-Control", "public, max-age=86400")?;
    Ok(Response::ok(content)?.with_headers(headers))
}

pub async fn serve_sitemap(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://riverviewadventurecompany.com/</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>weekly</changefreq>
        <priority>1.0</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/about</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>monthly</changefreq>
        <priority>0.8</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/services</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.9</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/tubing</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.9</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/bikes</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.8</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/ebikes</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.8</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/contact</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>monthly</changefreq>
        <priority>0.7</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/conditions</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>hourly</changefreq>
        <priority>0.9</priority>
    </url>
<<<<<<< HEAD
    <url>
        <loc>https://riverviewadventurecompany.com/book</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>monthly</changefreq>
        <priority>0.9</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/waiver</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>monthly</changefreq>
        <priority>0.6</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/howitworks</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>monthly</changefreq>
        <priority>0.7</priority>
    </url>
    <url>
        <loc>https://riverviewadventurecompany.com/gallery</loc>
        <lastmod>2026-01-04</lastmod>
        <changefreq>monthly</changefreq>
        <priority>0.6</priority>
    </url>
=======
>>>>>>> 69f4b5dbcdd74d012c2ebfcef975cf9cdc95e2b2
</urlset>"#;
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/xml")?;
    headers.set("Cache-Control", "public, max-age=3600")?;
    Ok(Response::ok(content)?.with_headers(headers))
}
