use std::error;

use gloo_net::http::Request;
use shared::DemonstreraBesittarHjärnaFörfrågon;
use web_sys::HtmlButtonElement;
use web_sys::HtmlDivElement;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::Node;
use yew::classes;
use yew::Classes;
use yew::{html, Component, Html, InputEvent, MouseEvent, NodeRef, Properties, TargetCast};

use wasm_bindgen_futures::spawn_local;
#[derive(Properties, PartialEq)]
pub struct LoginFormProps {
    pub login_explainer: String,
}

pub enum Message {
    SetNamn(String),
    SetPassword(String),
    Submit,
    DoNothing,
}

pub struct LoginFormComponent {
    namn_ref: NodeRef,
    password_ref: NodeRef,
    error_holder_ref: NodeRef,
    error_text_ref: NodeRef,
    button_ref: NodeRef,
    button_classes: Classes,
}

impl LoginFormComponent {
    fn get_default_button_classes() -> Classes {
        classes!(
            "px-6",
            "py-2",
            "mt-4",
            "text-white",
            "bg-blue-600",
            "rounded-lg",
            "hover:bg-blue-900"
        )
    }

    fn update_button_status(&mut self, namn: &String, password: &String) -> bool {
        if !namn.is_empty() && !password.is_empty() {
            self.set_button_enabled()
        } else {
            self.set_button_disabled()
        }
    }

    fn set_button_enabled(&mut self) -> bool {
        let button_element = self.button_ref.cast::<HtmlButtonElement>().unwrap();
        if button_element.disabled() {
            button_element.set_disabled(false);
            self.button_classes = LoginFormComponent::get_default_button_classes();
            return true;
        }
        false
    }
    fn set_button_disabled(&mut self) -> bool {
        let button_element = self.button_ref.cast::<HtmlButtonElement>().unwrap();
        if !button_element.disabled() {
            button_element.set_disabled(true);
            self.button_classes = classes!(
                "px-6",
                "py-2",
                "mt-4",
                "text-white",
                "bg-gray-300",
                "rounded-lg",
                "hover:bg-gray-900"
            );
            return true;
        }
        false
    }
}

impl Component for LoginFormComponent {
    type Message = Message;
    type Properties = LoginFormProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            namn_ref: NodeRef::default(),
            password_ref: NodeRef::default(),
            error_holder_ref: NodeRef::default(),
            error_text_ref: NodeRef::default(),
            button_ref: NodeRef::default(),
            button_classes: LoginFormComponent::get_default_button_classes(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SetNamn(val) => {
                let password_element = self.password_ref.cast::<HtmlInputElement>().unwrap();
                let password = password_element.value();
                self.update_button_status(&val, &password)
            }
            Message::SetPassword(val) => {
                let namn_element = self.namn_ref.cast::<HtmlInputElement>().unwrap();
                let namn = namn_element.value();
                self.update_button_status(&namn, &val)
            }
            Message::Submit => {
                self.set_button_disabled();
                let namn_element = self.namn_ref.cast::<HtmlInputElement>().unwrap();
                let password_element = self.password_ref.cast::<HtmlInputElement>().unwrap();

                let error_holder_element = self.error_holder_ref.cast::<HtmlDivElement>().unwrap();
                error_holder_element.set_hidden(true);

                let error_text_element = self.error_text_ref.cast::<HtmlElement>().unwrap();

                let namn = namn_element.value();
                let password = password_element.value();
                log::debug!("Submitting form: {namn}, {password}");
                spawn_local(async move {
                    let resp = Request::post("/api/loginasbrain")
                        .json(&DemonstreraBesittarHjärnaFörfrågon::producera(
                            namn, password,
                        ))
                        .unwrap()
                        .send()
                        .await
                        .unwrap();

                    if !resp.ok() {
                        error_text_element
                            .set_inner_text(&resp.text().await.unwrap().replace("\"", ""));
                        error_holder_element.set_hidden(false);
                    } else {
                        log::debug!("{}", resp.text().await.unwrap());
                    }

                    password_element.set_value("");
                });

                self.set_button_enabled();
                false
            }
            Message::DoNothing => false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let explainer = &ctx.props().login_explainer;

        let on_namn_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            Message::SetNamn(input_el.value())
        });

        let on_password_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            Message::SetPassword(input_el.value())
        });

        let on_click = {
            let namn_ref = self.namn_ref.clone();
            let password_ref = self.password_ref.clone();
            ctx.link().callback(move |e: MouseEvent| {
                let namn_element = namn_ref.cast::<HtmlInputElement>().unwrap();
                let password_element = password_ref.cast::<HtmlInputElement>().unwrap();
                if namn_element.value().is_empty() || password_element.value().is_empty() {
                    log::debug!("Doing nothing");
                    Message::DoNothing //Dit triggert eigenlijk nooit behalve bij initiale creatie want button is disabled... weghalen misschien? Todo
                } else {
                    Message::Submit
                }
            })
        };
        html! {
        <div class="flex items-center justify-center min-h-screen bg-gray-100">
            <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg">
                <h3 class="text-2xl font-bold text-center">{explainer}</h3>
                <div ref={self.error_holder_ref.clone()} class="mt-2 bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4" role="alert">
                    <p ref={self.error_text_ref.clone()}>{"Something not ideal might be happening."}</p>
                </div>
                <div class="mt-4">
                    <div>
                        <label class="block" for="namn">{"Namn"}</label>
                        <input ref={self.namn_ref.clone()} id="namn" type="text" placeholder={"Namn"} oninput={on_namn_input}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Password"}</label>
                        <input ref={self.password_ref.clone()} type="password" placeholder={"Password"} oninput={on_password_input}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="flex items-baseline justify-between">
                        <button ref={self.button_ref.clone()} onclick={on_click} class={self.button_classes.clone()}>{"Login"}</button>
                        <a href="/register" class="text-sm text-blue-600 hover:underline">{"Don't have an account?"}</a>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
