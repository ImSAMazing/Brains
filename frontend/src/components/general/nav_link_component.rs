use yew::{classes, html, Component, Html, Properties};
use yew_router::{navigator, prelude::Link, scope_ext::RouterScopeExt};

use crate::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct NavLinkProps {
    to: Route,
}

pub enum Message {}

pub struct NavLinkComponent {}
impl Component for NavLinkComponent {
    type Message = Message;
    type Properties = NavLinkProps;
    fn create(ctx: &yew::Context<Self>) -> Self {
        let route: Route = ctx.link().route().unwrap();
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <Link<Route> to={Route::Home} classes={classes!("block","py-2","pl-3","pr-4","text-white","bg-blue-700","rounded","md:bg-transparent","md:text-blue-700","md:p-0","dark:text-white")}>{"Home"}</Link<Route>>
        }
    }
}
