//! Weather API clients (Tomorrow.io and NWS)

use worker::*;
use crate::models::{TomorrowIoResponse, NwsAlertResponse};

pub async fn fetch_tomorrow_io(
    api_key: &str,
    lat: &str,
    lon: &str
) -> Result<TomorrowIoResponse> {
    let url = format!(
        "https://api.tomorrow.io/v4/timelines?location={},{}&fields=temperature,temperatureApparent,humidity,windSpeed,windDirection,windGust,precipitationIntensity,visibility,uvIndex,weatherCode&timesteps=current&units=imperial&apikey={}",
        lat, lon, api_key
    );
    
    let parsed_url = Url::parse(&url).map_err(|_| Error::from("Invalid URL"))?;
    let mut response = Fetch::Url(parsed_url).send().await?;
    
    if response.status_code() != 200 {
        return Err(Error::from("Tomorrow.io API error"));
    }
    
    response.json().await
}

pub async fn fetch_nws_alerts(zone: &str) -> Result<NwsAlertResponse> {
    let url = format!("https://api.weather.gov/alerts/active/zone/{}", zone);
    
    let mut headers = Headers::new();
    headers.set("User-Agent", "RiverviewAdventure/5.0 (riverviewadventureco@gmail.com)")?;
    headers.set("Accept", "application/geo+json")?;
    
    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);
    
    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    
    if response.status_code() != 200 {
        return Ok(NwsAlertResponse { features: vec![] });
    }
    
    response.json().await
}

pub async fn fetch_nws_forecast(lat: &str, lon: &str) -> Result<serde_json::Value> {
    // First get the forecast office and grid coordinates
    let points_url = format!("https://api.weather.gov/points/{},{}", lat, lon);
    
    let mut headers = Headers::new();
    headers.set("User-Agent", "RiverviewAdventure/5.0 (riverviewadventureco@gmail.com)")?;
    headers.set("Accept", "application/geo+json")?;
    
    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers.clone());
    
    let request = Request::new_with_init(&points_url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    
    if response.status_code() != 200 {
        return Err(Error::from("NWS API error"));
    }
    
    let points_data: serde_json::Value = response.json().await?;
    let forecast_url = points_data["properties"]["forecastHourly"]
        .as_str()
        .ok_or_else(|| Error::from("No forecast URL"))?;
    
    // Get hourly forecast
    let mut init2 = RequestInit::new();
    init2.with_method(Method::Get);
    init2.with_headers(headers);
    
    let request2 = Request::new_with_init(forecast_url, &init2)?;
    let mut forecast_response = Fetch::Request(request2).send().await?;
    
    forecast_response.json().await
}
