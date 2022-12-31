use gloo_net::http::Request;
use shared::RegistreraHjärnaFörfrågan;
use web_sys::HtmlDivElement;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use yew::classes;
use yew::Callback;
use yew::Classes;
use yew::{html, Component, Html, InputEvent, MouseEvent, NodeRef, Properties, TargetCast};

use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::Link;

use crate::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct RegisterFormProps {
    pub register_explainer: String,
    pub on_succesfull_registration: Callback<String>,
}

struct AfterApiAction {
    should_redraw: bool,
    should_show_warning: bool,
}

pub enum Message {
    SetField,
    Submit,
    AfterApiResponse(AfterApiAction),
}

pub struct RegisterFormComponent {
    namn_ref: NodeRef,
    lösenord_ref: NodeRef,
    lösenord_extra_ref: NodeRef,
    error_holder_ref: NodeRef,
    error_text_ref: NodeRef,
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
        if RegistreraHjärnaFörfrågan::validera(&fields.0, &fields.1, &fields.2) {
            self.set_button_enabled()
        } else {
            self.set_button_disabled()
        }
    }

    fn set_button_enabled(&mut self) -> bool {
        if self.button_disabled {
            self.button_disabled = false;
            return true;
        }
        false
    }

    fn set_show_warning(&mut self) {
        self.show_warning = true;
    }
    fn set_button_disabled(&mut self) -> bool {
        if !self.button_disabled {
            self.button_disabled = true;
            return true;
        }
        false
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
            error_holder_ref: NodeRef::default(),
            error_text_ref: NodeRef::default(),
            button_disabled: true,
            show_warning: false,
        }
    }
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::SetField => self.update_button_status(),
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

                    let should_redraw = true;
                    let should_show_warning = !resp.ok();
                    if !resp.ok() {
                        error_text_element
                            .set_inner_text(&resp.text().await.unwrap().replace("\"", ""));
                    } else {
                        lösenord_element.set_value("");
                        lösenord_extra_element.set_value("");
                        on_succesfull_registration.emit(resp.text().await.unwrap());
                    }
                    Message::AfterApiResponse(AfterApiAction {
                        should_redraw,
                        should_show_warning,
                    })
                });
                false
            }
            Message::AfterApiResponse(action) => {
                self.update_button_status();
                if action.should_show_warning {
                    self.show_warning = true;
                } else {
                    self.show_warning = false;
                }
                action.should_redraw
            }
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
                <div ref={self.error_holder_ref.clone()} hidden={!self.show_warning.clone()} class="mt-2 bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4" role="alert">
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
                        <button disabled={self.button_disabled.clone()} onclick={on_click} class={self.get_classes()}>{"Register"}</button>
                        <Link<Route> to={Route::Login} classes={classes!("text-sm", "text-blue-600", "hover:underline")}>{"Already have an account?"}</Link<Route>>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
