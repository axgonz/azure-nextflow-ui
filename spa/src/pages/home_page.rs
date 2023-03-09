use crate::models::{
    app_state::AppState
};

use crate::components::{
    progress_bar::*,
};

use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Find global state form cx
    let data = use_context::<RwSignal<AppState>>(cx).expect("state to have been provided");

    // `create_slice` lets us create a "lens" into the data
    let (global_count, set_global_count) = create_slice(
        cx,
        // we take a slice *from* `state`
        data,
        // our getter returns a "slice" of the data
        |data| data.count,
        // our setter describes how to mutate that slice, given a new value
        |data, n| data.count = n,
    );

    // Define on_click function
    let on_click_global = move |_| set_global_count.set(global_count.get() + 1);

    // Local count
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|n| *n += 1);
    let _double_count = move || count.get() * 2;

    view! { cx,
        <div class="flex flex-col space-x-4 space-y-4">
            <div class="text-4xl">"Home Page"</div>
            <ProgressBar max=2 value=count/>
            <button on:click=on_click class:red={move || count.get() & 1 == 1} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-64">
                "Count: " {move|| count.get()}
            </button>
            <button on:click=on_click_global class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-64">
                "Global Count: " {move || global_count.get()}
            </button>        
        </div>
    }
}