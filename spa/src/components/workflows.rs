use crate::components::{
    icons::*,
};

use crate::models::{
    repositories::*,
    dispatchers::*,
    params::*,
};

use crate::controllers::{
    actions::*,
    loaders::*,
};

use common::*;

use leptos::*;

use leptos::html::Input;
use web_sys::{
    Event,
    MouseEvent,
};

#[component]
fn DispatchForm(cx: Scope, workflow: NextflowWorkflow) -> impl IntoView {
    let dispatchers = use_context::<ReadSignal<NextflowDispatchers>>(cx).unwrap();

    // Get our form pre-reqs from parent (cx)
    let show_form = use_context::<ReadSignal<bool>>(cx).unwrap();
    let set_show_form = use_context::<WriteSignal<bool>>(cx).unwrap();
    let action = use_context::<Action<(String, bool, DispatchReq), DispatchRes>>(cx).unwrap();

    // Form signals
    let (request, set_request) = create_signal(cx, 
        DispatchReq {
            config_uri: dispatchers.get().config_url(),
            pipeline_uri: workflow.pipeline.url,
            parameters_uri: workflow.parameters.url,
            parameters_json: vec![],
            auto_delete: true
        }
    );
    let (f_what_if, set_f_what_if) = create_signal(cx, true);
    let (params, set_params) = create_signal(cx, DispatchParams::new());
    let (f_add_param_name, set_f_add_param_name) = create_signal(cx, "".to_string());
    let (f_add_param_value, set_f_add_param_value) = create_signal(cx, "".to_string());

    // Form inputs
    let toggle_show = move |_: MouseEvent| {
        set_show_form.update(|b| *b = !*b);
    };   
    let _update_cfg_uri = move |ev: Event| {
        set_request.update(|req| req.config_uri = event_target_value(&ev))
    };
    let _update_pln_uri = move |ev: Event| {
        set_request.update(|req| req.pipeline_uri = event_target_value(&ev))
    };
    let _update_arg_uri = move |ev: Event| {
        set_request.update(|req| req.parameters_uri = event_target_value(&ev))
    };
    let toggle_auto_delete = move |ev: Event| {
        set_request.update(|req| req.auto_delete = event_target_checked(&ev))
    };
    let toggle_what_if = move |ev: Event| {
        set_f_what_if.set(event_target_checked(&ev));
    };
    let update_add_param_name = move |ev: Event| {
        set_f_add_param_name.set(event_target_value(&ev))
    };
    let update_add_param_value = move |ev: Event| {
        set_f_add_param_value.set(event_target_value(&ev))
    };

    // Form NodeRefs
    let ref_add_param_name: NodeRef::<Input> = create_node_ref(cx);
    let ref_add_param_value: NodeRef::<Input> = create_node_ref(cx);

    // Form action
    let on_click_add_param = move |_| {
        let node_add_param_name = ref_add_param_name.get().unwrap();
        let node_add_param_value = ref_add_param_value.get().unwrap();

        set_params.update(
            |params| params.add(
                DispatchParam::new(
                    Uuid::new_v4(),
                    f_add_param_name.get(),
                    f_add_param_value.get()
                )
            )
        );
            
        set_f_add_param_name.set("".to_string());
        set_f_add_param_value.set("".to_string());
        
        node_add_param_name.set_value("");
        node_add_param_value.set_value("");
    };

    let on_click_confirm = move |mouse_event: MouseEvent| {
        toggle_show(mouse_event);

        set_request.update(|req| 
            req.parameters_json = params.get().items
            .iter()
            .map(DispatchReqParam::from)
            .collect::<Vec<DispatchReqParam>>()
        );

        action.dispatch(
            (
                dispatchers.get().api_url(),
                f_what_if.get(),
                request.get(),
            )
        )
    };
    
    view!{cx,
        <Show 
            when={move || show_form.get()}
            fallback=|_cx| view! { cx, }
        >
            <div class="absolute inset-0 bg-black bg-opacity-30 h-screen w-full flex justify-center items-start md:items-center pt-10 md:pt-0">
            <div class="bg-gray-100 rounded px-4 py-4">
            <div class="flex">
                <h2 class="w-64">"Dispatch workflow"</h2>
                <div class="grow" />
                <IconButton 
                    kind=ButtonKind::Button 
                    colour=Some(IconColour::Gray)
                    icon="close-outline".to_string() 
                    label="Cancel".to_string() 
                    on_click=toggle_show
                />
            </div>
            <div class="flex flex-col">               
                <label class="rounded">"Repository"</label>
                <input type="text" value={format!("{}/{}", &workflow.project.org, &workflow.project.repo)} readonly/>

                <label class="rounded">"Project"</label>
                <input type="text" value={&workflow.project.name} readonly/>

                <label class="rounded">"Dispatcher"</label>
                <input type="text" value={dispatchers.get().api_url()} readonly/>                

                <label class="rounded">"Config"</label>
                <input type="text" value={request.get().config_uri} readonly/>

                <label class="rounded">"Pipeline"</label>
                <input type="text" value={request.get().pipeline_uri} readonly/>

                <label class="rounded">"Parameters"</label>
                <input type="text" value={request.get().parameters_uri} readonly/>

                <div class="flex">
                    <label class="rounded">"Auto delete"</label>
                    <div class="grow" />
                    <input id="toggle_auto_delete" type="checkbox"
                        prop:checked={move || request.get().auto_delete}
                        on:input=toggle_auto_delete
                    />
                </div>

                <div class="flex">
                    <label class="rounded">"What if"</label>
                    <div class="grow" />
                    <input id="toggle_what_if" type="checkbox"
                        prop:checked={move || f_what_if.get()}
                        on:input=toggle_what_if
                    />
                </div>

                <p>"Parameter overrides"</p>
                <Show
                    when={|| true}
                    fallback={move |_cx| view! {cx, }}
                >
                    <ul>
                    <For
                        each={move || params.get().items}
                        key={|param| param.id }
                        view={move |cx, param| {
                            view! {
                                cx, 
                                <li>
                                    <div class="flex">
                                        <div>{format!("--{}", param.name)}</div>
                                        <pre>" "</pre>
                                        <div>{param.value}</div>
                                        <div class="grow" />
                                        <IconButton
                                            colour=Some(IconColour::Gray)
                                            icon="trash-outline".to_string()
                                            kind=ButtonKind::Button
                                            label="Add parameter".to_string()
                                            on_click={move |_| { 
                                                set_params.update(|params| params.remove(param.id))
                                            }}
                                        />
                                    </div>
                                </li>
                            }
                        }}
                    />
                    </ul>
                </Show>

                <div class="flex">
                    <input id="add-param-name" type="text" placeholder="name" 
                        node_ref=ref_add_param_name
                        on:input=update_add_param_name 
                        param:value={move || f_add_param_name.get()}
                    />
                    <input id="add-param-value" type="text" placeholder="value" 
                        node_ref=ref_add_param_value
                        on:input=update_add_param_value 
                        param:value={move || f_add_param_value.get()}
                    />
                    <div class="grow" />
                    <IconButton
                        colour=Some(IconColour::Gray)
                        icon="add-outline".to_string()
                        kind=ButtonKind::Button
                        label="Add parameter".to_string()
                        on_click=on_click_add_param
                    />
                </div>                     
               
                <div class="flex">
                    <div class="grow"/>
                    // <button type="submit">"submit"</button>
                    <IconButton 
                        kind=ButtonKind::Submit 
                        colour=Some(IconColour::Blue)
                        icon="checkmark-outline".to_string() 
                        label="Confirm".to_string() 
                        on_click=on_click_confirm
                    />
                </div>
            </div>
            </div>
            </div>
        </Show>
    }
}

#[component] 
fn DisplayWorkflow(cx: Scope, workflow: NextflowWorkflow) -> impl IntoView {
    let dispatchers = use_context::<ReadSignal<NextflowDispatchers>>(cx).unwrap();

    // Setup our form pre-reqs
    let (show_form, set_show_form) = create_signal(cx, false);
    let action = create_action(cx, 
        |input: &(String, bool, DispatchReq)| {
            let input = input.clone();
            async move { Actions::web_action_dispatch_workflow(input.0, input.1, input.2).await }
        } 
    );
    provide_context(cx, show_form);
    provide_context(cx, set_show_form);
    provide_context(cx, action);
    let workflow_for_form = workflow.clone();
    // End form pre-reqs

    let toggle_show_form = move |_| {
        set_show_form.update(|b| *b = !*b);
    };

    let submitted = action.input();
    let pending = action.pending();
    let dispatch_res = action.value(); 

    view! { cx,
        <DispatchForm workflow=workflow_for_form />        
        <li class="my-2 py-1 px-2 bg-gray-200 rounded">
            <div class="flex">
                <a href={&workflow.project.html_url} class="mr-2">{&workflow.project.name}</a>
                <a href={&workflow.pipeline.url} class="mr-2">{&workflow.pipeline.name}</a>
                <a href={&workflow.parameters.url} class="mr-2">{&workflow.parameters.name}</a>
                <div class="grow" />
                <Show 
                    when={move || (pending.get() || dispatchers.get().is_empty()) }
                    fallback={
                        move |cx| {
                            // let on_click = on_click.to_owned();
                            view! { cx, 
                                <IconButton 
                                    kind=ButtonKind::Button
                                    colour=Some(IconColour::Green)
                                    icon="play-outline".to_string() 
                                    label="Dispatch workflow".to_string() 
                                    on_click=toggle_show_form
                                />
                            }
                        } 
                    }
                >
                    <Show
                        when={move || !dispatchers.get().is_empty()}
                        fallback={move |cx| view! {cx, 
                            <Icon
                                colour=Some(IconColour::Disabled)
                                icon="play-outline".to_string() 
                            />
                        }}
                    >
                        <p>"waiting..."</p>
                    </Show>
                </Show>
            </div> 

            // Request
            <Show 
                when={move || pending.get() && submitted.get().is_some()}
                fallback={|_cx| view! { cx, }}
            >
                <pre class="overflow-auto" id="json">{
                    move || {
                        if submitted.get().is_some() {
                            format!("{:#?}", submitted.get().unwrap())
                        }
                        else {
                            format!("")
                        }
                    }
                }
                </pre>
            </Show>
            
            // Response
            <Show 
                when={move || !pending.get() && dispatch_res.get().is_some()}
                fallback=|_cx| view! { cx, }
            >
                <pre class="bg-green-100 overflow-auto" id="json">{move || format!("{:#?}", dispatch_res.get().unwrap())}</pre>
            </Show>
        </li>
    }
} 

#[component]
pub fn Workflows(cx: Scope, repo: NextflowRepo) -> impl IntoView {
    let set_repos = use_context::<WriteSignal<NextflowRepos>>(cx).unwrap();

    let (count, set_count) = create_signal(cx, 0);

    let repo_for_loader = repo.clone();
    let loader = create_resource(cx, 
        move || count.get(), 
        move |_| {
            let repo = repo_for_loader.to_owned();
            async { Loaders::web_load_github_nextflow_workflows(repo).await }
        }
    );

    let fallback = move || view! { cx, <p>"Loading..."</p> };
    let workflows = move || loader.read(cx).unwrap_or_default();

    let on_click_refresh = {
        move |_| set_count.update(|n| *n += 1)
    };

    let on_click_delete = {
        move |_| set_repos.update(|t| t.remove(repo.id))
    };

    view! { cx,
        <li>
            <div class="pt-2 flex">
                <h3 class="font-bold">{repo.org}"/"{repo.name}</h3>
                <div class="grow" />
                <IconButton 
                    kind=ButtonKind::Button 
                    colour=Some(IconColour::Gray)
                    icon="refresh-outline".to_string() 
                    label="Refresh repository".to_string() 
                    on_click=on_click_refresh
                />
                <div class="w-2" />
                <IconButton 
                    kind=ButtonKind::Button 
                    colour=Some(IconColour::Red)
                    icon="trash-outline".to_string() 
                    label="Remove repository".to_string() 
                    on_click=on_click_delete
                />
            </div>
            <Suspense fallback=fallback>
            <ul>
                <For
                    each=workflows
                    key={|workflow| workflow.project.url.clone() }
                    view={move |cx, workflow| {
                        view! {
                            cx, 
                            <DisplayWorkflow workflow=workflow />
                        }
                    }}
                />
            </ul>
            </Suspense>
        </li>
    }
}