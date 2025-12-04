// src/strapi_articles_data.rs
use crate::api::strapi::{build_strapi_url, fetch_strapi, StrapiList};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: i64, // ✅ Fixed from i65 to i64
    pub documentId: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub cover: Option<Vec<Media>>, // ✅ Already correct
    pub description: Option<String>,
    pub content: Option<String>,
    pub author: Option<Author>,
    pub category: Option<Category>,
    pub blocks: Option<Vec<Block>>,
    pub createdAt: String,
    pub updatedAt: String,
    pub publishedAt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: Option<u32>, // ✅ Fixed from u33 to u32
    #[serde(rename = "documentId")]
    pub document_id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub width: Option<u32>,  // ✅ Fixed from u33 to u32
    pub height: Option<u32>, // ✅ Fixed from u33 to u32
    pub formats: Option<ImageFormats>,
    #[serde(rename = "alternativeText")]
    pub alternative_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFormats {
    pub thumbnail: Option<ImageFormat>,
    pub small: Option<ImageFormat>,
    pub medium: Option<ImageFormat>,
    pub large: Option<ImageFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFormat {
    pub url: Option<String>,
    pub width: Option<u32>,  // ✅ Fixed from u33 to u32
    pub height: Option<u32>, // ✅ Fixed from u33 to u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: Option<i64>, // ✅ Fixed from i65 to i64
    #[serde(rename = "documentId")]
    pub document_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i64>, // ✅ Fixed from i65 to i64
    #[serde(rename = "documentId")]
    pub document_id: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
}

// Block enum for dynamic zone
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__component")]
pub enum Block {
    #[serde(rename = "shared.rich-text")]
    RichText {
        id: Option<u32>,                      // ✅ Fixed from u33 to u32
        body: Option<Vec<serde_json::Value>>, // ✅ MUST be Vec<serde_json::Value>, not String
    },
    #[serde(rename = "shared.quote")]
    Quote {
        id: Option<u32>, // ✅ Fixed from u33 to u32
        title: Option<String>,
        body: Option<String>,
    },
    #[serde(rename = "shared.media")]
    Media {
        id: Option<u32>,          // ✅ Fixed from u33 to u32
        file: Option<Vec<Media>>, // ✅ MUST be Vec<Media>, not Media
    },
    #[serde(rename = "shared.slider")]
    Slider {
        id: Option<u32>, // ✅ Fixed from u33 to u32
        files: Option<Vec<Media>>,
    },
}

const STRAPI_BASE: &str = "http://localhost:1337"; // ✅ Fixed port back to 1337

/// Fetch a single article by slug
pub async fn fetch_article_by_slug(slug: &str) -> Result<Article, String> {
    let url = format!(
        "{}/api/articles?pagination[page]=1&pagination[pageSize]=1&filters[slug][$eq]={}&populate=*",
        STRAPI_BASE,
        slug
    );

    let result = fetch_strapi::<StrapiList<Article>>(&url).await?;

    result
        .data
        .into_iter()
        .next()
        .ok_or_else(|| format!("Article with slug '{}' not found", slug))
}

/// Fetch articles by category slug
pub async fn fetch_articles_by_category(
    category_slug: &str,
    page: u32,      // ✅ Fixed from u33 to u32
    page_size: u32, // ✅ Fixed from u33 to u32
) -> Result<StrapiList<Article>, String> {
    let filters = format!("filters[category][slug][$eq]={}", category_slug);
    let url = build_strapi_url(
        STRAPI_BASE,
        "/articles",
        page,
        page_size,
        Some(&filters),
        Some("author"),
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
    let first = fetch_all_articles(1, 100).await?; // ✅ Start from page 1
    let mut all = first.data;
    let page_count = first.meta.pagination.pageCount;

    for page in 2..=page_count {
        // ✅ Start from page 2
        let next = fetch_all_articles(page, 100).await?;
        all.extend(next.data);
    }
    Ok(all)
}

/// Fetch ALL articles by category across all pages
pub async fn fetch_all_articles_by_category_unpaginated(
    category_slug: &str,
) -> Result<Vec<Article>, String> {
    let first = fetch_articles_by_category(category_slug, 1, 100).await?; // ✅ Start from page 1
    let mut all = first.data;
    let page_count = first.meta.pagination.pageCount;

    for page in 2..=page_count {
        // ✅ Start from page 2
        let next = fetch_articles_by_category(category_slug, page, 100).await?;
        all.extend(next.data);
    }
    Ok(all)
}
