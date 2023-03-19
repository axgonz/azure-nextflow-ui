use leptos::*;
use leptos_router::*;

use crate::components::{auth::*};

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        // <div class="flex space-x-4 px-2 py-3 mx-1 my-1 rounded bg-gradient-to-r from-gray-800 to-fuchsia-700 text-white">
        <div class="flex space-x-4 px-2 py-3 mx-1 my-1 rounded bg-gray-700 text-white">
            <A href="/">"Home"</A>
            <A href="/about">"About"</A>
            <div class="grow" />
            <Auth />
        </div>
    }
}