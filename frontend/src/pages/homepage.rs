use yew::{classes, html, Component, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    components::general::{
        loading_component::LoadingComponent, navbar_component::NavbarComponent,
        user_info_component::UserInfoComponent,
    },
    HelperService, Route,
};

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
        if let Some(_) = self.get_jwt_information() {
            html! { <div><NavbarComponent/></div> }
        } else {
            navigator.push(&Route::Login);
            html! {<LoadingComponent/>}
        }
    }
}
