use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::classes;
use yew::prelude::*;
use yew_router::navigator;
use yew_router::prelude::*;

mod components;
use crate::components::authentication::login_form_component::LoginFormComponent;
use crate::components::authentication::register_form_component::RegisterFormComponent;
use crate::components::general::loading_component::LoadingComponent;

#[function_component(Homepage)]
fn homepage() -> Html {
    let navigator = use_navigator().unwrap();
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(value)) = local_storage.get_item("token") {
        html! { <div><h1 class={classes!("text-center","text-red-400", "text-lg")}>{ format!("{}", value) }</h1> <a class={classes!("text-red-100")} href="/hello-server">{"Link"}</a></div> }
    } else {
        navigator.push(&Route::Login);
        html! {<LoadingComponent/>}
    }
}

#[function_component(Login)]
fn login() -> Html {
    html! {<LoginFormComponent login_explainer={"Välkommen till Hjärnor!"}/>}
}

#[function_component(Register)]
fn register() -> Html {
    let navigator = use_navigator().unwrap();
    let on_registration = Callback::from(move |value: String| {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        if let Ok(()) = local_storage.set_item("token", &value) {
            navigator.push(&Route::Home);
        }
    });
    html! {<RegisterFormComponent on_succesfull_registration={on_registration} register_explainer={"Välkommen till Hjärnor!"}/>}
}
#[function_component(HelloServer)]
fn hello_server() -> Html {
    let data = use_state(|| None);

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/hello").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Got server response: "}{data}<br/><a href="/">{"Back"}</a></div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! {<Homepage/>}
        }
        Route::Login => html! {<Login />},
        Route::Register => html! {<Register/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
