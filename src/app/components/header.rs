use crate::api::strapi_categories_data::fetch_all_categories;
use leptos::prelude::*;
use leptos::*; // required for ElementChild trait
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

#[component]
pub fn Header() -> impl IntoView {
    let location = use_location();
    let path = location.pathname;
    let categories = LocalResource::new(|| async { fetch_all_categories().await });

    view! {
        <div class="container px-4 mx-auto lg:px-0">
            <header class="flex items-center py-3 border-b border-grey-light">
                <a href="#" class="font-sans no-underline hover:underline text-grey-darker">
                    Subscribe
                </a>

                <nav class="flex gap-6 items-center ml-auto text-grey-darker">
                    <a href="/" class="mx-4 text-3xl no-underline hover:underline">
                        Home
                    </a>
                    <a href="/categories" class="mx-4 text-3xl no-underline hover:underline">
                        Categories
                    </a>
                    <a href="/articles" class="mx-4 text-3xl no-underline hover:underline">
                        Articles
                    </a>
                </nav>

                <div class="flex items-center ml-auto">
                    <a href="#" class="no-underline text-grey-darker">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="20"
                            height="20"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="mx-3 stroke-current"
                        >
                            <circle cx="10.5" cy="10.5" r="7.5"></circle>
                            <line x1="21" y1="21" x2="15.8" y2="15.8"></line>
                        </svg>
                    </a>
                    <button class="p-2 font-sans text-sm bg-transparent rounded border hover:text-white text-grey-darker border-grey-darker hover:bg-grey-darker">
                        Sign up
                    </button>
                </div>
            </header>
            <Suspense fallback=|| {
                view! {
                    <div class="py-5 px-4 bg-gray-900">
                        <div class="flex gap-3">
                            <span class="w-28 h-11 bg-gray-700 rounded-md animate-pulse"></span>
                            <span class="w-32 h-11 bg-gray-700 rounded-md animate-pulse"></span>
                            <span class="w-24 h-11 bg-gray-700 rounded-md animate-pulse"></span>
                        </div>
                    </div>
                }
            }>
                {move || match categories.get() {
                    Some(Ok(cats)) => {
                        let neon_colors = vec![
                            "border-cyan-400 text-cyan-400 hover:bg-cyan-400/20 hover:shadow-cyan-400/50",
                            "border-pink-400 text-pink-400 hover:bg-pink-400/20 hover:shadow-pink-400/50",
                            "border-purple-400 text-purple-400 hover:bg-purple-400/20 hover:shadow-purple-400/50",
                            "border-green-400 text-green-400 hover:bg-green-400/20 hover:shadow-green-400/50",
                            "border-yellow-400 text-yellow-400 hover:bg-yellow-400/20 hover:shadow-yellow-400/50",
                            "border-blue-400 text-blue-400 hover:bg-blue-400/20 hover:shadow-blue-400/50",
                            "border-red-400 text-red-400 hover:bg-red-400/20 hover:shadow-red-400/50",
                            "border-indigo-400 text-indigo-400 hover:bg-indigo-400/20 hover:shadow-indigo-400/50",
                        ];

                        view! {
                            <div class="relative py-5 px-4 bg-gradient-to-br from-gray-900 via-gray-800 to-black border-gray-700 border-y">
                                <div class="absolute inset-0 opacity-20 bg-[radial-gradient(circle_at_50%_50%,rgba(120,119,198,0.3),transparent_50%)]"></div>
                                <ul class="flex overflow-x-auto relative gap-4 scrolling-touch scroll-smooth no-scrollbar">
                                    {cats
                                        .into_iter()
                                        .enumerate()
                                        .map(|(idx, cat)| {
                                            let name = cat
                                                .name
                                                .clone()
                                                .unwrap_or_else(|| "Untitled".to_string());
                                            let slug = cat.slug.clone().unwrap_or_default();
                                            let href = format!("/category/{}", slug);
                                            let neon_class = neon_colors[idx % neon_colors.len()];

                                            view! {
                                                <li class="flex-shrink-0">
                                                    <a
                                                        href=href
                                                        class=format!(
                                                            "block py-3 px-6 font-bold no-underline uppercase bg-transparent rounded-md border-2 shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-2xl whitespace-nowrap text-sm tracking-wider {}",
                                                            neon_class,
                                                        )
                                                    >
                                                        {name}
                                                    </a>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </div>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <div class="py-4 px-4 bg-gray-900 border-y border-red-500/50">
                                <p class="font-bold text-center text-red-400">
                                    {"⚠️ "}{format!("Error: {}", e)}
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                    None => {
                        view! {
                            <div class="py-5 px-4 bg-gray-900">
                                <div class="flex gap-3">
                                    <span class="w-28 h-11 bg-gray-700 rounded-md animate-pulse"></span>
                                    <span class="w-32 h-11 bg-gray-700 rounded-md animate-pulse"></span>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Suspense>

            <Outlet />
        </div>
    }
}
