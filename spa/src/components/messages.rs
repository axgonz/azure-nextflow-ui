use common::*;

use crate::components::{
    date_time::*,
    icons::*,
};

use leptos::*;

#[component]
fn DisplayMessage(cx: Scope, message: Message) -> impl IntoView {
    let (show_params, set_show_params) = create_signal(cx, false);
    let toggle_show_params = move |_| set_show_params.update(|is_set| *is_set = !*is_set);

    let (show_errors, _set_show_errors) = create_signal(cx, true);
    // let toggle_show_errors = move |_| set_show_errors.update(|is_set| *is_set = !*is_set);

    // let mut has_error_message: bool = false;
    let error_message: String = match message.metadata.workflow.errorMessage {
        Some(value) => {
            // has_error_message = true;
            value
        },
        None => "".to_string()
    };

    view! { cx,
        <li class="my-2 py-1 px-2 bg-gray-200 rounded">
            <div class="flex">
                <div class="mr-2">
                    <Date value=message.utcTime.clone() />
                </div>
                <div class="mr-2">
                    <Time value=message.utcTime />
                </div>
                <div class="mr-2 w-24">{message.event}</div>
                <div class="mr-2">{message.runName}</div>
                <div class="grow"></div>
                
                // // Toggle errors button
                // <Show 
                //     when={move || has_error_message}
                //     fallback=|_cx| view! { cx, }
                // >
                //     <RedButton text="Errors".to_string() on_click=toggle_show_errors />
                // </Show>
                            
                // Toggle params button 
                <IconButton 
                    kind=ButtonKind::Button
                    colour=Some(IconColour::Blue)
                    icon="chevron-down-outline".to_string() 
                    label="Show parameters".to_string()  
                    on_click=toggle_show_params 
                />
            </div> 

            // Errors  
            <Show 
                when={move || show_errors.get()}
                fallback=|_cx| view! { cx, }
            >
                <pre class="bg-red-100" id="json">{&error_message}</pre>
            </Show>

            // Params
            <Show 
            when={move || show_params.get()}
            fallback=|_cx| view! { cx, }
            >
                <pre class="bg-blue-100" id="json">{format!("{:#}",&message.metadata.parameters)}</pre>
            </Show>
        </li>
    }
}

#[component]
pub fn Messages(cx: Scope, dispatcher: NextflowDispatcher) -> impl IntoView {
    let set_dispatchers = use_context::<WriteSignal<NextflowDispatchers>>(cx).unwrap();

    let (count, set_count) = create_signal(cx, 0);

    let dispatcher_for_loader = dispatcher.clone();   
    let loader = create_resource(cx, 
        move || count.get(), 
        move |_| { 
            let dispatcher = dispatcher_for_loader.to_owned();
            async { Loaders::web_load_dispatcher_messages(dispatcher).await }
        }
    );
    
    let fallback = move || view! { cx, <p>"Loading..."</p> };
    let messages = move || loader.read(cx).unwrap_or_default();

    let on_click_refresh = {
        move |_| set_count.update(|n| *n += 1)
    };

    let on_click_delete = {
        move |_| set_dispatchers.update(|t| t.remove(dispatcher.id))
    };
    
    view! { cx,
        <li>
            <div class="pt-2 flex">
                <h3 class="font-bold">{dispatcher.url}</h3>
                <div class="grow" />
                <IconButton 
                    kind=ButtonKind::Button 
                    colour=Some(IconColour::Red)
                    icon="trash-outline".to_string() 
                    label="Remove dispatcher".to_string() 
                    on_click=on_click_delete
                />
                <div class="w-2" />
                <IconButton 
                    kind=ButtonKind::Button 
                    colour=Some(IconColour::Gray)
                    icon="refresh-outline".to_string() 
                    label="Refresh messages".to_string() 
                    on_click=on_click_refresh
                />
            </div>
            <Suspense fallback=fallback>
            <ul>
                <For
                    each=messages
                    key={|message| message.runId.clone() }
                    view={move |cx, message| {
                        view! {
                            cx,
                            <DisplayMessage message />
                        }
                    }}
                />
            </ul>
            </Suspense>
        </li>
    }
}