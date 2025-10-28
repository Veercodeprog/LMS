// src/strapi_categories_data.rs
use crate::api::strapi::{build_strapi_url, fetch_strapi, StrapiList};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: i64,
    pub documentId: String,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub publishedAt: Option<String>,
}

const STRAPI_BASE: &str = "https://bold-reward-a5d659b526.strapiapp.com";
const STRAPI_TOKEN: Option<&str> = Some("YOUR_API_TOKEN_HERE"); // or None for public

/// Fetch a single page of categories
pub async fn fetch_categories_page(
    page: u32,
    page_size: u32,
) -> Result<StrapiList<Category>, String> {
    let url = build_strapi_url(STRAPI_BASE, "/categories", page, page_size, None, None);
    fetch_strapi::<StrapiList<Category>>(&url).await
}

/// Fetch all categories across all pages
pub async fn fetch_all_categories() -> Result<Vec<Category>, String> {
    let first = fetch_categories_page(1, 100).await?;
    let mut all = first.data;
    let page_count = first.meta.pagination.pageCount;

    for page in 2..=page_count {
        let next = fetch_categories_page(page, 100).await?;
        all.extend(next.data);
    }
    Ok(all)
}
