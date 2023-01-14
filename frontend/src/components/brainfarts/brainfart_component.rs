use shared::BrainfartInformation;
use yew::{classes, html, Component, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct BrainfartProps {
    pub forster: BrainfartInformation,
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
        let forster = &ctx.props().forster;
        let dag = forster.birthdate.format("%Y/%m/%d %H:%M").to_string();
        html! {
            <div key={forster.id.to_string()} class={classes!("block", "xl:w-2/5", "md:w-2/3", "sm:w-4/5", "xs:w-full", "border", "border-gray-300", "rounded-lg", "shadow-md", "bg-gray-50", "mt-2")}>
                <div class={classes!("p-2", "border-b", "rounded-t", "dark:border-gray-600", "items-center", "justify-center")}>
                    <h1 class={classes!( "font-bold", "tracking-tight", "text-center")}>
                    {&forster.title}
                    </h1>
                </div>
                <div class={classes!("p-3", "space-y-3", "bg-gray-100")}>
                    <p class={classes!("font-normal", "text-gray-600",  "dark:text-gray-400", "text-base", "leading-relaxed")}>
                    {&forster.content}
                    </p>
                </div>
                <div class={classes!("flex","justify-end", "border-t", "border-gray-200", "rounded-b", "space-x-2")}>
                    <p class="inline-flex items-center px-3 py-2 text-sm font-medium text-center">
                        <svg class="w-4 h-4 mr-1" viewBox="0 0 20 20">
                            <path d="M10.25,2.375c-4.212,0-7.625,3.413-7.625,7.625s3.413,7.625,7.625,7.625s7.625-3.413,7.625-7.625S14.462,2.375,10.25,2.375M10.651,16.811v-0.403c0-0.221-0.181-0.401-0.401-0.401s-0.401,0.181-0.401,0.401v0.403c-3.443-0.201-6.208-2.966-6.409-6.409h0.404c0.22,0,0.401-0.181,0.401-0.401S4.063,9.599,3.843,9.599H3.439C3.64,6.155,6.405,3.391,9.849,3.19v0.403c0,0.22,0.181,0.401,0.401,0.401s0.401-0.181,0.401-0.401V3.19c3.443,0.201,6.208,2.965,6.409,6.409h-0.404c-0.22,0-0.4,0.181-0.4,0.401s0.181,0.401,0.4,0.401h0.404C16.859,13.845,14.095,16.609,10.651,16.811 M12.662,12.412c-0.156,0.156-0.409,0.159-0.568,0l-2.127-2.129C9.986,10.302,9.849,10.192,9.849,10V5.184c0-0.221,0.181-0.401,0.401-0.401s0.401,0.181,0.401,0.401v4.651l2.011,2.008C12.818,12.001,12.818,12.256,12.662,12.412"></path>
                        </svg>
                        {dag}
                        <svg class="w-4 h-4 ml-2 mr-1" viewBox="0 0 20 20">
                            <path d="M12.075,10.812c1.358-0.853,2.242-2.507,2.242-4.037c0-2.181-1.795-4.618-4.198-4.618S5.921,4.594,5.921,6.775c0,1.53,0.884,3.185,2.242,4.037c-3.222,0.865-5.6,3.807-5.6,7.298c0,0.23,0.189,0.42,0.42,0.42h14.273c0.23,0,0.42-0.189,0.42-0.42C17.676,14.619,15.297,11.677,12.075,10.812 M6.761,6.775c0-2.162,1.773-3.778,3.358-3.778s3.359,1.616,3.359,3.778c0,2.162-1.774,3.778-3.359,3.778S6.761,8.937,6.761,6.775 M3.415,17.69c0.218-3.51,3.142-6.297,6.704-6.297c3.562,0,6.486,2.787,6.705,6.297H3.415z"></path>
                        </svg>
                        {&forster.mastermind_name}
                    </p>
                </div>
            </div>
        }
    }
}
