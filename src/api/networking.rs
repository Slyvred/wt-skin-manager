use crate::{api::structures::*, FILTERS};
use dioxus::logger::tracing;
use regex::Regex;
use reqwest::Client;

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
        .header("Referer", "https://live.warthunder.com/feed/all/")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Network error : {e}"))?;

    let status_code = res.status();
    tracing::debug!("Filters status: {:?}", status_code);

    let json_filters: Filters;

    if status_code != reqwest::StatusCode::OK {
        tracing::debug!("Failed to fetch filters, reverting to local backup");
        let bytes = dioxus::asset_resolver::read_asset_bytes(&FILTERS)
            .await
            .unwrap();

        json_filters = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    } else {
        let body_text = res.text().await.map_err(|e| e.to_string())?;
        let re = Regex::new(r"(?s)const filters = (\{.*?\});").unwrap();

        let mut results = String::new();
        for (_, [json]) in re.captures_iter(&body_text).map(|c| c.extract()) {
            results = json.to_string();
        }

        json_filters = tokio::task::spawn_blocking(move || serde_json::from_str(&results))
            .await
            .map_err(|_| "Thread pool error".to_string())?
            .map_err(|e| format!("Failed to parse JSON: {e}"))?;
    }

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

    tracing::debug!("POST Params: {:?}", params);

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

    tracing::debug!("Page: {:?}", &page);

    Ok(page)
}

pub async fn fetch_skin(client: Client, lang_group: i32) -> Result<Skin, String> {
    let lang_group_str = lang_group.to_string();
    let params = [("lang_group", lang_group_str.as_str()), ("language", "en")];

    tracing::debug!("POST Params: {:?}", params);

    let res = client
        .post("https://live.warthunder.com/api/posts/get/")
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

    let skin: Skin =
        serde_json::from_str(&body_text).map_err(|e| format!("Failed to parse JSON: {e}"))?;

    tracing::debug!("Skin: {:?}", &skin);

    Ok(skin)
}
