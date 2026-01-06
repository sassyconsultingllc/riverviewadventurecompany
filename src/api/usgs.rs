//! USGS Water Services API client

use worker::*;
use crate::models::UsgsResponse;

const USGS_API_BASE: &str = "https://waterservices.usgs.gov/nwis/iv";

pub async fn fetch_current_flow(station_id: &str) -> Result<UsgsResponse> {
    let url = format!(
        "{}?format=json&sites={}&parameterCd=00060,00010&siteStatus=all",
        USGS_API_BASE, station_id
    );
    
    let mut headers = Headers::new();
    headers.set("User-Agent", "RiverviewAdventure/5.0")?;
    
    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);
    
    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    
    if response.status_code() != 200 {
        return Err(Error::from("Failed to fetch USGS data"));
    }
    
    response.json().await
}

pub async fn fetch_historical_flow(
    station_id: &str,
    start_date: &str,
    end_date: &str
) -> Result<UsgsResponse> {
    let url = format!(
        "{}?format=json&sites={}&parameterCd=00060,00010&startDT={}&endDT={}",
        USGS_API_BASE, station_id, start_date, end_date
    );
    
    let mut headers = Headers::new();
    headers.set("User-Agent", "RiverviewAdventure/5.0")?;
    
    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);
    
    let request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    
    if response.status_code() != 200 {
        return Err(Error::from("Failed to fetch historical USGS data"));
    }
    
    response.json().await
}
