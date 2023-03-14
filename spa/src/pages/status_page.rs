use common::*;

use crate::components::{
    repositories::*,
    dispatchers::*,
};

use leptos::*;

#[component]
pub fn StatusPage(cx: Scope) -> impl IntoView {
    let (repos, set_repos) = create_signal(cx, NextflowRepos::new());
    provide_context(cx, repos);
    provide_context(cx, set_repos);

    let (dispatchers, set_dispatchers) = create_signal(cx, NextflowDispatchers::new());
    provide_context(cx, dispatchers);
    provide_context(cx, set_dispatchers);

    view! { cx,
        <div class="flex flex-col">
            <Repositories />
            <Dispatchers />
        </div>
    }
}