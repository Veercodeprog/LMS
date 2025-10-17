use crate::app::components::Header;
use leptos::prelude::*;
use leptos::*; // required for ElementChild trait
#[component]
pub fn Categories() -> impl IntoView {
    view! {
        <Header />
        <div class="overflow-x-hidden min-h-screen">

            <div class="justify-center items-center mx-auto w-full text-white max-w-[64rem] align-center">
                "Categories page here"
            </div>
        </div>
    }
}
