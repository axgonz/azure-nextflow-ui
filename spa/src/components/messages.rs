use crate::components::{
    date_time::*,
    icons::*,
};

use crate::models::{
    dispatchers::*,
};

use crate::controllers::{
    actions::*,
    loaders::*,
};

use common::*;

use leptos::*;

#[component]
fn DisplayMessage(cx: Scope, message: Message) -> impl IntoView {
    let (show_params, set_show_params) = create_signal(cx, false);
    let (show_errors, set_show_errors) = create_signal(cx, false);

    let toggle_show_params = move |_| set_show_params.update(|is_set| *is_set = !*is_set);

    let error_message: String = match message.metadata.workflow.errorMessage {
        Some(value) => {
            set_show_errors.set(true);
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
                                          
                // Toggle params button 
                <Show 
                    when={move || show_params.get()}
                    fallback={move |cx| view!{cx,
                        <IconButton 
                            kind=ButtonKind::Button
                            colour=Some(IconColour::Blue)
                            icon="chevron-down-outline".to_string() 
                            label="Show parameters".to_string()  
                            on_click=toggle_show_params 
                        />
                    }}
                >
                    <IconButton 
                        kind=ButtonKind::Button
                        colour=Some(IconColour::Blue)
                        icon="chevron-up-outline".to_string() 
                        label="Hide parameters".to_string()  
                        on_click=toggle_show_params 
                    />
                </Show>
            </div> 

            // Errors  
            <Show 
                when={move || show_errors.get()}
                fallback=|_cx| view! { cx, }
            >
                <pre class="mt-2 bg-red-100 rounded px-1 overflow-auto text-ellipsis" id="json">{&error_message}</pre>
            </Show>

            // Params
            <Show 
                when={move || show_params.get()}
                fallback=|_cx| view! { cx, }
            >
                <pre class="mt-2 bg-gray-700 rounded px-1 text-white overflow-auto text-ellipsis" id="json">{format!("{:#}",&message.metadata.parameters)}</pre>
            </Show>
        </li>
    }
}

#[component]
pub fn Messages(cx: Scope, dispatcher: NextflowDispatcher) -> impl IntoView {
    let set_dispatchers = use_context::<WriteSignal<NextflowDispatchers>>(cx).unwrap();
    let access_token = use_context::<RwSignal<Option<String>>>(cx).unwrap();

    let (count, set_count) = create_signal(cx, 0);
    let (rev_messages, set_rev_messages) = create_signal(cx, false);

    let dispatcher_for_loader = dispatcher.clone();   
    let loader = create_resource(cx, 
        move || count.get(), 
        move |_| { 
            let dispatcher = dispatcher_for_loader.to_owned();
            async move { Loaders::web_load_dispatcher_messages(dispatcher, 32, access_token.get()).await }
        }
    );
    let action = create_action(cx, 
        |input: &(String, u8, Option<String>)| {
            let input = input.clone();
            async move { Actions::web_action_dispatcher_messages_dequeue(input.0, input.1, input.2).await }
        } 
    );    
    
    let fallback = move || view! { cx, <p>"Loading..."</p> };

    let messages = move || {
        if rev_messages.get() {
            loader.read(cx).unwrap_or_default().into_iter().rev().collect::<Vec<Message>>()
        }
        else {
            loader.read(cx).unwrap_or_default()
        }
    };

    let on_click_refresh = {
        move |_| set_count.update(|n| *n += 1)
    };

    let on_click_delete = {
        move |_| set_dispatchers.update(|t| t.remove(dispatcher.id))
    };

    let on_click_rev_messages = move |_| {
        set_rev_messages.update(|b| *b = !*b);
        set_count.update(|n| *n += 1)
    };

    let api_url = dispatcher.api_url.clone();
    let on_click_dequeue = move |_| {
        action.dispatch(
            (
                api_url.to_owned(),
                1,
                access_token.get()
            )
        );
        set_count.update(|n| *n += 1)
    };
    
    view! { cx,
        <li>
            <div class="pt-2 flex">
                <h3 class="font-bold">{dispatcher.api_url}</h3>
                <div class="grow" />
                <Show 
                    when={move || rev_messages.get()}
                    fallback={move |_cx| view!{cx, 
                        <IconButton 
                            kind=ButtonKind::Button 
                            colour=Some(IconColour::Gray)
                            icon="arrow-up-outline".to_string() 
                            label="Sort oldest on bottom".to_string() 
                            on_click=on_click_rev_messages
                        />
                    }}
                >
                    <IconButton 
                        kind=ButtonKind::Button 
                        colour=Some(IconColour::Gray)
                        icon="arrow-down-outline".to_string() 
                        label="Sort oldest on top".to_string() 
                        on_click=on_click_rev_messages
                    />
                </Show>
                <div class="w-2" />
                <IconButton 
                    kind=ButtonKind::Button
                    colour=Some(IconColour::Red)
                    icon="layers-outline".to_string() 
                    label="Dequeue messages".to_string()  
                    on_click=on_click_dequeue
                />
                <div class="w-2" />
                <Show 
                    when={move || messages().iter().count() < 32}
                    fallback={move |_cx| view!{cx, 
                        <Icon
                            colour=Some(IconColour::Disabled)
                            icon="refresh-outline".to_string() 
                        />
                    }}
                >
                    <IconButton 
                        kind=ButtonKind::Button 
                        colour=Some(IconColour::Gray)
                        icon="refresh-outline".to_string() 
                        label="Refresh messages".to_string() 
                        on_click=on_click_refresh
                    />
                </Show>
                <div class="w-2" />
                <IconButton 
                    kind=ButtonKind::Button 
                    colour=Some(IconColour::Red)
                    icon="trash-outline".to_string() 
                    label="Remove dispatcher".to_string() 
                    on_click=on_click_delete
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