mod components;
mod models;
mod pages;

use crate::components::{
    nav_bar::NavBar,
    nav_bar::NavBarProps,
};

use crate::models::{
    app_state::AppState,
};

use crate::pages::{
    home_page::HomePage,
    home_page::HomePageProps,
    about_page::AboutPage,
    about_page::AboutPageProps
};

use leptos::*;
use leptos_router::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    log!("Hello Log! (from App component)");

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // Create a provide global state
    let data = create_rw_signal(cx, AppState::new());
    provide_context(cx, data);
    
    view! {
        cx,
        <Title text="Welcome to Leptos"/>
        <Router>
            <NavBar />
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/about" view=|cx| view! { cx, <AboutPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App /> })
}