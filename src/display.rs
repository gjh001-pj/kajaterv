
use yew::prelude::*;
use crate::terv::Terv;
use std::rc::Rc;
use crate::osszetevok::OsszetevoPage;
use crate::recipe::RecipePage;

#[derive(Clone, PartialEq)]
pub struct AppState{
    pub terv: Rc<UseStateHandle<Terv>>,
}

impl AppState {
    pub fn new(state: UseStateHandle<Terv>) -> Self {
        AppState {
            terv: Rc::new(state),
        }
    }
}

pub struct App {
    current_page: &'static str,
}

impl App {
    pub fn new() -> Self {
        App {
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
        let state = use_state(|| Terv::new());

        html! {
            <div class="root">
                <div class="menu">
                    <button onclick={link.callback(|_| AppMsg::Osszetevok)}>{ "Összetevők" }</button>
                    <button onclick={link.callback(|_| AppMsg::Recipes)}>{ "Receptek" }</button>
                    <button onclick={link.callback(|_| AppMsg::Meals)}>{ "Étkezések" }</button>
                </div>
                <p>{ self.current_page }</p>
                // <div class="container">
                //     <ContextProvider<AppState> context={AppState::new(state)}>
                //     {match self.current_page {
                //         "Összetevők" => {
                //             html! {<OsszetevoPage />}
                //         },
                //         "Receptek" => {
                //             html! {<RecipePage />}
                //             //html! {<p>{ "Receptek" }</p>}
                //         },
                //         "Étkezések" => {
                //             //html! {<Meal />}
                //             html! {<p>{ "Étkezések" }</p>}
                //         },
                //         _ => {html! {<p>{ "Ismeretlen" }</p>}}
                //     }}
                //     </ContextProvider<AppState>>
                // </div>
            </div>
            
        }
    }
}