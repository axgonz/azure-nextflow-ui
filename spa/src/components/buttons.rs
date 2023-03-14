use leptos::{*, ev::MouseEvent};

/// A pre-styled button component
#[component]
pub fn BlueButton<F>(cx: Scope,
    /// Text to display on the button 
    text: String, 
    /// Closure to call when clicked
    on_click: F) -> impl IntoView
where F: FnMut(MouseEvent) + 'static 
{
    view! {cx,
        <button 
            on:click=on_click
            class="bg-blue-500 hover:bg-blue-700 text-white py-1 px-1 rounded"
        >
            {text}
        </button>
    }
}

/// A pre-styled button component
#[component]
pub fn RedButton<F>(cx: Scope,
    /// Text to display on the button 
    text: String, 
    /// Closure to call when clicked
    on_click: F) -> impl IntoView
where F: FnMut(MouseEvent) + 'static {
    view! {cx,
        <button 
            on:click=on_click
            class="bg-red-500 hover:bg-red-700 text-white py-1 px-2 rounded"
        >
            {text}
        </button>
    }
}

/// A pre-styled button component
#[component]
pub fn GreenButton<F>(cx: Scope,
    /// Text to display on the button 
    text: String, 
    /// Closure to call when clicked
    on_click: F) -> impl IntoView
where F: FnMut(MouseEvent) + 'static {
    view! {cx,
        <button 
            on:click=on_click
            class="bg-green-500 hover:bg-green-700 text-white py-1 px-2 rounded"
        >
            {text}
        </button>
    }
}