use crate::api::strapi_categories_data::fetch_all_categories;
use leptos::prelude::*;
use leptos::*; // required for ElementChild trait
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
                    <ul class="flex overflow-auto justify-between py-3 px-2 w-full list-reset scrolling-touch">
                        <li class="mx-2 lg:mx-0">"Loading categories..."</li>
                    </ul>
                }
            }>
                {move || match categories.get() {
                    Some(Ok(cats)) => {
                        view! {
                            <ul class="flex overflow-auto justify-between py-3 px-2 w-full list-reset scrolling-touch">
                                {cats
                                    .into_iter()
                                    .map(|cat| {
                                        let name = cat
                                            .name
                                            .clone()
                                            .unwrap_or_else(|| "Untitled".to_string());
                                        let slug = cat.slug.clone().unwrap_or_default();
                                        let href = format!("/category/{}", slug);
                                        view! {
                                            <li class="mx-2 lg:mx-0">
                                                <a
                                                    href=href
                                                    class="font-sans no-underline hover:underline text-grey-darker"
                                                >
                                                    {name}
                                                </a>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <ul class="flex overflow-auto justify-between py-3 px-2 w-full list-reset scrolling-touch">
                                <li class="mx-2 text-red-600 lg:mx-0">{format!("Error: {}", e)}</li>
                            </ul>
                        }
                            .into_any()
                    }
                    None => {
                        view! {
                            <ul class="flex overflow-auto justify-between py-3 px-2 w-full list-reset scrolling-touch">
                                <li class="mx-2 lg:mx-0">"…"</li>
                            </ul>
                        }
                            .into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
