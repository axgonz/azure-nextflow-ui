use common::*;

use crate::components::{
    icons::*,
};

use leptos::*;

#[component] 
fn DisplayWorkflow(cx: Scope, workflow: NextflowWorkflow) -> impl IntoView {
    let dispatchers = use_context::<ReadSignal<NextflowDispatchers>>(cx).unwrap();

    let req = DispatchReq {
        config_uri: "https://.../nextflow.config".to_string(),
        pipeline_uri: "https://.../pipeline.nf".to_string(),
        parameters_uri: "https://.../parameters.json".to_string(),
        parameters_json: vec![
            DispatchReqParam {
                name: "myString".to_string(),
                value: "foobar".into()
            },
            DispatchReqParam {
                name: "myBool".to_string(),
                value: true.into()
            },
            DispatchReqParam {
                name: "myInt".to_string(),
                value: 123.into()
            },
            DispatchReqParam {
                name: "myFloat".to_string(),
                value: 0.12.into()
            }
        ],
        auto_delete: true
    };

    let dispatch_workflow = create_action(cx, 
        |input: &DispatchReq| {
            let req = input.to_owned();
            async { Actions::web_action_dispatch_workflow(req).await }
        } 
    );

    let on_click = move |_| {
        dispatch_workflow.dispatch(req.to_owned());
    };

    let submitted = dispatch_workflow.input();
    let pending = dispatch_workflow.pending();
    let dispatch_res = dispatch_workflow.value(); 

    view! { cx,
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
                            let on_click = on_click.to_owned();
                            view! { cx, 
                                <IconButton 
                                    kind=ButtonKind::Button
                                    colour=Some(IconColour::Green)
                                    icon="play-outline".to_string() 
                                    label="Dispatch workflow".to_string() 
                                    on_click=on_click
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
            <Show 
                when={move || pending.get() && submitted.get().is_some()}
                fallback={|_cx| view! { cx, }}
            >
                <pre id="json">{
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
            <Show 
                when={move || !pending.get() && dispatch_res.get().is_some()}
                fallback=|_cx| view! { cx, }
            >
                <pre class="bg-green-100" id="json">{move || format!("{:#?}", dispatch_res.get().unwrap())}</pre>
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
                    colour=Some(IconColour::Red)
                    icon="trash-outline".to_string() 
                    label="Remove repository".to_string() 
                    on_click=on_click_delete
                />
                <div class="w-2" />
                <IconButton 
                    kind=ButtonKind::Button 
                    colour=Some(IconColour::Gray)
                    icon="refresh-outline".to_string() 
                    label="Refresh repository".to_string() 
                    on_click=on_click_refresh
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