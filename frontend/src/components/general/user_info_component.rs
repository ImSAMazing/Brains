use yew::{html, Component, Html, Properties};

use crate::HelperService;

#[derive(Properties, Clone, PartialEq)]
pub struct UserInfoProps {}

pub enum Message {}

pub struct UserInfoComponent {}

impl UserInfoComponent {}

impl Component for UserInfoComponent {
    type Message = Message;
    type Properties = UserInfoProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
        <div class="flex justify-center items-center">
            <h1>{format!("Welcome to Brains, {}",HelperService::get_jwt_information().unwrap().brainname)}</h1>
        </div>
        }
    }
}
