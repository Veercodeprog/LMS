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

    /*   let STRAPI_TOKEN="c4572b98bac66bfea088dbf3666802ba0ce1f2558a49768d9b20f11f3f3b847f39bdfaf2ac0ef250476916722e1050b1d1a2b92049b85a2a00aca5c16a4291aef059e6ed826211e93b4641a8eba43fa96adbee818bcb1e60ed0b3cc50f4f540450a85255fbbcb4a2400b9ed12998547ea1038d6e07b4a7770d87da9c0003d7d9"; */

    let STRAPI_TOKEN = env::var("STRAPI_TOKEN").expect("STRAPI_TOKEN must be set");

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
