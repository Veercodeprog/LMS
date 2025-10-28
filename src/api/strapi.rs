// src/strapi.rs
extern crate dotenv;

use dotenv::dotenv;
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
    dotenv().ok();

    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
    // let STRAPI_TOKEN = env::var("STRAP_TOKEN").ok();
    // console_log(&format!("🔑 Token found: {:?}...", STRAPI_TOKEN));
    let STRAPI_TOKEN="826791505ac8852f47ecf59be2f375a3dd86c62f2a6dec6e24d7769f0459e58cdc1daa376bfd9bcb8aa32991a64d58335fd7dcb93060f81cffea3ff256eee706a095058a86f4fa4191470c5aaf6bf9482b8edf6b596ef092e327128871f73aace96a6b3217901db9b797e6cbf6a41ffce2422d6e5ea216d72b51b86c56582286".to_string();
    dotenv().ok();

    if let Ok(token) = env::var("STRAPI_TOKEN") {
        // use the token
        console_log(&format!("🔑 Token found: {:?}...", STRAPI_TOKEN));

        req = req.header("Authorization", format!("Bearer {}", STRAPI_TOKEN).as_str());
    } else {
        // bail out gracefully
        console_log(&format!("🔑 Token not found"));
        println!("not ok");
    }

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

/// Build a paginated Strapi URL with query string
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
