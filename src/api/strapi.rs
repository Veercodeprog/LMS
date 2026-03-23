// src/strapi.rs
// use dotenv::dotenv;
use gloo_net::http::Request;
use leptos::leptos_dom::logging::console_log;
use serde::{Deserialize, Serialize};
use std::env;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub pageSize: u32,
    pub pageCount: u32,
    pub total: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    pub pagination: Pagination,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StrapiList<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}

/// Generic Strapi fetch – returns deserialized JSON of type T
pub async fn fetch_strapi<T>(url: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let mut req = Request::get(url).header("Accept", "application/json");

    let STRAPI_TOKEN="6ae104fc02689d25c684a42ce53dd81e6bcdfa36cc0bdf6e720a745f18f1b03f5909f1b99b4a0cb907e4740fd8a15cf4c6d43e7a52d2483e7adeba0a2360fc0f32d29731aa227353360136a15e33cc6c88d87ee66acdb16b0a3ecb1414803237b24594f1b0c571ae61efe9b56a2bb075389103efe1b28428f1c53f75ce0a1820";

    // let STRAPI_TOKEN = env::var("STRAPI_TOKEN").expect("STRAPI_TOKEN must be set");

    // let STRAPI_TOKEN = env::var("STRAPI_TOKEN");
    console_log(&format!("token: {}", STRAPI_TOKEN));
    req = req.header("Authorization", &format!("Bearer {}", STRAPI_TOKEN));

    // if let Some(t) = STRAPI_TOKEN {
    //     req = req.header("Authorization", &format!("Bearer {}", t));
    // }

    let resp = req
        .send()
        .await
        .map_err(|e| format!("network error: {}", e))?;

    if !resp.ok() {
        return Err(format!("http {} {}", resp.status(), resp.status_text()));
    }

    resp.json::<T>()
        .await
        .map_err(|e| format!("parse error: {}", e))
}

// Build a paginated Strapi URL with query string
pub fn build_strapi_url(
    base: &str,
    endpoint: &str,
    page: u32,
    page_size: u32,
    filters: Option<&str>,
    populate: Option<&str>,
) -> String {
    let mut url = format!(
        "{}/api{}?pagination[page]={}&pagination[pageSize]={}",
        base, endpoint, page, page_size
    );

    if let Some(f) = filters {
        url.push_str("&");
        url.push_str(f);
    }

    if let Some(p) = populate {
        url.push_str("&populate=");
        url.push_str(p);
    }

    url
}
