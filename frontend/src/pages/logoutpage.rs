use yew::{html, Component, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{components::general::loading_component::LoadingComponent, HelperService, Route};

#[derive(Properties, Clone, PartialEq)]
pub struct LogoutPageProps {}

pub enum Message {}

pub struct LogoutPage {}

impl LogoutPage {}

impl HelperService for LogoutPage {}
impl Component for LogoutPage {
    type Message = Message;
    type Properties = LogoutPageProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let target = if let Err(_) = local_storage.delete("token") {
            &Route::Home
        } else {
            &Route::Login
        };
        navigator.push(target);
        html! {<LoadingComponent/>}
    }
}
