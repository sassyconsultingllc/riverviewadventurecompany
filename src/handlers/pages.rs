//! Page handlers - serve HTML pages

use worker::*;



fn html_response(content: &str) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Content-Type", "text/html; charset=utf-8")?;
    headers.set("Cache-Control", "public, max-age=300")?;
    return Ok(Response::ok(content)?.with_headers(headers));

pub async fn home(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/index.html"))
}

pub async fn about(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/about.html"))
}

pub async fn services(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/services.html"))
}

pub async fn tubing(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/tubing.html"))
}

pub async fn bikes(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/bikes.html"))
}

pub async fn ebikes(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/ebikes.html"))
}

pub async fn contact(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/contact.html"))
}

pub async fn waiver(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/waiver.html"))
}

pub async fn how_it_works(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/howitworks.html"))
}

pub async fn gallery(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/gallery.html"))
}

pub async fn conditions(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/conditions.html"))
}

pub async fn book(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/book.html"))
}

pub async fn privacy(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/privacy.html"))
}

pub async fn terms(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/terms.html"))
}

pub async fn services(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/services.html"))
}

pub async fn conditions(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
pub async fn ebikes(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {

pub async fn tubing(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/tubing.html"))
}

pub async fn bikes(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/bikes.html"))
}

pub async fn ebikes(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/ebikes.html"))
}

pub async fn contact(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/contact.html"))
}

pub async fn waiver(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/waiver.html"))
}

pub async fn how_it_works(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/howitworks.html"))
}

pub async fn gallery(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/gallery.html"))
}

pub async fn conditions(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/conditions.html"))
}

pub async fn book(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/book.html"))
}

pub async fn privacy(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/privacy.html"))
}

pub async fn terms(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    html_response(include_str!("../../static/terms.html"))
}

