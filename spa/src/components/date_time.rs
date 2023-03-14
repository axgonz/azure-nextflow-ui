use leptos::*;

use chrono::{DateTime, Local};

/// A pre-styled div to display time in local timezone
#[component]
pub fn Time(cx: Scope, 
    /// A string representing the time in rfc3339 format (e.g. 2023-03-13T03:42:40Z).
    value: String) -> impl IntoView 
{
    let rfc3339 = DateTime::parse_from_rfc3339(&value).unwrap();
    let local: DateTime<Local> = DateTime::from(rfc3339);

    view!{cx,
        <p>{format!("{}", local.format("%H:%M:%S"))}</p>
    }
}

/// A pre-styled div to display date in local timezone
#[component]
pub fn Date(cx: Scope, 
    /// A string representing the time in rfc3339 format (e.g. 2023-03-13T03:42:40Z).
    value: String) -> impl IntoView 
{
    let rfc3339 = DateTime::parse_from_rfc3339(&value).unwrap();
    let local: DateTime<Local> = DateTime::from(rfc3339);

    view!{cx,
        <p>{format!("{}", local.format("%Y-%m-%d"))}</p>
    }
}