use yew::{classes, html, Component, Html, Properties};

use crate::HelperService;

use super::{nav_link_component::NavLinkComponent, user_info_component::UserInfoComponent};
use crate::Route;
use yew_router::prelude::Link;

#[derive(Properties, Clone, PartialEq)]
pub struct NavbarProps {}

pub enum Message {}

pub struct NavbarComponent {}

impl NavbarComponent {}

impl Component for NavbarComponent {
    type Message = Message;
    type Properties = NavbarProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        let _jwt = HelperService::get_jwt_information().unwrap();

        html! {<nav class="bg-white border-gray-200 shadow px-2 sm:px-4 py-2.5 rounded dark:bg-gray-900 mb-3">
        <div class="container flex flex-wrap items-center justify-between mx-auto">
          <Link<Route> to={Route::Home} classes={classes!("flex","items-center")}>
              <img src="assets/hjärnor.svg" class="h-6 mr-3 sm:h-9" alt="Hjärnor Logo" />
              <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">{"Hjärnor"}</span>
          </Link<Route>>
          <button data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-default" aria-expanded="false">
            <span class="sr-only">{"Open main menu"}</span>
            <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
          </button>
          <div class="hidden w-full md:block md:w-auto" id="navbar-default">
            <ul class="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
              <li>
                <NavLinkComponent to={Route::Home} text={"Home"}/>
              </li>
              <li>
                <NavLinkComponent to={Route::Logout} text={"Logout"}/>
              </li>
            </ul>
          </div>
          <UserInfoComponent/>
        </div>
        </nav>}
    }
}
