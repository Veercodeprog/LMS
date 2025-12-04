use crate::api::strapi_articles_data::fetch_articles_by_category;
use crate::app::components::Header;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[component]
pub fn CategoriesPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let articles = LocalResource::new(move || {
        let slug_value = slug();
        async move { fetch_articles_by_category(&slug_value, 1, 10).await }
    });

    Effect::new(move |_| {
        console_log(&format!("Current slug: {}", slug()));
    });

    view! {
        <Header />
        <main>
            <section
                id="content"
                class="container py-2 px-4 my-2 mx-auto font-sans prose lg:prose-2xl"
            >
                <header class="mx-auto mb-8 w-full">
                    <p class="flex items-center">
                        <a href="/categories" class="text-xl text-orange-800">
                            <i class="fa-solid fa-chevron-left text-md"></i>
                            " "
                            <span class="underline">"Back to Categories"</span>
                        </a>
                    </p>
                    <h1 style="margin-bottom: 0.5em;">
                        {move || {
                            slug()
                                .replace("-", " ")
                                .split_whitespace()
                                .map(|word| {
                                    let mut chars = word.chars();
                                    match chars.next() {
                                        None => String::new(),
                                        Some(first) => {
                                            first.to_uppercase().collect::<String>() + chars.as_str()
                                        }
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(" ")
                        }}
                    </h1>
                    <p class="text-lg text-gray-600">
                        "Explore articles and tutorials about " {move || slug().replace("-", " ")}
                    </p>
                </header>

                <Suspense fallback=move || {
                    view! { <p class="text-center">"Loading articles..."</p> }
                }>
                    {move || match articles.get() {
                        Some(Ok(articles_data)) => {
                            if articles_data.data.is_empty() {
                                view! {
                                    <div class="text-center text-gray-600">
                                        <p>"No articles found in this category."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    // ✅ Changed: Single column layout with max-width
                                    <div class="flex flex-col gap-6 mx-auto mb-8 max-w-3xl">
                                        {articles_data
                                            .data
                                            .into_iter()
                                            .map(|article| {
                                                let title = article
                                                    .title
                                                    .clone()
                                                    .unwrap_or_else(|| "Untitled".to_string());
                                                let article_slug = article.slug.clone().unwrap_or_default();
                                                let href = format!("/article/{}", article_slug);
                                                let description = article
                                                    .description
                                                    .clone()
                                                    .unwrap_or_else(|| "No description available".to_string());
                                                let published_at = article
                                                    .publishedAt
                                                    .clone()
                                                    .unwrap_or_else(|| "Unknown date".to_string());
                                                let author_name = article
                                                    .author
                                                    .as_ref()
                                                    .and_then(|a| a.name.clone())
                                                    .unwrap_or_else(|| "Unknown Author".to_string());
                                                let category_name = slug()
                                                    .replace("-", " ")
                                                    .split_whitespace()
                                                    .map(|word| {
                                                        let mut chars = word.chars();
                                                        match chars.next() {
                                                            None => String::new(),
                                                            Some(first) => {
                                                                first.to_uppercase().collect::<String>() + chars.as_str()
                                                            }
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                                    .join(" ");
                                                let image_url = article
                                                    .cover
                                                    .as_ref()
                                                    .and_then(|covers| covers.first())
                                                    .and_then(|cover| cover.url.clone())
                                                    .unwrap_or_else(|| {
                                                        "https://picsum.photos/400/300".to_string()
                                                    });
                                                let full_image_url = if image_url.starts_with("http") {
                                                    image_url.clone()
                                                } else {
                                                    format!("http://localhost:1337{}", image_url)
                                                };

                                                view! {
                                                    // ✅ Changed: Horizontal card layout
                                                    <article class="flex overflow-hidden bg-white rounded-lg shadow-lg transition-transform hover:scale-[1.02]">
                                                        <a href=href.clone() class="flex-shrink-0">
                                                            <img
                                                                class="object-cover w-48 h-full"
                                                                alt=title.clone()
                                                                src=full_image_url
                                                            />
                                                        </a>
                                                        <div class="flex flex-col flex-1 p-6">
                                                            <div class="flex gap-3 items-center mb-3">
                                                                <span class="py-1 px-3 text-xs font-semibold text-white bg-gradient-to-r from-orange-600 to-orange-800 rounded-full">
                                                                    {category_name}
                                                                </span>
                                                                <time class="text-sm text-gray-500">
                                                                    {published_at.split('T').next().unwrap_or(&published_at)}
                                                                </time>
                                                            </div>
                                                            <a href=href.clone() class="no-underline group">
                                                                <h2 class="mb-3 text-2xl font-bold text-gray-900 transition-colors group-hover:text-orange-800">
                                                                    {title}
                                                                </h2>
                                                            </a>
                                                            <p class="flex-1 mb-4 text-base leading-relaxed text-gray-600 line-clamp-2">
                                                                {description}
                                                            </p>
                                                            <div class="flex items-center pt-4 mt-auto border-t border-gray-100">
                                                                <img
                                                                    alt="Author"
                                                                    height="40"
                                                                    width="40"
                                                                    class="mr-3 rounded-full"
                                                                    src="https://api.dicebear.com/9.x/adventurer/svg?seed=Author"
                                                                />
                                                                <div class="text-sm">
                                                                    <p class="font-semibold text-gray-900">
                                                                        "by "{author_name}
                                                                    </p>
                                                                    <p class="text-gray-500">"5 min read"</p>
                                                                </div>
                                                                <a
                                                                    href=href
                                                                    class="py-2 px-4 ml-auto text-sm font-medium text-orange-800 no-underline bg-orange-100 rounded-lg transition-colors hover:text-white hover:bg-orange-800"
                                                                >
                                                                    "Read More →"
                                                                </a>
                                                            </div>
                                                        </div>
                                                    </article>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                                    .into_any()
                            }
                        }
                        Some(Err(e)) => {
                            view! {
                                <div class="text-center text-red-600">
                                    <p>{format!("Error loading articles: {}", e)}</p>
                                </div>
                            }
                                .into_any()
                        }
                        None => view! { <p class="text-center">"Loading..."</p> }.into_any(),
                    }}
                </Suspense>
            </section>
        </main>
    }
}
