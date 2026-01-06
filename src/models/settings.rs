//! Site settings models for admin dashboard

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteSettings {
    pub general: GeneralSettings,
    pub appearance: AppearanceSettings,
    pub business: BusinessSettings,
    pub social: SocialSettings,
    pub seo: SeoSettings,
    pub notifications: NotificationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub site_name: String,
    pub tagline: String,
    pub site_url: String,
    pub timezone: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub background_color: String,
    pub text_color: String,
    pub font_family: String,
    pub logo_url: String,
    pub favicon_url: String,
    pub hero_image_url: String,
    pub dark_mode_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSettings {
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub phone: String,
    pub email: String,
    pub hours: BusinessHours,
    pub appointment_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHours {
    pub monday: DayHours,
    pub tuesday: DayHours,
    pub wednesday: DayHours,
    pub thursday: DayHours,
    pub friday: DayHours,
    pub saturday: DayHours,
    pub sunday: DayHours,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayHours {
    pub open: String,
    pub close: String,
    pub closed: bool,
    pub by_appointment: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSettings {
    pub facebook_url: String,
    pub instagram_url: String,
    pub twitter_url: String,
    pub youtube_url: String,
    pub tiktok_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoSettings {
    pub meta_title: String,
    pub meta_description: String,
    pub meta_keywords: Vec<String>,
    pub og_image_url: String,
    pub google_analytics_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email_notifications: bool,
    pub contact_form_email: String,
    pub low_inventory_alerts: bool,
    pub weather_alerts: bool,
}

impl Default for SiteSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                site_name: "Riverview Adventure Company".to_string(),
                tagline: "Start your adventure off right!".to_string(),
                site_url: "https://riverviewadventurecompany.com".to_string(),
                timezone: "America/Chicago".to_string(),
                language: "en".to_string(),
            },
            appearance: AppearanceSettings {
                primary_color: "#0ea5e9".to_string(),
                secondary_color: "#22c55e".to_string(),
                accent_color: "#f59e0b".to_string(),
                background_color: "#ffffff".to_string(),
                text_color: "#1f2937".to_string(),
                font_family: "Inter, sans-serif".to_string(),
                logo_url: "/images/logo.png".to_string(),
                favicon_url: "/favicon.ico".to_string(),
                hero_image_url: "/images/hero.jpg".to_string(),
                dark_mode_enabled: false,
            },
            business: BusinessSettings {
                address: "740 Water St.".to_string(),
                city: "Sauk City".to_string(),
                state: "WI".to_string(),
                zip: "53583".to_string(),
                phone: "+1 608 515 3456".to_string(),
                email: "riverviewadventureco@gmail.com".to_string(),
                hours: BusinessHours {
                    monday: DayHours { open: "09:00".to_string(), close: "17:00".to_string(), closed: false, by_appointment: true },
                    tuesday: DayHours { open: "09:00".to_string(), close: "17:00".to_string(), closed: false, by_appointment: true },
                    wednesday: DayHours { open: "09:00".to_string(), close: "17:00".to_string(), closed: false, by_appointment: true },
                    thursday: DayHours { open: "09:00".to_string(), close: "17:00".to_string(), closed: false, by_appointment: true },
                    friday: DayHours { open: "09:00".to_string(), close: "17:00".to_string(), closed: false, by_appointment: true },
                    saturday: DayHours { open: "09:00".to_string(), close: "18:00".to_string(), closed: false, by_appointment: false },
                    sunday: DayHours { open: "10:00".to_string(), close: "16:00".to_string(), closed: false, by_appointment: false },
                },
                appointment_only: true,
            },
            social: SocialSettings {
                facebook_url: "https://www.facebook.com/riverviewadventureco/".to_string(),
                instagram_url: "https://www.instagram.com/riverviewadventureco/".to_string(),
                twitter_url: String::new(),
                youtube_url: String::new(),
                tiktok_url: String::new(),
            },
            seo: SeoSettings {
                meta_title: "Wisconsin River Tubing & E-Bike Rentals | Riverview Adventure Company".to_string(),
                meta_description: "Experience Wisconsin River tubing, kayak rentals, and Velotric e-bikes in Sauk City, WI. Located 20 minutes from Madison. Book your adventure today!".to_string(),
                meta_keywords: vec![
                    "Wisconsin River tubing".to_string(),
                    "Sauk City".to_string(),
                    "e-bike rentals".to_string(),
                    "Velotric dealer".to_string(),
                    "kayak rentals".to_string(),
                    "Great Sauk Trail".to_string(),
                ],
                og_image_url: "/images/og-image.jpg".to_string(),
                google_analytics_id: String::new(),
            },
            notifications: NotificationSettings {
                email_notifications: true,
                contact_form_email: "riverviewadventureco@gmail.com".to_string(),
                low_inventory_alerts: true,
                weather_alerts: true,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicSettings {
    pub site_name: String,
    pub tagline: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub logo_url: String,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub social: SocialSettings,
}

impl From<&SiteSettings> for PublicSettings {
    fn from(settings: &SiteSettings) -> Self {
        Self {
            site_name: settings.general.site_name.clone(),
            tagline: settings.general.tagline.clone(),
            primary_color: settings.appearance.primary_color.clone(),
            secondary_color: settings.appearance.secondary_color.clone(),
            accent_color: settings.appearance.accent_color.clone(),
            logo_url: settings.appearance.logo_url.clone(),
            phone: settings.business.phone.clone(),
            email: settings.business.email.clone(),
            address: format!("{}, {}, {} {}", 
                settings.business.address, 
                settings.business.city, 
                settings.business.state, 
                settings.business.zip
            ),
            social: settings.social.clone(),
        }
    }
}
