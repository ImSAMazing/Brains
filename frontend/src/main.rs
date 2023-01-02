use gloo_net::http::Request;
use pages::homepage::HomePage;
use pages::loginpage::LoginPage;
use pages::logoutpage::LogoutPage;
use pages::registerpage::RegisterPage;
use wasm_bindgen_futures::spawn_local;
use web_sys::Storage;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

pub trait HelperService {
    fn get_storage(&self) -> Storage {
        web_sys::window().unwrap().local_storage().unwrap().unwrap()
    }
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
    #[at("/logout")]
    Logout,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! {<HomePage/>}
        }
        Route::Login => html! {<LoginPage />},
        Route::Register => html! {<RegisterPage/>},
        Route::Logout => html! {<LogoutPage/>},
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
