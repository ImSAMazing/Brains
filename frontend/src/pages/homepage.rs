use gloo_net::http::Request;
use log::debug;
use shared::Fantasiforster;
use yew::{classes, html, Component, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    components::general::{loading_component::LoadingComponent, navbar_component::NavbarComponent},
    HelperService, Route,
};

#[derive(Properties, Clone, PartialEq)]
pub struct HomePageProps {}

pub enum Message {
    None,
    Fantasiforster(Vec<Fantasiforster>),
}

pub struct HomePage {}

impl HomePage {}

impl Component for HomePage {
    type Message = Message;
    type Properties = HomePageProps;
    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let resp = Request::get("/api/getbrainfarts")
                .header(
                    "Authorization",
                    &format!(
                        "Bearer {}",
                        HelperService::get_storage()
                            .get_item("token")
                            .unwrap()
                            .unwrap()
                    ),
                )
                .send()
                .await
                .unwrap();

            let response_text = resp.text().await.unwrap();

            if !resp.ok() {
                log::error!(
                    "Received an error while trying to get fantasifoster: {:?}",
                    resp
                );
                Message::None
            } else {
                if let Ok(fantasiforster) = serde_json::from_str(&response_text) {
                    log::debug!("Response: {:?}", fantasiforster);
                    Message::Fantasiforster(fantasiforster)
                } else {
                    log::debug!("IMproper response");
                    Message::None
                }
            }
        });
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::None => false,
            Message::Fantasiforster(fantasiforster) => true,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        if let Some(_) = HelperService::get_jwt_information() {
            html! {
            <div>
                <NavbarComponent/>
                <div>

                </div>
            </div> }
        } else {
            navigator.push(&Route::Login);
            html! {<LoadingComponent/>}
        }
    }
}
