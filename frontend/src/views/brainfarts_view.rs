use gloo_net::http::Request;
use shared::FantasiforsterInformation;
use yew::{classes, html, Component, Html, Properties};

use crate::{
    components::{
        brainfarts::brainfart_component::BrainfartComponent,
        general::{error_component::ErrorComponent, loading_component::LoadingComponent},
    },
    HelperService,
};

#[derive(Properties, Clone, PartialEq)]
pub struct BrainfartsProps {}

pub enum Message {
    None,
    Fantasiforster(Vec<FantasiforsterInformation>),
}

pub struct BrainfartsView {
    fantasiforster: Vec<FantasiforsterInformation>,
}

impl BrainfartsView {
    fn get_brainfarts(ctx: &yew::Context<Self>) {
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
                let json = serde_json::from_str(&response_text);
                if let Ok(fantasiforster) = json {
                    Message::Fantasiforster(fantasiforster)
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
        Self {
            fantasiforster: vec![],
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::None => false,
            Message::Fantasiforster(fantasiforster) => {
                self.fantasiforster = fantasiforster;
                true
            }
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        if let Some(_) = HelperService::get_jwt_information() {
            let forster = self
                .fantasiforster
                .iter()
                .map(|forster| {
                    html! {<BrainfartComponent forster={forster.clone()}/>}
                })
                .collect::<Html>();
            html! {
                <div>
                if self.fantasiforster.len() > 0{
                    <div class={classes!("flex","items-center","justify-center")}>
                    {forster}
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
}
