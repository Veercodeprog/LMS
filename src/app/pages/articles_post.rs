use crate::api::strapi_articles_data::{fetch_article_by_slug, Block};
use crate::app::components::Header;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::*;
use leptos_router::hooks::use_params_map;
use pulldown_cmark::{html, Options, Parser};

#[component]
pub fn ArticlesPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let article = LocalResource::new(move || {
        let slug_value = slug();
        async move { fetch_article_by_slug(&slug_value).await }
    });

    Effect::new(move |_| {
        console_log(&format!("Current article slug: {}", slug()));
    });

    view! {
        <Header />
        <main>
            <Suspense fallback=move || {
                view! { <p class="py-8 text-center">"Loading article..."</p> }
            }>
                {move || match article.get() {
                    Some(Ok(article_data)) => {
                        let title = article_data
                            .title
                            .clone()
                            .unwrap_or_else(|| "Untitled".to_string());
                        let description = article_data
                            .description
                            .clone()
                            .unwrap_or_else(|| "No description".to_string());
                        let published_at = article_data
                            .publishedAt
                            .clone()
                            .unwrap_or_else(|| article_data.createdAt.clone());
                        let image_url = article_data
                            .cover
                            .as_ref()
                            .and_then(|covers| covers.first())
                            .and_then(|cover| cover.url.clone())
                            .unwrap_or_else(|| {
                                "https://images.unsplash.com/photo-1571030701211-aa96da577e91"
                                    .to_string()
                            });
                        let full_image_url = if image_url.starts_with("http") {
                            image_url.clone()
                        } else {
                            format!("http://localhost:1337{}", image_url)
                        };
                        let cover_alt = article_data
                            .cover
                            .as_ref()
                            .and_then(|covers| covers.first())
                            .and_then(|cover| cover.alternative_text.clone())
                            .unwrap_or_else(|| title.clone());
                        let formatted_date = published_at
                            .split('T')
                            .next()
                            .unwrap_or(&published_at);
                        let author_name = article_data
                            .author
                            .as_ref()
                            .and_then(|a| a.name.clone())
                            .unwrap_or_else(|| "Unknown Author".to_string());
                        let category_name = article_data
                            .category
                            .as_ref()
                            .and_then(|c| c.name.clone())
                            .unwrap_or_else(|| "Uncategorized".to_string());
                        let blocks = article_data.blocks.clone().unwrap_or_default();

                        // ✅ FIX: Add .first() to get first element from Vec<Media>

                        // ✅ FIX: Add .first() to get first element from Vec<Media>

                        view! {
                            <section
                                id="content"
                                class="container py-2 px-4 my-2 mx-auto font-sans prose lg:prose-2xl"
                            >
                                <article>
                                    <header class="mx-auto w-full">
                                        <p class="flex items-center">
                                            <a href="/articles" class="text-xl text-orange-800">
                                                <i class="fa-solid fa-chevron-left text-md"></i>
                                                " "
                                                <span class="underline">"Back to Blog"</span>
                                            </a>
                                        </p>
                                        <time datetime=published_at.clone()>{formatted_date}</time>
                                        <h1 style="margin-bottom: 0;">{title.clone()}</h1>
                                    </header>

                                    <footer class="mx-auto w-full">
                                        <div
                                            style="height: 1.5em; margin: 2em 0 2.5em 0"
                                            class="flex items-center"
                                        >
                                            <img
                                                alt=author_name.clone()
                                                height="60"
                                                width="60"
                                                class="mr-3 rounded-full border-2 border-orange-500"
                                                src=format!(
                                                    "https://api.dicebear.com/9.x/adventurer/svg?seed={}",
                                                    author_name.clone(),
                                                )
                                            />
                                            <div class="flex items-center">
                                                <address class="not-italic author">
                                                    "By "
                                                    <a
                                                        rel="author"
                                                        class="font-semibold url fn n"
                                                        href="/author/author"
                                                    >
                                                        {author_name}
                                                    </a>
                                                </address>
                                            </div>
                                            <div class="flex flex-1 gap-3 justify-end items-center">
                                                <span class="py-1 px-3 text-sm text-white bg-orange-800 rounded-full">
                                                    {category_name}
                                                </span>
                                                <a
                                                    aria-label="Share this article"
                                                    href="#"
                                                    class="text-gray-600 hover:text-orange-800"
                                                >
                                                    <i class="text-xl fa-solid fa-share-from-square"></i>
                                                </a>
                                            </div>
                                        </div>
                                    </footer>

                                    <figure>
                                        <img
                                            class="rounded shadow-lg"
                                            alt=cover_alt
                                            src=full_image_url
                                        />
                                        <figcaption class="mt-2 text-base italic text-gray-600">
                                            {description.clone()}
                                        </figcaption>
                                    </figure>

                                    <div class="md:px-8">
                                        {blocks
                                            .into_iter()
                                            .map(|block| {
                                                match block {
                                                    Block::RichText { body, .. } => {
                                                        if let Some(body_array) = body {
                                                            let content = body_array
                                                                .iter()
                                                                .filter_map(|paragraph| {
                                                                    paragraph
                                                                        .get("children")
                                                                        .and_then(|children| children.as_array())
                                                                        .map(|child_array| {
                                                                            child_array
                                                                                .iter()
                                                                                .filter_map(|child| {
                                                                                    child
                                                                                        .get("text")
                                                                                        .and_then(|text| text.as_str())
                                                                                        .map(|s| s.to_string())
                                                                                })
                                                                                .collect::<Vec<_>>()
                                                                                .join("")
                                                                        })
                                                                })
                                                                .collect::<Vec<_>>()
                                                                .join("\n\n");
                                                            let parser = Parser::new_ext(&content, Options::all());
                                                            let mut html_output = String::new();
                                                            html::push_html(&mut html_output, parser);
                                                            // ✅ FIX: Handle Vec<serde_json::Value> properly
                                                            // Extract text from nested JSON

                                                            view! { <div class="my-4" inner_html=html_output></div> }
                                                                .into_any()
                                                        } else {
                                                            view! { <></> }.into_any()
                                                        }
                                                    }
                                                    Block::Quote { title, body, .. } => {
                                                        view! {
                                                            <blockquote class="pl-4 my-6 italic border-l-4 border-orange-700">
                                                                {title
                                                                    .map(|t| {
                                                                        view! { <h3 class="not-italic font-bold">{t}</h3> }
                                                                            .into_any()
                                                                    })} <p>{body.unwrap_or_default()}</p>
                                                            </blockquote>
                                                        }
                                                            .into_any()
                                                    }
                                                    Block::Media { file, .. } => {
                                                        if let Some(media_array) = file {
                                                            if let Some(media) = media_array.first() {
                                                                let url = media.url.clone().unwrap_or_default();
                                                                let full_url = if url.starts_with("http") {
                                                                    url
                                                                } else {
                                                                    format!(
                                                                        "https://bold-reward-a5d659b526.strapiapp.com{}",
                                                                        url,
                                                                    )
                                                                };
                                                                // ✅ FIX: Handle Vec<Media> with .first()
                                                                view! {
                                                                    <img
                                                                        class="my-6 w-full rounded shadow-lg"
                                                                        alt=media.alternative_text.clone().unwrap_or_default()
                                                                        src=full_url
                                                                    />
                                                                }
                                                                    .into_any()
                                                            } else {
                                                                view! { <></> }.into_any()
                                                            }
                                                        } else {
                                                            view! { <></> }.into_any()
                                                        }
                                                    }
                                                    Block::Slider { files, .. } => {
                                                        if let Some(images) = files {
                                                            view! {
                                                                <div class="flex overflow-x-auto gap-4 my-6 snap-x">
                                                                    {images
                                                                        .into_iter()
                                                                        .map(|img| {
                                                                            let url = img.url.unwrap_or_default();
                                                                            let full_url = if url.starts_with("http") {
                                                                                url
                                                                            } else {
                                                                                format!(
                                                                                    "https://bold-reward-a5d659b526.strapiapp.com{}",
                                                                                    url,
                                                                                )
                                                                            };
                                                                            view! {
                                                                                <img
                                                                                    class="object-cover h-64 rounded shadow-lg snap-start"
                                                                                    alt=img.alternative_text.unwrap_or_default()
                                                                                    src=full_url
                                                                                />
                                                                            }
                                                                        })
                                                                        .collect_view()}
                                                                </div>
                                                            }
                                                                .into_any()
                                                        } else {
                                                            view! { <></> }.into_any()
                                                        }
                                                    }
                                                }
                                            })
                                            .collect_view()} <section class="mb-6">
                                            <div class="pt-6 pb-4 mb-6 border-t border-gray-200">
                                                <div class="flex gap-4 justify-center items-center text-gray-600">
                                                    <span class="font-semibold">"Share Post"</span>
                                                    <a href="#" class="transition-colors hover:text-blue-600">
                                                        <i
                                                            style="font-size: 1.8em;"
                                                            class="fa-brands fa-facebook"
                                                        ></i>
                                                    </a>
                                                    <a href="#" class="transition-colors hover:text-pink-600">
                                                        <i
                                                            style="font-size: 1.8em;"
                                                            class="fa-brands fa-instagram"
                                                        ></i>
                                                    </a>
                                                    <a href="#" class="transition-colors hover:text-blue-400">
                                                        <i
                                                            style="font-size: 1.8em;"
                                                            class="fa-brands fa-twitter"
                                                        ></i>
                                                    </a>
                                                </div>
                                            </div>
                                        </section>
                                    </div>
                                </article>
                            </section>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <section class="container py-8 mx-auto text-center">
                                <div class="p-6 mx-auto max-w-md bg-red-50 rounded-lg">
                                    <h2 class="mb-3 text-2xl font-bold text-red-700">
                                        "Error Loading Article"
                                    </h2>
                                    <p class="mb-4 text-red-600">{format!("Error: {}", e)}</p>
                                    <a
                                        href="/articles"
                                        class="inline-block py-2 px-4 text-white bg-orange-800 rounded hover:bg-orange-900"
                                    >
                                        "← Back to Articles"
                                    </a>
                                </div>
                            </section>
                        }
                            .into_any()
                    }
                    None => {
                        view! {
                            <section class="container py-8 mx-auto text-center">
                                <p class="text-lg text-gray-600">"Loading..."</p>
                            </section>
                        }
                            .into_any()
                    }
                }}
            </Suspense>
        </main>
    }
}
