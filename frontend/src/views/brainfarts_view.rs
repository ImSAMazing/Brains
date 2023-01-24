use gloo_net::http::Request;
use shared::{
    BrainfartInformation, NotifyAboutMindExplosionRequest, NotifyAboutMindImplosionRequest, Uuid,
};
use web_sys::{HtmlDivElement, MouseEvent, WheelEvent};
use yew::{classes, html, html::onscroll::Event, Component, Html, NodeRef, Properties};

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
    NewExplosion(Uuid),
    NewImplosion(Uuid),
    UpdatedBrainfart(BrainfartInformation),
}

pub struct BrainfartsView {
    brainfarts: Vec<BrainfartInformation>,
    brainfarts_div: NodeRef,
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
                Message::None
            } else {
                let json = serde_json::from_str(&response_text);
                if let Ok(brainfarts) = json {
                    Message::Brainfart(brainfarts)
                } else {
                    if let Err(_) = json {
                        Message::None
                    } else {
                        Message::None
                    }
                }
            }
        });
    }
    fn send_new_implosion(ctx: &yew::Context<Self>, brainfart_id: Uuid) {
        ctx.link().send_future(async move {
            let resp = HelperService::add_authorization_header(Request::post(
                "/api/registermindimplosion",
            ))
            .json(&NotifyAboutMindImplosionRequest { brainfart_id })
            .unwrap()
            .send()
            .await
            .unwrap();

            let response_text = resp.text().await.unwrap();

            if !resp.ok() {
                Message::None
            } else {
                let json = serde_json::from_str(&response_text);
                if let Ok(brainfart) = json {
                    Message::UpdatedBrainfart(brainfart)
                } else {
                    if let Err(_) = json {
                        Message::None
                    } else {
                        Message::None
                    }
                }
            }
        });
    }

    fn send_new_explosion(ctx: &yew::Context<Self>, brainfart_id: Uuid) {
        ctx.link().send_future(async move {
            let resp = HelperService::add_authorization_header(Request::post(
                "/api/registermindexplosion",
            ))
            .json(&NotifyAboutMindExplosionRequest { brainfart_id })
            .unwrap()
            .send()
            .await
            .unwrap();

            let response_text = resp.text().await.unwrap();

            if !resp.ok() {
                Message::None
            } else {
                let json = serde_json::from_str(&response_text);
                if let Ok(brainfart) = json {
                    Message::UpdatedBrainfart(brainfart)
                } else {
                    if let Err(_) = json {
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
            brainfarts: vec![],
            brainfarts_div: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::None => false,
            Message::Brainfart(brainfarts) => {
                self.brainfarts = brainfarts;
                true
            }
            Message::NewExplosion(fartid) => {
                Self::send_new_explosion(ctx, fartid);
                false
            }
            Message::NewImplosion(fartid) => {
                Self::send_new_implosion(ctx, fartid);
                false
            }
            Message::UpdatedBrainfart(changed_fart) => {
                if let Some(index) = self
                    .brainfarts
                    .iter_mut()
                    .find(|item| item.id == changed_fart.id)
                {
                    index.blew_minds = changed_fart.blew_minds;
                    index.imploded_minds = changed_fart.imploded_minds;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let brainfart_div_ref = self.brainfarts_div.clone();
        let handle_scroll = ctx.link().callback(move |_: Event| {
            let div = brainfart_div_ref.cast::<HtmlDivElement>().unwrap();
            log::debug!("{}, {}", div.offset_height(), div.scroll_top());
            Message::None
        });
        let brainfart_div_ref = self.brainfarts_div.clone();
        let handle_wheel = ctx.link().callback(move |_: WheelEvent| {
            let w = web_sys::window().unwrap();
            log::debug!("{}, {}", w.page_y_offset().unwrap(), w.scroll_y().unwrap());
            Message::None
        });
        if let Some(_) = HelperService::get_jwt_information() {
            let brainfart = self
                .brainfarts
                .iter()
                .map(|brainfart| {
                    let brain_id = brainfart.id.clone();
                    let brain_id_two = brainfart.id.clone();
                    let on_explosion = ctx
                        .link()
                        .callback(move |_: MouseEvent| Message::NewExplosion(brain_id.clone()));
                    let on_implosion = ctx
                        .link()
                        .callback(move |_: MouseEvent| Message::NewImplosion(brain_id_two.clone()));
                    html! {<BrainfartComponent brainfart={brainfart.clone()} on_explosion={on_explosion} on_implosion={on_implosion} />}
                })
                .collect::<Html>();
            html! {
                <div>
                if self.brainfarts.len() > 0{
                    <div ref={self.brainfarts_div.clone()} class={classes!("flex","items-center","justify-between","flex-col")} onscroll={handle_scroll.clone()} onwheel={handle_wheel.clone()}>
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
