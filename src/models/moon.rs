//! Moon phase and sun time models

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonData {
    pub phase: String,
    pub phase_emoji: String,
    pub illumination: f64,
    pub age_days: f64,
}

impl MoonData {
    pub fn calculate(year: i32, month: u32, day: u32) -> Self {
        // Calculate Julian date
        let a: i32 = (14 - month as i32) / 12;
        let y: i32 = year + 4800 - a;
        let m: i32 = month as i32 + 12 * a - 3;
        
        let jd: f64 = day as f64 + ((153 * m + 2) / 5) as f64 + (365 * y) as f64 
            + (y / 4) as f64 - (y / 100) as f64 + (y / 400) as f64 - 32045.0;
        
        // Calculate moon age (days since new moon)
        let days_since_new: f64 = (jd - 2451550.1) % 29.530588853;
        let age: f64 = if days_since_new < 0.0 { days_since_new + 29.530588853 } else { days_since_new };
        
        // Calculate illumination (approximate)
        let illumination: f64 = (1.0 - (2.0 * PI * age / 29.530588853).cos()) / 2.0 * 100.0;
        
        // Determine phase name and emoji
        let (phase, emoji) = if age < 1.84566 {
            ("New Moon", "🌑")
        } else if age < 5.53699 {
            ("Waxing Crescent", "🌒")
        } else if age < 9.22831 {
            ("First Quarter", "🌓")
        } else if age < 12.91963 {
            ("Waxing Gibbous", "🌔")
        } else if age < 16.61096 {
            ("Full Moon", "🌕")
        } else if age < 20.30228 {
            ("Waning Gibbous", "🌖")
        } else if age < 23.99361 {
            ("Last Quarter", "🌗")
        } else if age < 27.68493 {
            ("Waning Crescent", "🌘")
        } else {
            ("New Moon", "🌑")
        };
        
        Self {
            phase: phase.to_string(),
            phase_emoji: emoji.to_string(),
            illumination: (illumination * 10.0_f64).round() / 10.0_f64,
            age_days: (age * 10.0_f64).round() / 10.0_f64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunData {
    pub sunrise: String,
    pub sunset: String,
    pub day_length: String,
    pub solar_noon: String,
}

impl SunData {
    pub fn calculate(year: i32, month: u32, day: u32, lat: f64, lon: f64) -> Self {
        // Day of year
        let n1: f64 = (275 * month as i32 / 9) as f64;
        let n2: f64 = ((month as i32 + 9) / 12) as f64;
        let n3: f64 = 1.0 + ((year as f64 - 4.0 * (year as f64 / 4.0).floor() + 2.0) / 3.0).floor();
        let n: f64 = n1 - (n2 * n3) + day as f64 - 30.0;
        
        // Approximate sun position
        let lng_hour: f64 = lon / 15.0;
        
        // Sunrise
        let t_rise: f64 = n + ((6.0 - lng_hour) / 24.0);
        let m_rise: f64 = (0.9856 * t_rise) - 3.289;
        let l_rise_raw: f64 = m_rise + (1.916 * (m_rise * PI / 180.0).sin()) 
            + (0.020 * (2.0 * m_rise * PI / 180.0).sin()) + 282.634;
        let l_rise: f64 = ((l_rise_raw % 360.0) + 360.0) % 360.0;
        
        let ra_rise_raw: f64 = (l_rise * PI / 180.0).tan().atan() * 180.0 / PI;
        let ra_rise_norm: f64 = ((ra_rise_raw % 360.0) + 360.0) % 360.0;
        let l_quad: f64 = (l_rise / 90.0).floor() * 90.0;
        let ra_quad: f64 = (ra_rise_norm / 90.0).floor() * 90.0;
        let ra_rise: f64 = (ra_rise_norm + (l_quad - ra_quad)) / 15.0;
        
        let sin_dec: f64 = 0.39782 * (l_rise * PI / 180.0).sin();
        let cos_dec: f64 = sin_dec.asin().cos();
        
        let zenith: f64 = 90.833;
        let cos_h: f64 = ((zenith * PI / 180.0).cos() - (sin_dec * (lat * PI / 180.0).sin())) 
            / (cos_dec * (lat * PI / 180.0).cos());
        
        let h_rise_deg: f64 = if cos_h > 1.0 { 0.0 } else if cos_h < -1.0 { 180.0 } 
            else { 360.0 - cos_h.acos() * 180.0 / PI };
        let h_rise: f64 = h_rise_deg / 15.0;
        
        let t_utc_rise: f64 = h_rise + ra_rise - (0.06571 * t_rise) - 6.622;
        let ut_rise: f64 = ((t_utc_rise - lng_hour) % 24.0 + 24.0) % 24.0;
        
        // Sunset
        let t_set: f64 = n + ((18.0 - lng_hour) / 24.0);
        let m_set: f64 = (0.9856 * t_set) - 3.289;
        let l_set_raw: f64 = m_set + (1.916 * (m_set * PI / 180.0).sin()) 
            + (0.020 * (2.0 * m_set * PI / 180.0).sin()) + 282.634;
        let l_set: f64 = ((l_set_raw % 360.0) + 360.0) % 360.0;
        
        let ra_set_raw: f64 = (l_set * PI / 180.0).tan().atan() * 180.0 / PI;
        let ra_set_norm: f64 = ((ra_set_raw % 360.0) + 360.0) % 360.0;
        let l_quad_set: f64 = (l_set / 90.0).floor() * 90.0;
        let ra_quad_set: f64 = (ra_set_norm / 90.0).floor() * 90.0;
        let ra_set: f64 = (ra_set_norm + (l_quad_set - ra_quad_set)) / 15.0;
        
        let sin_dec_set: f64 = 0.39782 * (l_set * PI / 180.0).sin();
        let cos_dec_set: f64 = sin_dec_set.asin().cos();
        
        let cos_h_set: f64 = ((zenith * PI / 180.0).cos() - (sin_dec_set * (lat * PI / 180.0).sin())) 
            / (cos_dec_set * (lat * PI / 180.0).cos());
        
        let h_set_deg: f64 = if cos_h_set > 1.0 { 180.0 } else if cos_h_set < -1.0 { 0.0 } 
            else { cos_h_set.acos() * 180.0 / PI };
        let h_set: f64 = h_set_deg / 15.0;
        
        let t_utc_set: f64 = h_set + ra_set - (0.06571 * t_set) - 6.622;
        let ut_set: f64 = ((t_utc_set - lng_hour) % 24.0 + 24.0) % 24.0;
        
        // Convert to local time (CST = UTC-6)
        let local_rise: f64 = (ut_rise - 6.0 + 24.0) % 24.0;
        let local_set: f64 = (ut_set - 6.0 + 24.0) % 24.0;
        let local_noon: f64 = (local_rise + local_set) / 2.0;
        
        let day_length_hours_raw: f64 = local_set - local_rise;
        let day_length_hours: f64 = if day_length_hours_raw < 0.0 { day_length_hours_raw + 24.0 } else { day_length_hours_raw };
        
        Self {
            sunrise: format!("{:02}:{:02}", local_rise as i32, ((local_rise % 1.0) * 60.0) as i32),
            sunset: format!("{:02}:{:02}", local_set as i32, ((local_set % 1.0) * 60.0) as i32),
            day_length: format!("{}h {}m", day_length_hours as i32, ((day_length_hours % 1.0) * 60.0) as i32),
            solar_noon: format!("{:02}:{:02}", local_noon as i32, ((local_noon % 1.0) * 60.0) as i32),
        }
    }
}
