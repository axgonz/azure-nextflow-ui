use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex space-x-4 ">
            <A href="/">"Home"</A>
            <A href="/about">"About"</A>
            <A href="/status">"Status"</A>
        </div>
    }
}