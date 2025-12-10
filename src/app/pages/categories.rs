use crate::api::strapi_categories_data::fetch_all_categories;
use crate::app::components::Header;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::*;

#[component]
pub fn Categories() -> impl IntoView {
    // Fetch all categories
    let categories = LocalResource::new(|| async move { fetch_all_categories().await });

    Effect::new(move |_| {
        console_log("Categories page loaded");
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
                        <a href="/" class="text-xl text-orange-800">
                            <i class="fa-solid fa-chevron-left text-md"></i>
                            " "
                            <span class="underline">"Back to Home"</span>
                        </a>
                    </p>
                    <h1 style="margin-bottom: 0.5em;">"Explore Topics"</h1>
                    <p class="text-lg text-gray-600">
                        "Explore our collection of tutorials and articles organized by topic"
                    </p>
                </header>

                <Suspense fallback=move || {
                    view! { <p class="text-center">"Loading categories..."</p> }
                }>

                    {move || match categories.get() {
                        Some(Ok(categories_data)) => {
                            if categories_data.is_empty() {
                                view! {
                                    <div class="text-center text-gray-600">
                                        <p>"No categories found."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <div class="grid grid-cols-1 gap-8 mx-auto mb-8 md:grid-cols-2 lg:grid-cols-3">
                                        {categories_data
                                            .into_iter()
                                            .map(|category| {
                                                let name = category
                                                    .name
                                                    .clone()
                                                    .unwrap_or_else(|| "Untitled Category".to_string());
                                                let slug = category.slug.clone().unwrap_or_default();
                                                let description = category
                                                    .description
                                                    .clone()
                                                    .unwrap_or_else(|| "No description available".to_string());
                                                let href = format!("/category/{}", slug);
                                                let category_initial = name
                                                    .chars()
                                                    .next()
                                                    .unwrap_or('C')
                                                    .to_uppercase()
                                                    .to_string();
                                                let gradient_colors = vec![
                                                    ("from-orange-500", "to-red-500"),
                                                    ("from-blue-500", "to-cyan-500"),
                                                    ("from-purple-500", "to-pink-500"),
                                                    ("from-green-500", "to-emerald-500"),
                                                    ("from-yellow-500", "to-orange-500"),
                                                    ("from-indigo-500", "to-purple-500"),
                                                ];
                                                let color_index = (category.id as usize)
                                                    % gradient_colors.len();
                                                let (from_color, to_color) = gradient_colors[color_index];
                                                view! {
                                                    <a
                                                        href=href
                                                        class="block overflow-hidden rounded-xl shadow-lg transition-all hover:shadow-2xl hover:-translate-y-2 group"
                                                    >
                                                        // <div class="relative h-40 bg-gradient-to-br {from_color} {to_color} flex items-center justify-center">
                                                        // <div class="text-6xl font-bold text-white opacity-90">
                                                        // {category_initial}
                                                        // </div>
                                                        // <div class="absolute inset-0 bg-black opacity-0 transition-opacity group-hover:opacity-10"></div>
                                                        // </div>
                                                        <div class="p-6 bg-white">
                                                            <h3 class="mb-2 text-2xl font-bold text-gray-900 transition-colors group-hover:text-orange-800">
                                                                {name}
                                                            </h3>
                                                            <p class="mb-4 text-base text-gray-600 line-clamp-3">
                                                                {description}
                                                            </p>
                                                            <div class="flex justify-between items-center">
                                                                <span class="text-sm font-semibold text-orange-800 group-hover:underline">
                                                                    "Explore articles"
                                                                    <i class="ml-1 fa-solid fa-arrow-right"></i>
                                                                </span>
                                                            </div>
                                                        </div>
                                                    </a>
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
                                    <p>{format!("Error loading categories: {}", e)}</p>
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
