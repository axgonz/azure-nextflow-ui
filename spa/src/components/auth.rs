use crate::env::*;

use crate::controllers::{
    auth::*,
};

use leptos::*;

use web_sys::{
    window, MouseEvent,
};

const CLIENT_SECRET: Option<String> = None;

#[component]
pub fn Auth(cx: Scope) -> impl IntoView {
    let access_token = use_context::<RwSignal<Option<String>>>(cx).unwrap();

    let loader_begin = create_resource(cx, 
        move || (), 
        move |_| { 
            async { 
                Auth::begin(
                    CLIENT_ID.to_string(), 
                    CLIENT_SECRET,
                    ISSUER_URL.to_string(),
                    REDIRECT_URL.to_string(),
                ).await 
            }
        }
    );

    let loader_complete = create_resource(cx, 
        move || (), 
        move |_| { 
            async { 
                match Auth::complete(
                    CLIENT_ID.to_string(), 
                    CLIENT_SECRET,
                    ISSUER_URL.to_string(),
                    REDIRECT_URL.to_string(),
                ).await {
                    Ok(auth) => {
                        Some(auth)
                    }
                    Err(_) => {
                        log!("Login failed");
                        None
                    }
                }
            }
        }
    );
    provide_context(cx, loader_complete);

    let on_click_login = move |_cx: MouseEvent| {
        let window = window().unwrap();
        let location = window.location();

        // User has clicked login and we are about to be redirected.
        //  when we redirect we will loose state so we need to store
        //  our code verifiers 
        loader_begin.read(cx).unwrap().set_proof();

        // Navigate user to issuer
        if location.assign(&loader_begin.read(cx).unwrap().auth_url.unwrap()).is_err(){
            log!("Unable to navigate to login url");
        };
    };

    let on_click_logout = move |_cx: MouseEvent| {
        let window = window().unwrap();
        let location = window.location();

        // Clear tokens
        access_token.set(None);
        loader_begin.read(cx).unwrap().clear_access_token();
        loader_begin.read(cx).unwrap().clear_refresh_token();

        // Navigate user to home
        if location.assign("/logout").is_err(){
            log!("Unable to navigate to logout url");
        };
    };    

    let when_ready_to_login = move || {
        let mut b = false;
        if loader_begin.read(cx).is_some() {
            if loader_begin.read(cx).unwrap().auth_url.is_some() {
                b = true
            }
        }
        return b
    };

    let when_ready_to_logout = move || {
        let mut b = false;
        if loader_complete.read(cx).is_some() {
            if loader_complete.read(cx).unwrap().is_some() {
                if loader_complete.read(cx).unwrap().unwrap().access_token.is_some() &&
                    loader_complete.read(cx).unwrap().unwrap().refresh_token.is_some()
                {
                    access_token.set(Some(
                        loader_complete
                            .read(cx)
                            .unwrap()
                            .unwrap()
                            .access_token
                            .unwrap()
                            .secret()
                            .clone()
                        )
                    );
                    b = true
                }
            }

        }
        return b
    };    

    // fallbacks
    let none = move |_cx: Scope| view!{cx, };
    let _loading = move |_cx: Scope| view!{cx, <p>"Loading..."</p>};
    let _waiting = move |_cx: Scope| view!{cx, <p>"Waiting..."</p>};

    view! {cx,
        <Show 
            when={ move || when_ready_to_login() && !when_ready_to_logout() }
            fallback=none
        >
            <a href="" on:click=on_click_login>"Login"</a>
        </Show>
        <Show 
            when={move || when_ready_to_logout()}
            fallback=none
        >
            <a href="" on:click=on_click_logout>"Logout"</a>
        </Show>        
    }
}