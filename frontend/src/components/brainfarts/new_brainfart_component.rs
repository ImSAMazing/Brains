use gloo_net::http::Request;
use shared::CreateBrainfartRequest;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{classes, html, Callback, Classes, Component, Html, NodeRef, Properties};

use crate::HelperService;

#[derive(Properties, Clone, PartialEq)]
pub struct NewBrainfartProps {
    pub on_creation: Callback<String>,
    pub on_close: Callback<MouseEvent>,
    pub on_mouse_enters_content_area: Callback<MouseEvent>,
    pub on_mouse_leaves_content_area: Callback<MouseEvent>,
}
pub struct AfterApiAction {
    error_text: String,
}
pub enum Message {
    Submit,
    AfterCreation,
    SetField,
    AfterApiResponse(AfterApiAction),
}

pub struct NewBrainfartComponent {
    title_ref: NodeRef,
    innehal_ref: NodeRef,
    show_warning: bool,
    error_text: String,
    button_disabled: bool,
    is_busy: bool,
}

impl NewBrainfartComponent {
    fn button_is_enabled(&self) -> bool {
        return !self.button_disabled && !self.is_busy;
    }
    fn get_classes(&self) -> Classes {
        if self.button_is_enabled() {
            classes!(
                "px-6",
                "py-2",
                "mt-4",
                "text-white",
                "bg-blue-600",
                "rounded-lg",
                "hover:bg-blue-900"
            )
        } else {
            classes!(
                "px-6",
                "py-2",
                "mt-4",
                "text-white",
                "bg-gray-300",
                "rounded-lg",
            )
        }
    }

    fn update_button_status(&mut self) -> bool {
        let fields = self.get_input_fields_content();

        let should_be_disabled = if CreateBrainfartRequest::validate(&fields.0, &fields.1) {
            false
        } else {
            true
        };
        let will_value_change = should_be_disabled != self.button_disabled;
        self.button_disabled = should_be_disabled;
        will_value_change
    }

    fn get_input_fields_content(&self) -> (String, String) {
        let title_element = self.title_ref.cast::<HtmlInputElement>().unwrap();
        let title = title_element.value();
        let innehal_element = self.innehal_ref.cast::<HtmlInputElement>().unwrap();
        let innehal = innehal_element.value();
        (title, innehal)
    }
}

impl Component for NewBrainfartComponent {
    type Message = Message;
    type Properties = NewBrainfartProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            title_ref: NodeRef::default(),
            innehal_ref: NodeRef::default(),
            button_disabled: true,
            show_warning: false,
            error_text: String::default(),
            is_busy: false,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SetField => self.update_button_status(),
            Message::AfterCreation => {
                self.is_busy = false;
                self.show_warning = false;
                self.title_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");
                self.innehal_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");
                true
            }
            Message::Submit => {
                self.is_busy = true;
                self.show_warning = false;

                let fields = self.get_input_fields_content();
                let on_creation = ctx.props().clone().on_creation;

                ctx.link().send_future(async move {
                    let resp = HelperService::add_authorization_header(Request::post(
                        "/api/createbrainfart",
                    ))
                    .json(&CreateBrainfartRequest::create(fields.0, fields.1))
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                    let response_text = resp.text().await.unwrap().replace("\"", "");

                    if !resp.ok() {
                        Message::AfterApiResponse(AfterApiAction {
                            error_text: response_text,
                        })
                    } else {
                        on_creation.emit(response_text);
                        Message::AfterCreation
                    }
                });
                true
            }
            Message::AfterApiResponse(action) => {
                self.update_button_status();
                self.show_warning = true;

                self.error_text = action.error_text;
                self.is_busy = false;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let button_is_enabled = self.button_is_enabled();
        let on_click = ctx.link().callback(move |_e: MouseEvent| Message::Submit);
        let on_input = ctx.link().callback(move |_e: InputEvent| Message::SetField);
        let on_close = ctx.props().clone().on_close;
        let on_mouse_enters_content_area = ctx.props().clone().on_mouse_enters_content_area;
        let on_mouse_leaves_content_area = ctx.props().clone().on_mouse_leaves_content_area;
        let title_classes = classes!(
            "w-full",
            "px-4",
            "py-2",
            "mt-2",
            "border",
            "rounded-md",
            "focus:outline-none",
            "focus:ring-1",
            "focus:ring-blue-600"
        );
        html! {
            <div class={classes!("block", "border", "w-full", "p-6", "border-gray-300", "rounded-lg", "shadow-md", "bg-gray-400")} onmouseleave={on_mouse_leaves_content_area} onmouseenter={on_mouse_enters_content_area} >
                <button onclick={on_close} class={classes!("float-right","block","text-white","bg-blue-700","hover:bg-blue-800","focus:ring-4","focus:outline-none","focus:ring-blue-300","font-medium","rounded-lg","text-sm","px-5","py-2.5","text-center")}>{"Cancel"}</button>
                <h1 class={classes!("text-xl","mb-2", "font-bold", "tracking-tight", "text-center")}>
                {"New brainfart?"}
                </h1>
                <div>
                    <label class={classes!("block")} for="title">{"Title"}</label>
                    <input ref={self.title_ref.clone()} id="title" type="text" placeholder={"title"} oninput={on_input.clone()}
                    class={title_classes.clone()}/>
                </div>
                <div>
                    <label class={classes!("block")} for="innehal">{"Content"}</label>
                    <textarea ref={self.innehal_ref.clone()} id="innehal" oninput={on_input.clone()}
                    class={title_classes}>
                    </textarea>
                </div>
                <div class="flex justify-end">
                    <button disabled={!button_is_enabled} onclick={on_click} class={self.get_classes()}>{"Fart your brainwaves"}</button>
                </div>
            </div>
        }
    }
}
