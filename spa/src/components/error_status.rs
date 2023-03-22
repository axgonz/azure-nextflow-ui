use leptos::*;

#[component]
pub fn ErrorStatus(cx: Scope, msg: Option<String>) -> impl IntoView {
    let msg = create_rw_signal(cx, msg);
    
    view!{cx,
        <Show
            when={move || msg.get().is_some()}
            fallback={move |_cx| view!{cx, }}
        >
            <div class="px-1 pb-1 bg-red-100">
                {msg.get().unwrap()}
            </div>
        </Show>
    }
}