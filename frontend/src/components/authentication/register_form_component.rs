use gloo_net::http::Request;
use shared::RegisterBrainRequest;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use yew::classes;
use yew::Callback;
use yew::Classes;
use yew::{html, Component, Html, InputEvent, MouseEvent, NodeRef, Properties};

use yew_router::prelude::Link;

use crate::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct RegisterFormProps {
    pub register_explainer: String,
    pub on_succesfull_registration: Callback<String>,
}

pub struct AfterApiAction {
    error_text: String,
}

pub enum Message {
    SetField,
    Submit,
    AfterApiResponse(AfterApiAction),
    DoNothing,
}

pub struct RegisterFormComponent {
    name_ref: NodeRef,
    password_ref: NodeRef,
    password_extra_ref: NodeRef,
    error_text: String,
    button_disabled: bool,
    show_warning: bool,
    is_busy: bool,
}

impl RegisterFormComponent {
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

        let should_be_disabled = if RegisterBrainRequest::validate(&fields.0, &fields.1, &fields.2)
        {
            false
        } else {
            true
        };
        let will_value_change = should_be_disabled != self.button_disabled;
        self.button_disabled = should_be_disabled;
        will_value_change
    }

    fn get_input_fields_content(&self) -> (String, String, String) {
        let name_element = self.name_ref.cast::<HtmlInputElement>().unwrap();
        let name = name_element.value();
        let password_element = self.password_ref.cast::<HtmlInputElement>().unwrap();
        let password = password_element.value();
        let password_extra_element = self.password_extra_ref.cast::<HtmlInputElement>().unwrap();
        let password_extra = password_extra_element.value();
        (name, password, password_extra)
    }
}

impl Component for RegisterFormComponent {
    type Message = Message;
    type Properties = RegisterFormProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            name_ref: NodeRef::default(),
            password_ref: NodeRef::default(),
            password_extra_ref: NodeRef::default(),
            error_text: String::default(),
            button_disabled: true,
            show_warning: false,
            is_busy: false,
        }
    }
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SetField => self.update_button_status(),
            Message::Submit => {
                self.is_busy = true;
                self.show_warning = false;

                let fields = self.get_input_fields_content();
                let on_succesfull_registration = ctx.props().clone().on_succesfull_registration;
                ctx.link().send_future(async move {
                    let resp = Request::post("/api/registerbrain")
                        .json(&RegisterBrainRequest::create(fields.0, fields.1, fields.2))
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
                        on_succesfull_registration.emit(response_text);
                        Message::DoNothing
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
            Message::DoNothing => false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let explainer = &ctx.props().register_explainer;

        let on_input = ctx.link().callback(move |_e: InputEvent| Message::SetField);
        let button_is_enabled = self.button_is_enabled();
        let on_enter = ctx.link().callback(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                if button_is_enabled {
                    return Message::Submit;
                }
            }
            Message::DoNothing
        });

        let on_click = ctx.link().callback(move |_e: MouseEvent| Message::Submit);
        html! {
        <div class="flex items-center justify-center min-h-screen bg-gray-100">
            <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg">
                <h3 class="text-2xl font-bold text-center">{explainer}</h3>
                <div hidden={!self.show_warning.clone()} class="mt-2 bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4" role="alert">
                    <p>{self.error_text.clone()}</p>
                </div>
                <div class="mt-4">
                    <div>
                        <label class="block">{"Name"}</label>
                        <input ref={self.name_ref.clone()} type="text" placeholder={"Name"} onkeydown={on_enter.clone()} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Password"}</label>
                        <input ref={self.password_ref.clone()} type="password" placeholder={"Password"} onkeydown={on_enter.clone()} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Repeat password"}</label>
                        <input ref={self.password_extra_ref.clone()} type="password" placeholder={"Password"} onkeydown={on_enter.clone()} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="flex items-baseline justify-between">
                        <button disabled={!button_is_enabled} onclick={on_click} class={self.get_classes()}>{"Register"}</button>
                        <Link<Route> to={Route::Login} classes={classes!("text-sm", "text-blue-600", "hover:underline")}>{"Already have an account?"}</Link<Route>>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
