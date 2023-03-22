use leptos::*;

#[component]
pub fn ErrorMessage(cx: Scope, msg: Option<String>) -> impl IntoView {
    let msg = create_rw_signal(cx, msg);
    
    view!{cx,
        <Show
            when={move || msg.get().is_some()}
            fallback={move |_cx| view!{cx, }}
        >
            <div class="bg-red-100 rounded px-1 pb-1 overflow-auto text-ellipsis">
                {msg.get().unwrap()}
            </div>
        </Show>
    }
}