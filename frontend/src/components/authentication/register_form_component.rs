use gloo_net::http::Request;
use shared::RegistreraHjärnaFörfrågan;
use web_sys::HtmlInputElement;
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
    namn_ref: NodeRef,
    lösenord_ref: NodeRef,
    lösenord_extra_ref: NodeRef,
    error_text: String,
    button_disabled: bool,
    show_warning: bool,
}

impl RegisterFormComponent {
    fn get_classes(&self) -> Classes {
        if !self.button_disabled {
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

        let should_be_disabled =
            if RegistreraHjärnaFörfrågan::validera(&fields.0, &fields.1, &fields.2) {
                false
            } else {
                true
            };
        let will_value_change = should_be_disabled != self.button_disabled;
        self.button_disabled = should_be_disabled;
        log::debug!("{should_be_disabled}, {will_value_change}");
        will_value_change
    }

    fn get_input_fields_content(&self) -> (String, String, String) {
        let namn_element = self.namn_ref.cast::<HtmlInputElement>().unwrap();
        let namn = namn_element.value();
        let lösenord_element = self.lösenord_ref.cast::<HtmlInputElement>().unwrap();
        let lösenord = lösenord_element.value();
        let lösenord_extra_element = self.lösenord_extra_ref.cast::<HtmlInputElement>().unwrap();
        let lösenord_extra = lösenord_extra_element.value();
        (namn, lösenord, lösenord_extra)
    }
}

impl Component for RegisterFormComponent {
    type Message = Message;
    type Properties = RegisterFormProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            namn_ref: NodeRef::default(),
            lösenord_ref: NodeRef::default(),
            lösenord_extra_ref: NodeRef::default(),
            error_text: String::default(),
            button_disabled: true,
            show_warning: false,
        }
    }
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SetField => self.update_button_status(),
            Message::Submit => {
                self.button_disabled = true;
                self.show_warning = false;

                let namn_element = self.namn_ref.cast::<HtmlInputElement>().unwrap();
                let lösenord_element = self.lösenord_ref.cast::<HtmlInputElement>().unwrap();
                let lösenord_extra_element =
                    self.lösenord_extra_ref.cast::<HtmlInputElement>().unwrap();

                let namn = namn_element.value();
                let lösenord = lösenord_element.value();
                let lösenord_extra = lösenord_extra_element.value();

                let on_succesfull_registration = ctx.props().clone().on_succesfull_registration;
                ctx.link().send_future(async move {
                    let resp = Request::post("/api/registerbrain")
                        .json(&RegistreraHjärnaFörfrågan::producera(
                            namn,
                            lösenord,
                            lösenord_extra,
                        ))
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
                true
            }
            Message::DoNothing => true,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let explainer = &ctx.props().register_explainer;

        let on_input = ctx.link().callback(move |_e: InputEvent| Message::SetField);

        let on_click = { ctx.link().callback(move |_e: MouseEvent| Message::Submit) };
        html! {
        <div class="flex items-center justify-center min-h-screen bg-gray-100">
            <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg">
                <h3 class="text-2xl font-bold text-center">{explainer}</h3>
                <div hidden={!self.show_warning.clone()} class="mt-2 bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4" role="alert">
                    <p>{self.error_text.clone()}</p>
                </div>
                <div class="mt-4">
                    <div>
                        <label class="block">{"Namn"}</label>
                        <input ref={self.namn_ref.clone()} type="text" placeholder={"Namn"} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Lösenord"}</label>
                        <input ref={self.lösenord_ref.clone()} type="password" placeholder={"Lösenord"} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Lösenord Extra"}</label>
                        <input ref={self.lösenord_extra_ref.clone()} type="password" placeholder={"Lösenord"} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="flex items-baseline justify-between">
                        <button disabled={self.button_disabled.clone()} onclick={on_click} class={self.get_classes()}>{"Register"}</button>
                        <Link<Route> to={Route::Login} classes={classes!("text-sm", "text-blue-600", "hover:underline")}>{"Already have an account?"}</Link<Route>>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
