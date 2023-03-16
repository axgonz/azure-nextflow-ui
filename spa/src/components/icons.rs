use leptos::{*, ev::MouseEvent};

use strum_macros::{
    EnumString,
    Display
};

#[derive(EnumString, Display)]
pub enum ButtonKind {
    Button,
    Reset,
    Submit,
}

pub enum IconColour {
    Red,
    Blue,
    Green,
    Gray,
    Disabled,
}

/// A pre-styled icon button component (ToDo, add type and label)
#[component]
pub fn IconButton<F>(cx: Scope,
    /// The kind of button
    kind: ButtonKind,
    /// The background colour to use
    colour: Option<IconColour>,
    /// The ion-icon name to use
    icon: String,
    /// The label to display on hover
    label: String,
    /// Closure to call when clicked
    on_click: F) -> impl IntoView
where F: FnMut(MouseEvent) + 'static {
    let colour = match colour {
        Some(value) => value,
        None => IconColour::Blue
    };

    let mut class: String = "flex text-white font-bold py-1 px-1 rounded".to_string();
    match colour {
        IconColour::Blue  => {class = format!("{} bg-blue-500 hover:bg-blue-700", class)},
        IconColour::Red   => {class = format!("{} bg-red-500 hover:bg-red-700", class)},
        IconColour::Green => {class = format!("{} bg-green-500 hover:bg-green-700", class)},
        IconColour::Gray  => {class = format!("{} bg-gray-500 hover:bg-gray-700", class)},
        IconColour::Disabled  => {class = format!("{} bg-gray-300 hover:bg-gray-500", class)},
    }
        
    view! {cx,
        <button
            type=kind.to_string()
            on:click=on_click
            role="button"
            title=&label
        >
            <span class="sr-only">{label}</span>
            <ion-icon class=class name=icon />
        </button>
    }
}


/// A pre-styled button component (ToDo, add type and label)
#[component]
pub fn Icon(cx: Scope,
    /// The background colour to use
    colour: Option<IconColour>,
    /// The ion-icon name to use
    icon: String) -> impl IntoView
{
    let colour = match colour {
        Some(value) => value,
        None => IconColour::Blue
    };

    let mut class: String = "icon flex text-white font-bold py-1 px-1 rounded".to_string();
    match colour {
        IconColour::Blue        => {class = format!("{} bg-blue-500", class)},
        IconColour::Red         => {class = format!("{} bg-red-500", class)},
        IconColour::Green       => {class = format!("{} bg-green-500", class)},
        IconColour::Gray        => {class = format!("{} bg-gray-500", class)},
        IconColour::Disabled    => {class = format!("{} bg-gray-300", class)},
    }
        
    view! {cx,
        <ion-icon class=class name=icon />
    }
}