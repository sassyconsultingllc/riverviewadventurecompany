//! Riverview Adventure Company - Complete Website & Dashboard
//! 
//! A modern, high-performance website built with Rust and Cloudflare Workers
//! featuring real-time river conditions, weather data, and admin dashboard.

use worker::*;

mod api;
mod handlers;
mod models;
mod utils;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        // ============================================
        // PUBLIC WEBSITE ROUTES
        // ============================================
        .get_async("/", |req, ctx| async move { handlers::pages::home(req, ctx).await })
        .get_async("/about", |req, ctx| async move { handlers::pages::about(req, ctx).await })
        .get_async("/services", |req, ctx| async move { handlers::pages::services(req, ctx).await })
        .get_async("/tubing", |req, ctx| async move { handlers::pages::tubing(req, ctx).await })
        .get_async("/bikes", |req, ctx| async move { handlers::pages::bikes(req, ctx).await })
        .get_async("/ebikes", |req, ctx| async move { handlers::pages::ebikes(req, ctx).await })
        .get_async("/contact", |req, ctx| async move { handlers::pages::contact(req, ctx).await })
        .get_async("/waiver", |req, ctx| async move { handlers::pages::waiver(req, ctx).await })
        .get_async("/howitworks", |req, ctx| async move { handlers::pages::how_it_works(req, ctx).await })
        .get_async("/gallery", |req, ctx| async move { handlers::pages::gallery(req, ctx).await })
        .get_async("/conditions", |req, ctx| async move { handlers::pages::conditions(req, ctx).await })
        .get_async("/book", |req, ctx| async move { handlers::pages::book(req, ctx).await })
        .get_async("/privacy", |req, ctx| async move { handlers::pages::privacy(req, ctx).await })
        .get_async("/terms", |req, ctx| async move { handlers::pages::terms(req, ctx).await })
        
        // ============================================
        // STATIC ASSETS
        // ============================================
        .get_async("/css/:file", |req, ctx| async move { handlers::static_files::serve_css(req, ctx).await })
        .get_async("/js/:file", |req, ctx| async move { handlers::static_files::serve_js(req, ctx).await })
        .get_async("/images/:file", |req, ctx| async move { handlers::static_files::serve_image(req, ctx).await })
        .get_async("/favicon.ico", |req, ctx| async move { handlers::static_files::serve_favicon(req, ctx).await })
        .get_async("/robots.txt", |req, ctx| async move { handlers::static_files::serve_robots(req, ctx).await })
        .get_async("/sitemap.xml", |req, ctx| async move { handlers::static_files::serve_sitemap(req, ctx).await })
        
        // ============================================
        // PUBLIC API ENDPOINTS
        // ============================================
        .get_async("/api/flow", |req, ctx| async move { handlers::flow::get_flow_data(req, ctx).await })
        .get_async("/api/weather", |req, ctx| async move { handlers::weather::get_weather_data(req, ctx).await })
        .get_async("/api/alerts", |req, ctx| async move { handlers::weather::get_weather_alerts(req, ctx).await })
        .get_async("/api/moon", |req, ctx| async move { handlers::moon::get_moon_phase(req, ctx).await })
        .get_async("/api/conditions", |req, ctx| async move { handlers::conditions::get_all_conditions(req, ctx).await })
        .get_async("/api/historical/:period", |req, ctx| async move { handlers::historical::get_period_data(req, ctx).await })
        .get_async("/api/services", |req, ctx| async move { handlers::services::get_services(req, ctx).await })
        .get_async("/api/bikes", |req, ctx| async move { handlers::store::get_bikes(req, ctx).await })
        .get_async("/api/repairs", |req, ctx| async move { handlers::store::get_repairs(req, ctx).await })
        .get_async("/api/settings/public", |req, ctx| async move { handlers::settings::get_public_settings(req, ctx).await })
        
        // ============================================
        // ADMIN DASHBOARD ROUTES
        // ============================================
        .get_async("/admin", |req, ctx| async move { handlers::admin::dashboard(req, ctx).await })
        .get_async("/admin/login", |req, ctx| async move { handlers::admin::login_page(req, ctx).await })
        .get_async("/admin/settings", |req, ctx| async move { handlers::admin::settings_page(req, ctx).await })
        .get_async("/admin/services", |req, ctx| async move { handlers::admin::services_page(req, ctx).await })
        .get_async("/admin/analytics", |req, ctx| async move { handlers::admin::analytics_page(req, ctx).await })
        .get_async("/admin/content", |req, ctx| async move { handlers::admin::content_page(req, ctx).await })
        .get_async("/admin/css/:file", |req, ctx| async move { handlers::static_files::serve_admin_css(req, ctx).await })
        .get_async("/admin/js/:file", |req, ctx| async move { handlers::static_files::serve_admin_js(req, ctx).await })
        
        // ============================================
        // ADMIN API ENDPOINTS
        // ============================================
        .post_async("/api/admin/login", |req, ctx| async move { handlers::admin::verify_login(req, ctx).await })
        .post_async("/api/admin/verify-totp", |req, ctx| async move { handlers::admin::verify_totp(req, ctx).await })
        .get_async("/api/admin/settings", |req, ctx| async move { handlers::admin::get_settings(req, ctx).await })
        .post_async("/api/admin/settings", |req, ctx| async move { handlers::admin::update_settings(req, ctx).await })
        .post_async("/api/admin/services", |req, ctx| async move { handlers::admin::update_services(req, ctx).await })
        .post_async("/api/admin/thresholds", |req, ctx| async move { handlers::admin::update_thresholds(req, ctx).await })
        .post_async("/api/admin/content", |req, ctx| async move { handlers::admin::update_content(req, ctx).await })
        .get_async("/api/admin/analytics", |req, ctx| async move { handlers::admin::get_analytics(req, ctx).await })
        
        // ============================================
        // CONTACT FORM
        // ============================================
        .post_async("/api/contact", |req, ctx| async move { handlers::contact::submit_form(req, ctx).await })
        
        // ============================================
        // CORS PREFLIGHT
        // ============================================
        .options("/api/*path", |_, _| {
            let mut headers = Headers::new();
            headers.set("Access-Control-Allow-Origin", "*")?;
            headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
            headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization, X-TOTP-Code")?;
            headers.set("Access-Control-Max-Age", "86400")?;
            Ok(Response::empty()?.with_headers(headers).with_status(204))
        })
        
        .run(req, env)
        .await
}
