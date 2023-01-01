use yew::{html, Component, Html, Properties};

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
            <div class="spinner-border animate-spin inline-block w-8 h-8 border-4 rounded-full" role="status">
              <span class="visually-hidden">{"Loading..."}</span>
            </div>
        </div>
        }
    }
}
