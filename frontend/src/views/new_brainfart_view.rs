use web_sys::MouseEvent;
use yew::{classes, html, Callback, Component, Html, Properties};

use crate::components::brainfarts::new_brainfart_component::NewBrainfartComponent;

#[derive(Properties, Clone, PartialEq)]
pub struct NewBrainfartViewProps {
    pub on_new_brainfart: Callback<String>,
}

pub enum Message {
    TriggerModal,
    CloseModal,
}

pub struct NewBrainfartView {
    show_modal: bool,
}

impl NewBrainfartView {}

impl Component for NewBrainfartView {
    type Message = Message;
    type Properties = NewBrainfartViewProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self { show_modal: false }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::TriggerModal => {
                self.show_modal = true;
                true
            }
            Message::CloseModal => {
                self.show_modal = false;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let on_click = ctx
            .link()
            .callback(move |_e: MouseEvent| Message::TriggerModal);
        let on_close = ctx
            .link()
            .callback(move |_e: MouseEvent| Message::CloseModal);
        let on_new_brainfart = ctx.props().clone().on_new_brainfart;

        let mut base_modal_classes = classes!(
            "fixed",
            "grid",
            "place-items-center",
            "z-50",
            "w-full",
            "p-4",
            "overflow-x-hidden",
            "overflow-y-auto",
            "md:inset-0",
            "h-modal",
            "w-modal",
            "md:h-full",
            "align-center",
            "justify-center"
        );
        if !self.show_modal {
            base_modal_classes.push("hidden");
        }
        let inner_modal_classes =
            classes!("relative", "w-full", "h-full", "max-w-2xl", "md:h-auto");
        html! {
            <div>
                <div class={classes!("fixed", "top-1/3", "left-1")}>
                    <button onclick={on_click}
                    class={classes!("block","text-white","bg-blue-700","hover:bg-blue-800","focus:ring-4","focus:outline-none","focus:ring-blue-300","font-medium","rounded-lg","text-sm","px-5","py-2.5","text-center")}
                    >{"Feel a fart?"}</button>
                </div>
                <div tabindex="-1" aria-hidden="true" class={base_modal_classes}>
                    <div class={inner_modal_classes}>
                        <NewBrainfartComponent on_creation={on_new_brainfart} on_close={on_close}/>
                    </div>
                </div>

            </div>
        }
    }
}
