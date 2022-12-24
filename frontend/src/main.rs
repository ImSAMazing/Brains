use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::classes;
use yew::prelude::*;
use yew_router::prelude::*;
#[function_component(Homepage)]
fn homepage() -> Html {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(value)) = local_storage.get_item("token") {
        html! { <div><h1 class={classes!("text-center","text-red-400", "text-lg")}>{ format!("{}", value) }</h1> <a class={classes!("text-red-100")} href="/hello-server">{"Link"}</a></div> }
    } else {
        html! {<Login/>}
    }
}

#[function_component(Login)]
fn login() -> Html {
    html! {<div class="flex items-center justify-center min-h-screen bg-gray-100">
        <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg">
            <h3 class="text-2xl font-bold text-center">{"Login to your account"}</h3>
            <form action="">
                <div class="mt-4">
                    <div>
                        <label class="block" for="email">{"Email"}</label>
                                <input type="text" placeholder={"Email"}
                                    class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Password"}</label>
                                <input type="password" placeholder={"Password"}
                                    class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="flex items-baseline justify-between">
                        <button class="px-6 py-2 mt-4 text-white bg-blue-600 rounded-lg hover:bg-blue-900">{"Login"}</button>
                        <a href="#" class="text-sm text-blue-600 hover:underline">{"Forgot password?"}</a>
                    </div>
                </div>
            </form>
        </div>
    </div>}
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
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! {<Homepage/>}
        }
        Route::Login => html! {<Login />},
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
