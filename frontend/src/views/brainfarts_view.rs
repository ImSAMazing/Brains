use gloo_net::http::Request;
use shared::BrainfartInformation;
use yew::{classes, html, Component, Html, Properties};

use crate::{
    components::{
        brainfarts::brainfart_component::BrainfartComponent,
        general::{error_component::ErrorComponent, loading_component::LoadingComponent},
    },
    HelperService,
};

#[derive(Properties, Clone, PartialEq)]
pub struct BrainfartsProps {
    pub counter: u8,
}

pub enum Message {
    None,
    Brainfart(Vec<BrainfartInformation>),
}

pub struct BrainfartsView {
    brainfarts: Vec<BrainfartInformation>,
}

impl BrainfartsView {
    fn get_brainfarts(ctx: &yew::Context<Self>) {
        ctx.link().send_future(async move {
            let resp = HelperService::add_authorization_header(Request::get("/api/getbrainfarts"))
                .send()
                .await
                .unwrap();

            let response_text = resp.text().await.unwrap();

            if !resp.ok() {
                log::error!(
                    "Received an error while trying to get brainfarts: {:?}",
                    resp
                );
                Message::None
            } else {
                let json = serde_json::from_str(&response_text);
                if let Ok(brainfarts) = json {
                    Message::Brainfart(brainfarts)
                } else {
                    if let Err(e) = json {
                        log::debug!("IMproper response: {:?}", e);
                        Message::None
                    } else {
                        Message::None
                    }
                }
            }
        });
    }
}

impl Component for BrainfartsView {
    type Message = Message;
    type Properties = BrainfartsProps;
    fn create(ctx: &yew::Context<Self>) -> Self {
        Self::get_brainfarts(ctx);
        Self { brainfarts: vec![] }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::None => false,
            Message::Brainfart(brainfarts) => {
                self.brainfarts = brainfarts;
                true
            }
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        log::debug!("Viewed called");
        if let Some(_) = HelperService::get_jwt_information() {
            let brainfart = self
                .brainfarts
                .iter()
                .map(|brainfart| {
                    html! {<BrainfartComponent brainfart={brainfart.clone()}/>}
                })
                .collect::<Html>();
            html! {
                <div>
                if self.brainfarts.len() > 0{
                    <div class={classes!("flex","items-center","justify-between","flex-col")}>
                    {brainfart}
                    </div>
                }else{
                    <LoadingComponent/>
                }
                </div>
            }
        } else {
            html! {<ErrorComponent/>}
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        log::debug!("Changed");
        Self::get_brainfarts(ctx);
        true
    }
}
