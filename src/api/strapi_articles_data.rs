// src/strapi_articles_data.rs
use crate::api::strapi::{build_strapi_url, fetch_strapi, StrapiList};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: i64,
    pub documentId: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub publishedAt: Option<String>,
    // Add nested relations (cover, category, authorsBio) as needed
}

const STRAPI_BASE: &str = "https://bold-reward-a5d659b526.strapiapp.com";

/// Fetch articles by category slug with populate
pub async fn fetch_articles_by_category(
    category_slug: &str,
    page: u32,
    page_size: u32,
) -> Result<StrapiList<Article>, String> {
    let filters = format!("filters[category][slug][$eq]={}", category_slug);
    let url = build_strapi_url(
        STRAPI_BASE,
        "/articles",
        page,
        page_size,
        Some(&filters),
        Some("*"),
    );
    fetch_strapi::<StrapiList<Article>>(&url).await
}

/// Fetch a single page of articles
pub async fn fetch_all_articles(page: u32, page_size: u32) -> Result<StrapiList<Article>, String> {
    let url = build_strapi_url(STRAPI_BASE, "/articles", page, page_size, None, Some("*"));
    fetch_strapi::<StrapiList<Article>>(&url).await
}

/// Fetch ALL articles across all pages (unpaginated)
pub async fn fetch_all_articles_unpaginated() -> Result<Vec<Article>, String> {
    let first = fetch_all_articles(1, 100).await?;
    let mut all = first.data;
    let page_count = first.meta.pagination.pageCount;

    for page in 2..=page_count {
        let next = fetch_all_articles(page, 100).await?;
        all.extend(next.data);
    }
    Ok(all)
}

/// Fetch ALL articles by category across all pages
pub async fn fetch_all_articles_by_category_unpaginated(
    category_slug: &str,
) -> Result<Vec<Article>, String> {
    let first = fetch_articles_by_category(category_slug, 1, 100).await?;
    let mut all = first.data;
    let page_count = first.meta.pagination.pageCount;

    for page in 2..=page_count {
        let next = fetch_articles_by_category(category_slug, page, 100).await?;
        all.extend(next.data);
    }
    Ok(all)
}
