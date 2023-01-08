use yew::{html, Callback, Component, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{components::authentication::register_form_component::RegisterFormComponent, Route};

#[derive(Properties, Clone, PartialEq)]
pub struct RegisterPageProps {}

pub enum Message {}

pub struct RegisterPage {}

impl RegisterPage {}

impl Component for RegisterPage {
    type Message = Message;
    type Properties = RegisterPageProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        let on_registration = Callback::from(move |value: String| {
            let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
            if let Ok(()) = local_storage.set_item("token", &value) {
                navigator.push(&Route::Home);
            }
        });
        html! {<RegisterFormComponent on_succesfull_registration={on_registration} register_explainer={"Välkommen till Hjärnor!"}/>}
    }
}
