use gloo_net::http::Request;
use jwt_simple::prelude::*;
use once_cell::sync::Lazy;
use pages::homepage::HomePage;
use pages::loginpage::LoginPage;
use pages::logoutpage::LogoutPage;
use pages::registerpage::RegisterPage;
use shared::JwtInformation;
use wasm_bindgen_futures::spawn_local;
use web_sys::Storage;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;

static PUBLIC_KEY: Lazy<RS384PublicKey> = Lazy::new(|| {
    RS384PublicKey::from_pem(
        "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvlovXOLvdmIGZB71gKLi
KkynPnLA1tFQ9DX9i2ANLXSir58FivBjyHP3WZ9FHzvYSEbOzv3ElxVaPWGRaayL
B8UV3gDcjjXFAOCPSe7QypaZ2KpwgRNFWXQ6d11BMA1jp9lVApiAx+wYPhRuflPf
2LDAPt+BBp1ihpNwcO8PreEm9DkKZsP8IDuZ3axQT3UmlYWm4EezQntCP0TyAkPv
m8Y1IErh99GL4z5FyUoR04/lrPHbkZPEOX4fQSRiG211JYbGfDG6K20KxX+Gtew3
GkxP/Kcl8B4ujcYkzJhC/rq5X4MX/5YuJhpTs9C1/0s6Lm7SMW2Nkbj7r3pFXX0o
TwIDAQAB
-----END PUBLIC KEY-----
",
    )
    .unwrap()
});

pub struct HelperService {}
impl HelperService {
    pub fn get_storage() -> Storage {
        web_sys::window().unwrap().local_storage().unwrap().unwrap()
    }

    pub fn get_jwt_information() -> Option<JwtInformation> {
        let storage = Self::get_storage();
        if let Ok(Some(token)) = storage.get_item("token") {
            if let Ok(token) = PUBLIC_KEY.verify_token(&token, None) {
                Some(token.custom)
            } else {
                None
            }
        } else {
            None
        }
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
