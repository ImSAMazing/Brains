use yew::{html, Component, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    components::general::{loading_component::LoadingComponent, navbar_component::NavbarComponent},
    views::{brainfarts_view::BrainfartsView, new_brainfart_view::NewBrainfartView},
    HelperService, Route,
};

#[derive(Properties, Clone, PartialEq)]
pub struct HomePageProps {}

pub enum Message {
    ReRender,
}

pub struct HomePage {
    counter: u8,
}

impl HomePage {}

impl Component for HomePage {
    type Message = Message;
    type Properties = HomePageProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self { counter: 0 }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ReRender => {
                self.counter += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        let on_new_brainfart = ctx.link().callback(move |_s: String| Message::ReRender);
        if let Some(_) = HelperService::get_jwt_information() {
            html! {
            <div>
                <NavbarComponent/>
                <div>
                <NewBrainfartView on_new_brainfart={on_new_brainfart}/>
                </div>
                <div>
                <BrainfartsView counter={self.counter}/>
                </div>

            </div> }
        } else {
            navigator.push(&Route::Login);
            html! {<LoadingComponent/>}
        }
    }
}
