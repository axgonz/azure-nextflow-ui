mod components;
mod controllers;
mod models;
mod pages;

use crate::components::{
    nav_bar::*,
};

use crate::models::{
    app_state::AppState,
};

use crate::pages::{
    home_page::*,
    about_page::*,
};

use leptos::*;
use leptos_router::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let access_token = create_rw_signal(cx, None::<String>);
    provide_context(cx, access_token);

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // Create a provide global state
    let data = create_rw_signal(cx, AppState::new());
    provide_context(cx, data);
    
    view! {
        cx,
        <Title text="Nxfutil"/>
        <Router>
            <NavBar />
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/about" view=|cx| view! { cx, <AboutPage/> }/>
                    <Route path="/login" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/logout" view=|cx| view! { cx, <AboutPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App /> })
}