use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// --- SKIN ---
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
// ------------

// --- PAGE ---
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
// ------------

// --- FILTERS ---
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
// ---------------

pub async fn fetch_filters(client: Client) -> Result<Filters, String> {
    let params = [
        ("content", "camouflage"),
        ("sort", "rating"),
        ("user", ""),
        ("period", ""),
        ("searchString", ""),
        ("page", "0"),
        ("featured", "0"),
        ("subtype", "all"),
    ];

    let res = client
        .post("https://live.warthunder.com/api/feed/get_head/")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0",
        )
        .header("Origin", "https://live.warthunder.com")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Network error : {e}"))?;

    let body_text = res.text().await.map_err(|e| e.to_string())?;
    let re = Regex::new(r"(?s)const filters = (\{.*?\});").unwrap();

    let mut results = String::new();
    for (_, [json]) in re.captures_iter(&body_text).map(|c| c.extract()) {
        results = json.to_string();
    }

    let json_filters: Filters = tokio::task::spawn_blocking(move || serde_json::from_str(&results))
        .await
        .map_err(|_| "Thread pool error".to_string())?
        .map_err(|e| format!("Failed to parse JSON: {e}"))?;

    Ok(json_filters)
}

pub async fn fetch_page(
    client: Client,
    vehicle_country: &str,
    vehicle_type: &str,
    vehicle_class: &str,
    vehicle: &str,
    page: i32,
) -> std::prelude::v1::Result<Page, String> {
    let params = [
        ("content", "camouflage"),
        ("sort", "rating"),
        ("user", ""),
        ("period", ""),
        ("searchString", ""),
        ("featured", "0"),
        ("subtype", "all"),
        ("page", &page.to_string()),
        ("vehicleCountry", vehicle_country),
        ("vehicleType", vehicle_type),
        ("vehicleClass", vehicle_class),
        ("vehicle", vehicle),
    ];

    // println!("{:?}", params);

    let res = client
        .post("https://live.warthunder.com/api/feed/get_regular/")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0",
        )
        .header("Origin", "https://live.warthunder.com")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Network error : {e}"))?;

    let body_text = res.text().await.map_err(|e| e.to_string())?;

    let page: Page =
        serde_json::from_str(&body_text).map_err(|e| format!("Failed to parse JSON: {e}"))?;

    // println!("{:?}", page);

    Ok(page)
}
