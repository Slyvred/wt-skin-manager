use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

impl Skin {
    pub fn get_thumbnail(&self) -> String {
        match self.images.get(0) {
            Some(img) => img.src.clone(),
            None => String::from("https://media1.tenor.com/m/tlu3haOgKwsAAAAC/horse-wine.gif"),
        }
    }
}

impl FromStr for Skin {
    type Err = String;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(serde_json::from_str(s).unwrap_or_default())
    }
}

pub async fn fetch_skin(url: &str) -> std::prelude::v1::Result<Skin, String> {
    if url.is_empty() {
        return Err("Empty Url".to_string());
    }

    let client = reqwest::Client::new();

    // https://live.warthunder.com/post/1175341/en/ ->
    let url_components: Vec<&str> = url.split("/").collect();
    let params = [
        ("lang_group", url_components[4]),
        ("language", url_components[5]),
    ];

    let res = client
        .post("https://live.warthunder.com/api/posts/get/")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0",
        )
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Erreur réseau : {e}"))?;

    let body_text = res.text().await.map_err(|e| e.to_string())?;

    Skin::from_str(&body_text)
}

pub async fn fetch_page(
    country: &str,
    vehicle_type: &str,
) -> std::prelude::v1::Result<Page, String> {
    if country.is_empty() || vehicle_type.is_empty() {
        return Err("All parameters are required".to_string());
    }

    let client = reqwest::Client::new();

    // curl 'https://live.warthunder.com/api/feed/get_regular/' \
    //   --compressed \
    //   -X POST \
    //   -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0' \
    //   -H 'Accept: application/json, text/javascript, */*; q=0.01' \
    //   -H 'Accept-Language: fr,fr-FR;q=0.9,en-US;q=0.8,en;q=0.7' \
    //   -H 'Accept-Encoding: gzip, deflate, br, zstd' \
    //   -H 'Referer: https://live.warthunder.com/feed/camouflages/?vehicleCountry=france&vehicleType=aircraft' \
    //   -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' \
    //   -H 'Origin: https://live.warthunder.com' \
    //   -H 'Connection: keep-alive' \
    //   -H 'Cookie: conntrack=jlsI/mjyfZ9f1lnCAwvSAg==; lang=en' \
    //   -H 'Sec-Fetch-Dest: empty' \
    //   -H 'Sec-Fetch-Mode: cors' \
    //   -H 'Sec-Fetch-Site: same-origin' \
    //   -H 'TE: trailers' \
    //   --data-raw 'content=camouflage&sort=rating&user=&period=7&searchString=&page=0&featured=0&subtype=all&vehicleCountry=france&vehicleType=aircraft'

    let params = [
        ("content", "camouflage"),
        ("sort", "rating"),
        ("user", ""),
        ("period", ""),
        ("searchString", ""),
        ("featured", "0"),
        ("subtype", "all"),
        ("vehicleCountry", country),
        ("vehicleType", vehicle_type),
    ];

    let res = client
        .post("https://live.warthunder.com/api/feed/get_regular/")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0",
        )
        // .header("Origin", "https://live.warthunder.com")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Network error : {e}"))?;

    let body_text = res.text().await.map_err(|e| e.to_string())?;

    let page: Page = serde_json::from_str(&body_text).unwrap_or_default();

    println!("{:?}", page);

    Ok(page)
}
