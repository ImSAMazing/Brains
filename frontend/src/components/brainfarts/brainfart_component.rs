use shared::BrainfartInformation;
use web_sys::MouseEvent;
use yew::{classes, html, Callback, Component, Html, Properties};

use crate::HelperService;

#[derive(Properties, Clone, PartialEq)]
pub struct BrainfartProps {
    pub brainfart: BrainfartInformation,
    pub on_explosion: Callback<MouseEvent>,
    pub on_implosion: Callback<MouseEvent>,
}

pub enum Message {}

pub struct BrainfartComponent {}

impl BrainfartComponent {}

impl Component for BrainfartComponent {
    type Message = Message;
    type Properties = BrainfartProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let brainfart = &ctx.props().brainfart;
        let dag = brainfart.birthdate.format("%Y/%m/%d %H:%M").to_string();
        let on_explosion = &ctx.props().on_explosion;
        let on_implosion = &ctx.props().on_implosion;

        let has_exploded = if let Some(_) = &brainfart
            .blew_minds
            .iter()
            .find(|item| item.get_id() == &HelperService::get_jwt_information().unwrap().id)
        {
            true
        } else {
            false
        };
        let has_imploded = !has_exploded
            && if let Some(_) = &brainfart
                .imploded_minds
                .iter()
                .find(|item| item.get_id() == &HelperService::get_jwt_information().unwrap().id)
            {
                true
            } else {
                false
            };

        html! {
            <div key={brainfart.id.to_string()} class={classes!("block", "xl:w-2/5", "md:w-2/3", "sm:w-4/5", "xs:w-full", "border", "border-gray-300", "rounded-lg", "shadow-md", "bg-gray-50", "mt-2")}>
                <div class={classes!("p-2", "border-b", "rounded-t", "dark:border-gray-600", "items-center", "justify-center")}>
                    <h1 class={classes!( "font-bold", "tracking-tight", "text-center")}>
                    {&brainfart.title}
                    </h1>
                </div>
                <div class={classes!("p-3", "space-y-3", "bg-gray-100")}>
                    <p class={classes!("font-normal", "text-gray-600",  "dark:text-gray-400", "text-base", "leading-relaxed")}>
                    {&brainfart.content}
                    </p>
                </div>
                <div class={classes!("flex","justify-between", "border-t", "border-gray-200", "rounded-b", "space-x-2")}>
                    <p class="inline-flex items-center px-3 py-2 text-sm font-medium text-center divide-x shadow-md">
                        if has_exploded{
                            <div class="inline-flex items-center text-center text-yellow-500 px-3">
                                <svg class="w-4 h-4 stroke-yellow-500" fill="none" stroke-width="1.5" viewBox="0 0 24 24">
                                    <path d="M12 18v-5.25m0 0a6.01 6.01 0 001.5-.189m-1.5.189a6.01 6.01 0 01-1.5-.189m3.75 7.478a12.06 12.06 0 01-4.5 0m3.75 2.383a14.406 14.406 0 01-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 10-7.517 0c.85.493 1.509 1.333 1.509 2.316V18" stroke-linecap="round" stroke-linejoin="round"></path>
                                </svg>
                                {&brainfart.blew_minds.len()}
                            </div>
                        }else{
                            <div onclick={on_explosion} class="inline-flex items-center text-center px-3">
                                <svg class="w-4 h-4" stroke="currentColor" fill="none" stroke-width="1.5" viewBox="0 0 24 24">
                                    <path d="M12 18v-5.25m0 0a6.01 6.01 0 001.5-.189m-1.5.189a6.01 6.01 0 01-1.5-.189m3.75 7.478a12.06 12.06 0 01-4.5 0m3.75 2.383a14.406 14.406 0 01-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 10-7.517 0c.85.493 1.509 1.333 1.509 2.316V18" stroke-linecap="round" stroke-linejoin="round"></path>
                                </svg>
                                {&brainfart.blew_minds.len()}
                            </div>
                        }
                        if has_imploded{
                            <div class="inline-flex items-center text-center text-red-500 px-3">
                                <svg class="w-4 h-4 stroke-red-500" fill="none" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M11.412 15.655L9.75 21.75l3.745-4.012M9.257 13.5H3.75l2.659-2.849m2.048-2.194L14.25 2.25 12 10.5h8.25l-4.707 5.043M8.457 8.457L3 3m5.457 5.457l7.086 7.086m0 0L21 21" stroke-linecap="round" stroke-linejoin="round"></path>
                                </svg>
                                {&brainfart.imploded_minds.len()}
                            </div>
                        }else{
                            <div onclick={on_implosion} class="inline-flex items-center text-center px-3">
                                <svg class="w-4 h-4 " fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M11.412 15.655L9.75 21.75l3.745-4.012M9.257 13.5H3.75l2.659-2.849m2.048-2.194L14.25 2.25 12 10.5h8.25l-4.707 5.043M8.457 8.457L3 3m5.457 5.457l7.086 7.086m0 0L21 21" stroke-linecap="round" stroke-linejoin="round"></path>
                                </svg>
                                {&brainfart.imploded_minds.len()}
                            </div>
                        }
                    </p>
                    <p class="inline-flex items-center px-3 py-2 text-sm font-medium text-center shadow-md">
                        <svg class="w-4 h-4 mr-1" viewBox="0 0 20 20">
                            <path d="M10.25,2.375c-4.212,0-7.625,3.413-7.625,7.625s3.413,7.625,7.625,7.625s7.625-3.413,7.625-7.625S14.462,2.375,10.25,2.375M10.651,16.811v-0.403c0-0.221-0.181-0.401-0.401-0.401s-0.401,0.181-0.401,0.401v0.403c-3.443-0.201-6.208-2.966-6.409-6.409h0.404c0.22,0,0.401-0.181,0.401-0.401S4.063,9.599,3.843,9.599H3.439C3.64,6.155,6.405,3.391,9.849,3.19v0.403c0,0.22,0.181,0.401,0.401,0.401s0.401-0.181,0.401-0.401V3.19c3.443,0.201,6.208,2.965,6.409,6.409h-0.404c-0.22,0-0.4,0.181-0.4,0.401s0.181,0.401,0.4,0.401h0.404C16.859,13.845,14.095,16.609,10.651,16.811 M12.662,12.412c-0.156,0.156-0.409,0.159-0.568,0l-2.127-2.129C9.986,10.302,9.849,10.192,9.849,10V5.184c0-0.221,0.181-0.401,0.401-0.401s0.401,0.181,0.401,0.401v4.651l2.011,2.008C12.818,12.001,12.818,12.256,12.662,12.412"></path>
                        </svg>
                        {dag}
                        <svg class="w-4 h-4 ml-2 mr-1" viewBox="0 0 20 20">
                            <path d="M12.075,10.812c1.358-0.853,2.242-2.507,2.242-4.037c0-2.181-1.795-4.618-4.198-4.618S5.921,4.594,5.921,6.775c0,1.53,0.884,3.185,2.242,4.037c-3.222,0.865-5.6,3.807-5.6,7.298c0,0.23,0.189,0.42,0.42,0.42h14.273c0.23,0,0.42-0.189,0.42-0.42C17.676,14.619,15.297,11.677,12.075,10.812 M6.761,6.775c0-2.162,1.773-3.778,3.358-3.778s3.359,1.616,3.359,3.778c0,2.162-1.774,3.778-3.359,3.778S6.761,8.937,6.761,6.775 M3.415,17.69c0.218-3.51,3.142-6.297,6.704-6.297c3.562,0,6.486,2.787,6.705,6.297H3.415z"></path>
                        </svg>
                        {&brainfart.mastermind_name}
                    </p>
                </div>
            </div>
        }
    }
}
