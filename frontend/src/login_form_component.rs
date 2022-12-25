use email_address::EmailAddress;
use gloo_net::http::Request;
use shared::DemonstreraBesittarHjärnaFörfrågon;
use web_sys::HtmlButtonElement;
use web_sys::HtmlInputElement;
use yew::classes;
use yew::Classes;
use yew::{html, Component, Html, InputEvent, MouseEvent, NodeRef, Properties, TargetCast};

use crate::Login;
use wasm_bindgen_futures::spawn_local;
#[derive(Properties, PartialEq)]
pub struct LoginFormProps {
    pub login_explainer: String,
}

pub enum Message {
    SetEmail(String),
    SetPassword(String),
    Submit,
    DoNothing,
}

pub struct LoginFormComponent {
    email_ref: NodeRef,
    password_ref: NodeRef,
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

    fn update_button_status(&mut self, email: &String, password: &String) -> bool {
        if EmailAddress::is_valid(email) && !password.is_empty() {
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
            email_ref: NodeRef::default(),
            password_ref: NodeRef::default(),
            button_ref: NodeRef::default(),
            button_classes: LoginFormComponent::get_default_button_classes(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SetEmail(val) => {
                let password_element = self.password_ref.cast::<HtmlInputElement>().unwrap();
                let password = password_element.value();
                self.update_button_status(&val, &password)
            }
            Message::SetPassword(val) => {
                let email_element = self.email_ref.cast::<HtmlInputElement>().unwrap();
                let email = email_element.value();
                self.update_button_status(&email, &val)
            }
            Message::Submit => {
                self.set_button_disabled();
                let email_element = self.email_ref.cast::<HtmlInputElement>().unwrap();
                let password_element = self.password_ref.cast::<HtmlInputElement>().unwrap();
                let email = email_element.value();
                let password = password_element.value();
                log::debug!("Submitting form: {email}, {password}");
                spawn_local(async move {
                    let resp = Request::post("/api/loginasbrain")
                        .body(DemonstreraBesittarHjärnaFörfrågon::producera(
                            email, password,
                        ))
                        .send()
                        .await
                        .unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                });

                email_element.set_value("");
                password_element.set_value("");

                self.set_button_enabled();
                true
            }
            Message::DoNothing => false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let explainer = &ctx.props().login_explainer;

        let on_email_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            Message::SetEmail(input_el.value())
        });

        let on_password_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            Message::SetPassword(input_el.value())
        });

        let on_click = {
            let email_ref = self.email_ref.clone();
            let password_ref = self.password_ref.clone();
            ctx.link().callback(move |e: MouseEvent| {
                let email_element = email_ref.cast::<HtmlInputElement>().unwrap();
                let password_element = password_ref.cast::<HtmlInputElement>().unwrap();
                if email_element.value().is_empty() || password_element.value().is_empty() {
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

                <div class="mt-4">
                    <div>
                        <label class="block" for="email">{"Email"}</label>
                        <input ref={self.email_ref.clone()} type="text" placeholder={"Email"} oninput={on_email_input}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Password"}</label>
                        <input ref={self.password_ref.clone()} type="password" placeholder={"Password"} oninput={on_password_input}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="flex items-baseline justify-between">
                        <button ref={self.button_ref.clone()} onclick={on_click} class={self.button_classes.clone()}>{"Login"}</button>
                        <a href="#" class="text-sm text-blue-600 hover:underline">{"Don't have an account?"}</a>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
