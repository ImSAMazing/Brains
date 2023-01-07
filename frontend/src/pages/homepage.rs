use gloo_net::http::Request;
use log::debug;
use shared::{Fantasiforster, FantasiforsterInformation};
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
    Fantasiforster(Vec<FantasiforsterInformation>),
}

pub struct HomePage {
    fantasiforster: Vec<FantasiforsterInformation>,
}

impl HomePage {
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

impl Component for HomePage {
    type Message = Message;
    type Properties = HomePageProps;
    fn create(ctx: &yew::Context<Self>) -> Self {
        HomePage::get_brainfarts(ctx);
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

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        if let Some(_) = HelperService::get_jwt_information() {
            let forster = self
                .fantasiforster
                .iter()
                .map(|forster| {
                    let dag = forster.födelsedag.format("%Y/%m/%d %H:%M").to_string();
                    html! {
                        <div key={forster.id.to_string()} class={classes!("block", "xl:w-2/5", "md:w-2/3", "sm:w-4/5", "xs:w-full", "border", "p-6", "border-gray-300", "rounded-lg", "shadow-md", "bg-gray-100")}>
                            <h1 class={classes!("mb-2", "font-bold", "tracking-tight", "text-center")}>
                            {&forster.titel}
                            </h1>
                            <p class={classes!("font-normal", "text-gray-700", "mb-3")}>
                            {&forster.innehåll}
                            </p>
                            <div class="flex justify-end">
                                <p class="inline-flex items-center px-3 py-2 text-sm font-medium text-center">
                                    <svg class="w-4 h-4 mr-1" viewBox="0 0 20 20">
                                        <path d="M10.25,2.375c-4.212,0-7.625,3.413-7.625,7.625s3.413,7.625,7.625,7.625s7.625-3.413,7.625-7.625S14.462,2.375,10.25,2.375M10.651,16.811v-0.403c0-0.221-0.181-0.401-0.401-0.401s-0.401,0.181-0.401,0.401v0.403c-3.443-0.201-6.208-2.966-6.409-6.409h0.404c0.22,0,0.401-0.181,0.401-0.401S4.063,9.599,3.843,9.599H3.439C3.64,6.155,6.405,3.391,9.849,3.19v0.403c0,0.22,0.181,0.401,0.401,0.401s0.401-0.181,0.401-0.401V3.19c3.443,0.201,6.208,2.965,6.409,6.409h-0.404c-0.22,0-0.4,0.181-0.4,0.401s0.181,0.401,0.4,0.401h0.404C16.859,13.845,14.095,16.609,10.651,16.811 M12.662,12.412c-0.156,0.156-0.409,0.159-0.568,0l-2.127-2.129C9.986,10.302,9.849,10.192,9.849,10V5.184c0-0.221,0.181-0.401,0.401-0.401s0.401,0.181,0.401,0.401v4.651l2.011,2.008C12.818,12.001,12.818,12.256,12.662,12.412"></path>
                                    </svg>
                                    {dag}
                                    <svg class="w-4 h-4 ml-2 mr-1" viewBox="0 0 20 20">
                                        <path d="M12.075,10.812c1.358-0.853,2.242-2.507,2.242-4.037c0-2.181-1.795-4.618-4.198-4.618S5.921,4.594,5.921,6.775c0,1.53,0.884,3.185,2.242,4.037c-3.222,0.865-5.6,3.807-5.6,7.298c0,0.23,0.189,0.42,0.42,0.42h14.273c0.23,0,0.42-0.189,0.42-0.42C17.676,14.619,15.297,11.677,12.075,10.812 M6.761,6.775c0-2.162,1.773-3.778,3.358-3.778s3.359,1.616,3.359,3.778c0,2.162-1.774,3.778-3.359,3.778S6.761,8.937,6.761,6.775 M3.415,17.69c0.218-3.51,3.142-6.297,6.704-6.297c3.562,0,6.486,2.787,6.705,6.297H3.415z"></path>
                                    </svg>
                                    {&forster.uppfinnare_namn}
                                </p>
                            </div>
                        </div>
                    }
                })
                .collect::<Html>();
            html! {
            <div>
                <NavbarComponent/>
                <div>
                if self.fantasiforster.len() > 0{
                    <div class="flex items-center">
                    {forster}
                    </div>
                }else{
                    <LoadingComponent/>
                }
                </div>
            </div> }
        } else {
            navigator.push(&Route::Login);
            html! {<LoadingComponent/>}
        }
    }
}
