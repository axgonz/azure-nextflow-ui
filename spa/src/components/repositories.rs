use crate::components::{
    icons::*,
    workflows::*,
};

use crate::models::{
    repositories::*,
    dispatchers::*,
};

use common::{
    types::*,
};

use leptos::*;

#[component]
pub fn Repositories(cx: Scope) -> impl IntoView {
    let repos = use_context::<ReadSignal<NextflowRepos>>(cx).unwrap();
    let set_repos = use_context::<WriteSignal<NextflowRepos>>(cx).unwrap();

    let dispatchers = use_context::<ReadSignal<NextflowDispatchers>>(cx).unwrap();

    let (show, set_show) = create_signal(cx, false);
    let (new_repo_org, set_new_repo_org) = create_signal(cx, "".to_string());
    let (new_repo_name, set_new_repo_name) = create_signal(cx, "".to_string());
    
    let on_click_add = move |_| {
        log!("{:#?}", show.get());
        set_show.update(|b| *b = !*b);
    };

    let on_click_save = move |_| {
        log!("save: {}/{}", new_repo_org.get(), new_repo_name.get());

        set_repos.update(
            |repos| repos.add(
                NextflowRepo::new(
                    Uuid::new_v4(), 
                    new_repo_org.get(), 
                    new_repo_name.get()
                )
            )
        );

        set_new_repo_org.set("".to_string());
        set_new_repo_name.set("".to_string());
        set_show.update(|b| *b = !*b) 
    };
    
    let on_click_cancel = move |_| {
        log!("cancel");
        set_new_repo_org.set("".to_string());
        set_new_repo_name.set("".to_string());
        set_show.update(|b| *b = !*b) 
    };

    let on_input_org = move |ev| {
        set_new_repo_org.set(event_target_value(&ev));
    };

    let on_input_name = move |ev| {
        set_new_repo_name.set(event_target_value(&ev));
    };

    view!{cx,
        <div class="my-1 mx-2">
            <div class="flex">
                <h3 class="grow text-xl">"Workflows"</h3>
                <Show 
                    when={move || show.get()}
                    fallback=|_cx| view! {cx, }
                >
                    <div class="absolute inset-0 bg-black bg-opacity-30 h-screen w-full flex justify-center items-start md:items-center pt-10 md:pt-0">
                    <div class="bg-gray-100 rounded px-4 py-4">
                    <div class="flex">
                        <h2>"Add repository"</h2>
                        <div class="grow" />
                        <IconButton 
                            kind=ButtonKind::Button 
                            colour=Some(IconColour::Gray)
                            icon="close-outline".to_string() 
                            label="Cancel".to_string() 
                            on_click=on_click_cancel
                        />
                    </div>
                    <div class="flex flex-col">
                        <label class="rounded">"Organization"</label>
                        <input id="org" type="text" on:input=on_input_org prop:value={move || new_repo_org.get()}/>
                        
                        <label class="rounded">"Repository"</label>
                        <input id="rep" type="text" on:input=on_input_name prop:value={ move || new_repo_name.get()}/>

                        <div class="flex">
                            <div class="grow"/>
                            <IconButton 
                                kind=ButtonKind::Button 
                                colour=Some(IconColour::Blue)
                                icon="save-outline".to_string() 
                                label="Save".to_string() 
                                on_click=on_click_save
                            />
                        </div>
                    </div>
                    </div>
                    </div>
                </Show>
                <Show
                    when={move || !dispatchers.get().is_empty()}
                    fallback={move |cx| view! {cx,
                        <Icon
                            colour=Some(IconColour::Disabled)
                            icon="add-outline".to_string() 
                        />
                    }}
                >
                    <IconButton 
                        kind=ButtonKind::Button 
                        colour=Some(IconColour::Blue)
                        icon="add-outline".to_string() 
                        label="Add repository".to_string() 
                        on_click=on_click_add
                    />
                </Show>
            </div>
            <ul>
                <For
                    each={move || repos.get().items}
                    key={|repo| repo.id }
                    view={move |cx, repo| {
                        view! {
                            cx, 
                            <Workflows repo />
                        }
                    }}
                />
            </ul>
        </div>
    }
}