
use yew::prelude::*;
use crate::terv::Terv;
use crate::osszetevok::OsszetevoPage;
use crate::recipe::RecipePage;

pub struct App {
    terv: Terv,
    current_page: &'static str,
}

impl App {
    pub fn new() -> Self {
        App {
            terv: Terv::new(),
            current_page: "",
        }
    }
}

// Display

pub enum AppMsg {
    Osszetevok,
    Recipes,
    Meals,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App::new()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Osszetevok => {
                self.current_page = "Összetevők";
                true
            },
            AppMsg::Recipes => {
                self.current_page = "Receptek";
                true
            },
            AppMsg::Meals => {
                self.current_page = "Étkezések";
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let is_visible = |page: &str| -> &'static str {
            if self.current_page == page { "display: block;" } else { "display: none;" }
        };

        html! {
            <div class="root">
                <div class="menu">
                    <button onclick={link.callback(|_| AppMsg::Osszetevok)}>{ "Összetevők" }</button>
                    <button onclick={link.callback(|_| AppMsg::Recipes)}>{ "Receptek" }</button>
                    <button onclick={link.callback(|_| AppMsg::Meals)}>{ "Étkezések" }</button>
                </div>
                <p>{ self.current_page }</p>
                <div class="container">
                    //<div style={is_visible("Összetevők")}><OsszetevoPage /></div>
                    
                    {match self.current_page {
                        "Összetevők" => {
                            html! {<OsszetevoPage />}
                        },
                        "Receptek" => {
                            html! {<RecipePage />}
                            //html! {<p>{ "Receptek" }</p>}
                        },
                        "Étkezések" => {
                            //html! {<Meal />}
                            html! {<p>{ "Étkezések" }</p>}
                        },
                        _ => {html! {<p>{ "Ismeretlen" }</p>}}
                    }}
                </div>
            </div>
            
        }
    }
}