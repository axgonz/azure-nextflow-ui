use crate::components::{
    repositories::*,
    dispatchers::*,
};

use crate::models::{
    repositories::*,
    dispatchers::*,
};

use leptos::*;
use web_sys::window;

const STORAGE_KEY_PREFIX: &str = "azure-nextflow-ui";

#[component]
pub fn StatusPage(cx: Scope) -> impl IntoView {
    let repos_store = format!("{}-repositories", STORAGE_KEY_PREFIX);
    let dispatchers_store = format!("{}-dispatchers", STORAGE_KEY_PREFIX);

    let (repos, set_repos) = create_signal(cx, NextflowRepos::load(&repos_store));
    provide_context(cx, repos);
    provide_context(cx, set_repos);

    let (dispatchers, set_dispatchers) = create_signal(cx, NextflowDispatchers::load(&dispatchers_store));
    provide_context(cx, dispatchers);
    provide_context(cx, set_dispatchers);

    // Save repositories to local storage
    create_effect(cx, move |_| {
        if let Ok(Some(storage)) = window().unwrap().local_storage() {
            let objs = repos.get().items;
            
            let json = serde_json::to_string(&objs).expect("Couldn't serialize repositories.");
            log!("Saving to local storage: {:#?}", &json);

            if storage.set_item(&repos_store, &json).is_err() {
                log!("Error while trying to set item in local storage");
            }
        }
    });

    // Save dispatchers to local storage
    create_effect(cx, move |_| {
        if let Ok(Some(storage)) = window().unwrap().local_storage() {
            let objs = dispatchers.get().items;
            
            let json = serde_json::to_string(&objs).expect("Couldn't serialize dispatchers.");
            log!("Saving to local storage: {:#?}", &json);

            if storage.set_item(&dispatchers_store, &json).is_err() {
                log!("Error while trying to set item in local storage");
            }
        }
    });    

    view! { cx,
        <div class="flex flex-col">
            <Repositories />
            <Dispatchers />
        </div>
    }
}