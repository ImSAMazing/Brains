use yew::{classes, html, Component, Html, Properties};
use yew_router::{prelude::Link, scope_ext::RouterScopeExt};

use crate::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct NavLinkProps {
    pub to: Route,
    pub text: String,
}

pub enum Message {}

pub struct NavLinkComponent {}

impl NavLinkComponent {
    fn is_active(&self, ctx: &yew::Context<Self>) -> bool {
        let route: Route = ctx.link().route().unwrap();
        route == ctx.props().to
    }
}
impl Component for NavLinkComponent {
    type Message = Message;
    type Properties = NavLinkProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let mut classes = classes!("block", "py-2", "pl-3", "pr-4", "rounded", "md:p-0",);

        let props = ctx.props().clone();
        classes.extend(if self.is_active(ctx) {
            classes!(
                "text-white",
                "bg-blue-700",
                "md:bg-transparent",
                "md:text-blue-700",
                "dark:text-white"
            )
        } else {
            classes!(
                "text-gray-700",
                "hover:bg-gray-100",
                "md:hover:bg-transparent",
                "md:border-0",
                "md:hover:text-blue-700",
                "dark:text-gray-400",
                "md:dark:hover:text-white",
                "dark:hover:bg-gray-700",
                "dark:hover:text-white",
                "md:dark:hover:bg-transparent"
            )
        });
        html! {
            <Link<Route> to={props.to} classes={classes}>{props.text}</Link<Route>>
        }
    }
}
