use leptos::*;

#[component]
pub fn ProgressBar(
    cx: Scope, 
    value: ReadSignal<i32>,
    #[prop(optional)]
    max: u16,
) -> impl IntoView {
    view! { cx,
        <progress
            max=max
            // hmm... where will we get this from?
            value=move || value.get()
        />
    }
}