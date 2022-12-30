use std::error;

use gloo_net::http::Request;
use shared::DemonstreraBesittarHjärnaFörfrågon;
use shared::RegistreraHjärnaFörfrågan;
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
pub struct RegisterFormProps {
    pub register_explainer: String,
}

pub enum Message {
    SetField,
    Submit,
    DoNothing,
}

pub struct RegisterFormComponent {
    namn_ref: NodeRef,
    lösenord_ref: NodeRef,
    lösenord_extra_ref: NodeRef,
    error_holder_ref: NodeRef,
    error_text_ref: NodeRef,
    button_ref: NodeRef,
    button_classes: Classes,
}

impl RegisterFormComponent {
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

    fn update_button_status(
        &mut self,
        namn: &String,
        lösenord: &String,
        lösenord_extra: &String,
    ) -> bool {
        if RegistreraHjärnaFörfrågan::validera(namn, lösenord, lösenord_extra) {
            self.set_button_enabled()
        } else {
            self.set_button_disabled()
        }
    }

    fn set_button_enabled(&mut self) -> bool {
        let button_element = self.button_ref.cast::<HtmlButtonElement>().unwrap();
        if button_element.disabled() {
            button_element.set_disabled(false);
            self.button_classes = RegisterFormComponent::get_default_button_classes();
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

impl Component for RegisterFormComponent {
    type Message = Message;
    type Properties = RegisterFormProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            namn_ref: NodeRef::default(),
            lösenord_ref: NodeRef::default(),
            lösenord_extra_ref: NodeRef::default(),
            error_holder_ref: NodeRef::default(),
            error_text_ref: NodeRef::default(),
            button_ref: NodeRef::default(),
            button_classes: RegisterFormComponent::get_default_button_classes(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SetField => {
                let namn_element = self.namn_ref.cast::<HtmlInputElement>().unwrap();
                let namn = namn_element.value();
                let lösenord_element = self.lösenord_ref.cast::<HtmlInputElement>().unwrap();
                let lösenord = lösenord_element.value();
                let lösenord_extra_element =
                    self.lösenord_extra_ref.cast::<HtmlInputElement>().unwrap();
                let lösenord_extra = lösenord_extra_element.value();

                self.update_button_status(&namn, &lösenord, &lösenord_extra)
            }
            Message::Submit => {
                self.set_button_disabled();
                let namn_element = self.namn_ref.cast::<HtmlInputElement>().unwrap();
                let lösenord_element = self.lösenord_ref.cast::<HtmlInputElement>().unwrap();
                let lösenord_extra_element =
                    self.lösenord_extra_ref.cast::<HtmlInputElement>().unwrap();

                let error_holder_element = self.error_holder_ref.cast::<HtmlDivElement>().unwrap();
                error_holder_element.set_hidden(true);

                let error_text_element = self.error_text_ref.cast::<HtmlElement>().unwrap();

                let namn = namn_element.value();
                let lösenord = lösenord_element.value();
                let lösenord_extra = lösenord_extra_element.value();
                spawn_local(async move {
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

                    if !resp.ok() {
                        error_text_element
                            .set_inner_text(&resp.text().await.unwrap().replace("\"", ""));
                        error_holder_element.set_hidden(false);
                    } else {
                        log::debug!("{}", resp.text().await.unwrap());
                        lösenord_element.set_value("");
                        lösenord_extra_element.set_value("");
                    }
                });

                self.set_button_enabled();
                false
            }
            Message::DoNothing => false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let explainer = &ctx.props().register_explainer;

        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            Message::SetField
        });

        let on_click = { ctx.link().callback(move |e: MouseEvent| Message::Submit) };
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
                        <input ref={self.namn_ref.clone()} id="namn" type="text" placeholder={"Namn"} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Lösenord"}</label>
                        <input ref={self.lösenord_ref.clone()} type="lösenord" placeholder={"Lösenord"} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="mt-4">
                        <label class="block">{"Lösenord Extra"}</label>
                        <input ref={self.lösenord_extra_ref.clone()} type="lösenord" placeholder={"Lösenord"} oninput={on_input.clone()}
                            class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600"/>
                    </div>
                    <div class="flex items-baseline justify-between">
                        <button ref={self.button_ref.clone()} onclick={on_click} class={self.button_classes.clone()}>{"Register"}</button>
                        <a href="/login" class="text-sm text-blue-600 hover:underline">{"Already have an account?"}</a>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
