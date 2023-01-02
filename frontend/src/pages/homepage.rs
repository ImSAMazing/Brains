use yew::{classes, html, Component, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{components::general::loading_component::LoadingComponent, HelperService, Route};

#[derive(Properties, Clone, PartialEq)]
pub struct HomePageProps {}

pub enum Message {}

pub struct HomePage {}

impl HomePage {}

impl HelperService for HomePage {}
impl Component for HomePage {
    type Message = Message;
    type Properties = HomePageProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        let local_storage = self.get_storage();
        if let Ok(Some(value)) = local_storage.get_item("token") {
            html! { <div><h1 class={classes!("text-center","text-red-400", "text-lg")}>{ format!("{}", value) }</h1> <a class={classes!("text-red-100")} href="/hello-server">{"Link"}</a></div> }
        } else {
            navigator.push(&Route::Login);
            html! {<LoadingComponent/>}
        }
    }
}