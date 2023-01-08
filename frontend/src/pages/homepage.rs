use gloo_net::http::Request;
use log::debug;
use shared::{Fantasiforster, FantasiforsterInformation};
use yew::{classes, html, Component, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    components::{
        brainfarts::brainfart_component::BrainfartComponent,
        general::{loading_component::LoadingComponent, navbar_component::NavbarComponent},
    },
    views::brainfarts_view::BrainfartsView,
    HelperService, Route,
};

#[derive(Properties, Clone, PartialEq)]
pub struct HomePageProps {}

pub enum Message {}

pub struct HomePage {}

impl HomePage {}

impl Component for HomePage {
    type Message = Message;
    type Properties = HomePageProps;
    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        if let Some(_) = HelperService::get_jwt_information() {
            html! {
            <div>
                <NavbarComponent/>
                <div>
                <BrainfartsView/>
                </div>
            </div> }
        } else {
            navigator.push(&Route::Login);
            html! {<LoadingComponent/>}
        }
    }
}
