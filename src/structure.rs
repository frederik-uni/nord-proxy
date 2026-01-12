use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct City {
    pub name: crate::City,
    pub hub_score: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub code: crate::Country,
    pub city: City,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Locations {
    pub country: Country,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Services {
    pub identifier: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pivot {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Technologies {
    pub identifier: String,
    pub pivot: Pivot,
    pub metadata: Vec<Metadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    pub status: String,
    pub services: Vec<Services>,
    pub hostname: String,
    pub load: u32,
    pub locations: Vec<Locations>,
    pub technologies: Vec<Technologies>,
}
