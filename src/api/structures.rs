use serde::{Deserialize, Serialize};
use std::str::FromStr;

// --------- SKIN --------- //
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Author {
    pub nickname: String,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Images {
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub link: String,
    pub size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Skin {
    pub author: Author,
    pub lang_group: i32,
    pub id: i32,
    pub likes: i32,
    pub views: i32,
    pub downloads: i32,
    pub comments: i32,
    pub images: Vec<Images>,
    pub file: File,
}

impl Skin {
    pub fn get_thumbnail(&self) -> &str {
        match self.images.get(0) {
            Some(img) => &img.src,
            None => "https://media1.tenor.com/m/tlu3haOgKwsAAAAC/horse-wine.gif", // Fallback
        }
    }
}

impl FromStr for Skin {
    type Err = String;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(serde_json::from_str(s).unwrap_or_default())
    }
}
// --------- END SKIN --------- //

// --------- PAGE --------- //
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Data {
    pub list: Vec<Skin>,
    #[serde(rename = "pageTitle")]
    pub page_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Page {
    pub data: Data,
}
// --------- END PAGE --------- //

// --------- FILTERS --------- //
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct DepType {
    #[serde(rename = "vehicleCountry")]
    pub vehicle_country: Option<Vec<String>>,
    #[serde(rename = "vehicleType")]
    pub vehicle_type: Option<Vec<String>>,
    #[serde(rename = "vehicleClass")]
    pub vehicle_class: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Variant {
    pub value: Option<String>,
    pub name: String,
    pub count: Option<i32>,
    pub dep: Option<DepType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct FilterType {
    pub placeholder: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Filters {
    #[serde(rename = "vehicleCountry")]
    pub vehicle_country: FilterType,

    #[serde(rename = "vehicleType")]
    pub vehicle_type: FilterType,

    #[serde(rename = "vehicleClass")]
    pub vehicle_class: FilterType,

    pub vehicle: FilterType,
}
// --------- END FILTERS --------- //
