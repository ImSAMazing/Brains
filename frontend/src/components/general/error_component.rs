use yew::{html, Component, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct ErrorProps {}

pub enum Message {}

pub struct ErrorComponent {}

impl ErrorComponent {}

impl Component for ErrorComponent {
    type Message = Message;
    type Properties = ErrorProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <div><p>{"Error!"}</p></div>
        }
    }
}
