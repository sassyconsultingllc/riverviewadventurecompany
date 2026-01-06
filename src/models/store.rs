//! Store models for e-bikes and repair services

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelotricBike {
    pub id: String,
    pub name: String,
    pub model: String,
    pub price: f64,
    pub msrp: f64,
    pub description: String,
    pub features: Vec<String>,
    pub specs: BikeSpecs,
    pub image_url: String,
    pub in_stock: bool,
    pub category: BikeCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BikeSpecs {
    pub motor: String,
    pub battery: String,
    pub range: String,
    pub top_speed: String,
    pub weight: String,
    pub max_load: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BikeCategory {
    Commuter,
    Cruiser,
    Folding,
    Cargo,
    Mountain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairService {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price_min: f64,
    pub price_max: Option<f64>,
    pub duration: String,
    pub category: RepairCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RepairCategory {
    Basic,
    Drivetrain,
    Brakes,
    Wheels,
    Electrical,
    FullService,
}

pub fn default_velotric_products() -> Vec<VelotricBike> {
    vec![
        VelotricBike {
            id: "discover-1".to_string(),
            name: "Velotric Discover 1".to_string(),
            model: "Discover 1".to_string(),
            price: 1099.0,
            msrp: 1299.0,
            description: "The perfect entry-level e-bike for commuters and casual riders.".to_string(),
            features: vec![
                "500W rear hub motor".to_string(),
                "48V 10.4Ah battery".to_string(),
                "Up to 52 miles range".to_string(),
                "Integrated lights".to_string(),
                "LCD display".to_string(),
            ],
            specs: BikeSpecs {
                motor: "500W rear hub".to_string(),
                battery: "48V 10.4Ah".to_string(),
                range: "Up to 52 miles".to_string(),
                top_speed: "20 mph".to_string(),
                weight: "62 lbs".to_string(),
                max_load: "300 lbs".to_string(),
            },
            image_url: "/images/discover-1.jpg".to_string(),
            in_stock: true,
            category: BikeCategory::Commuter,
        },
        VelotricBike {
            id: "discover-2".to_string(),
            name: "Velotric Discover 2".to_string(),
            model: "Discover 2".to_string(),
            price: 1399.0,
            msrp: 1599.0,
            description: "Enhanced performance with torque sensor and premium components.".to_string(),
            features: vec![
                "500W rear hub motor".to_string(),
                "48V 14.4Ah battery".to_string(),
                "Up to 65 miles range".to_string(),
                "Torque sensor".to_string(),
                "Hydraulic disc brakes".to_string(),
            ],
            specs: BikeSpecs {
                motor: "500W rear hub".to_string(),
                battery: "48V 14.4Ah".to_string(),
                range: "Up to 65 miles".to_string(),
                top_speed: "20 mph".to_string(),
                weight: "65 lbs".to_string(),
                max_load: "300 lbs".to_string(),
            },
            image_url: "/images/discover-2.jpg".to_string(),
            in_stock: true,
            category: BikeCategory::Commuter,
        },
        VelotricBike {
            id: "go-1".to_string(),
            name: "Velotric Go 1".to_string(),
            model: "Go 1".to_string(),
            price: 999.0,
            msrp: 1199.0,
            description: "Compact folding e-bike perfect for mixed commutes.".to_string(),
            features: vec![
                "350W rear hub motor".to_string(),
                "36V 7.8Ah battery".to_string(),
                "Folds in seconds".to_string(),
                "Lightweight design".to_string(),
                "Fits in car trunk".to_string(),
            ],
            specs: BikeSpecs {
                motor: "350W rear hub".to_string(),
                battery: "36V 7.8Ah".to_string(),
                range: "Up to 35 miles".to_string(),
                top_speed: "20 mph".to_string(),
                weight: "45 lbs".to_string(),
                max_load: "265 lbs".to_string(),
            },
            image_url: "/images/go-1.jpg".to_string(),
            in_stock: true,
            category: BikeCategory::Folding,
        },
        VelotricBike {
            id: "nomad-1".to_string(),
            name: "Velotric Nomad 1".to_string(),
            model: "Nomad 1".to_string(),
            price: 1599.0,
            msrp: 1899.0,
            description: "All-terrain fat tire e-bike for adventure seekers.".to_string(),
            features: vec![
                "750W rear hub motor".to_string(),
                "48V 14.4Ah battery".to_string(),
                "4-inch fat tires".to_string(),
                "Full suspension".to_string(),
                "All-terrain capable".to_string(),
            ],
            specs: BikeSpecs {
                motor: "750W rear hub".to_string(),
                battery: "48V 14.4Ah".to_string(),
                range: "Up to 55 miles".to_string(),
                top_speed: "28 mph".to_string(),
                weight: "73 lbs".to_string(),
                max_load: "350 lbs".to_string(),
            },
            image_url: "/images/nomad-1.jpg".to_string(),
            in_stock: true,
            category: BikeCategory::Mountain,
        },
    ]
}

pub fn default_repair_services() -> Vec<RepairService> {
    vec![
        RepairService {
            id: "tune-up-basic".to_string(),
            name: "Basic Tune-Up".to_string(),
            description: "Safety check, brake adjustment, derailleur adjustment, tire inflation".to_string(),
            price_min: 45.0,
            price_max: None,
            duration: "Same day".to_string(),
            category: RepairCategory::Basic,
        },
        RepairService {
            id: "tune-up-full".to_string(),
            name: "Full Tune-Up".to_string(),
            description: "Complete overhaul including cleaning, lubrication, and all adjustments".to_string(),
            price_min: 85.0,
            price_max: Some(120.0),
            duration: "1-2 days".to_string(),
            category: RepairCategory::FullService,
        },
        RepairService {
            id: "flat-repair".to_string(),
            name: "Flat Tire Repair".to_string(),
            description: "Tube replacement or patch, tire inspection".to_string(),
            price_min: 15.0,
            price_max: Some(25.0),
            duration: "While you wait".to_string(),
            category: RepairCategory::Wheels,
        },
        RepairService {
            id: "brake-adjust".to_string(),
            name: "Brake Adjustment".to_string(),
            description: "Adjust brake pads, cables, and alignment".to_string(),
            price_min: 20.0,
            price_max: Some(35.0),
            duration: "Same day".to_string(),
            category: RepairCategory::Brakes,
        },
        RepairService {
            id: "brake-bleed".to_string(),
            name: "Hydraulic Brake Bleed".to_string(),
            description: "Full hydraulic brake system bleed and fluid replacement".to_string(),
            price_min: 40.0,
            price_max: Some(60.0),
            duration: "Same day".to_string(),
            category: RepairCategory::Brakes,
        },
        RepairService {
            id: "derailleur".to_string(),
            name: "Derailleur Adjustment".to_string(),
            description: "Front and/or rear derailleur adjustment and cable tension".to_string(),
            price_min: 25.0,
            price_max: Some(40.0),
            duration: "Same day".to_string(),
            category: RepairCategory::Drivetrain,
        },
        RepairService {
            id: "chain-replace".to_string(),
            name: "Chain Replacement".to_string(),
            description: "New chain installation and sizing".to_string(),
            price_min: 20.0,
            price_max: None,
            duration: "Same day".to_string(),
            category: RepairCategory::Drivetrain,
        },
        RepairService {
            id: "wheel-true".to_string(),
            name: "Wheel Truing".to_string(),
            description: "Straighten and tension wheel spokes".to_string(),
            price_min: 25.0,
            price_max: Some(40.0),
            duration: "1-2 days".to_string(),
            category: RepairCategory::Wheels,
        },
        RepairService {
            id: "ebike-diag".to_string(),
            name: "E-Bike Diagnostics".to_string(),
            description: "Full electrical system diagnostic and troubleshooting".to_string(),
            price_min: 50.0,
            price_max: Some(75.0),
            duration: "1-2 days".to_string(),
            category: RepairCategory::Electrical,
        },
    ]
}
