use yew::{html, Callback, Component, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    components::authentication::login_form_component::LoginFormComponent, HelperService, Route,
};

#[derive(Properties, Clone, PartialEq)]
pub struct LoginPageProps {}

pub enum Message {}

pub struct LoginPage {}

impl LoginPage {}

impl HelperService for LoginPage {}
impl Component for LoginPage {
    type Message = Message;
    type Properties = LoginPageProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        let on_login = Callback::from(move |value: String| {
            let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
            if let Ok(()) = local_storage.set_item("token", &value) {
                navigator.push(&Route::Home);
            }
        });
        html! {<LoginFormComponent on_succesfull_login={on_login} login_explainer={"Välkommen till Hjärnor!"}/>}
    }
}
