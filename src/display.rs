
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use gloo::console::log;

use crate::terv::{Terv, TervContext};
use crate::osszetevok::OsszetevoPage;
use crate::recipe::RecipePage;
use crate::meal::MealPage;
use crate::shop::ShopPage;
use crate::beszer::BeszerPage;
use crate::socket::Socket;

// #[derive(Clone, PartialEq)]
// pub struct AppState{
//     pub terv: Rc<UseStateHandle<Terv>>,
// }

// impl AppState {
//     pub fn new(state: UseStateHandle<Terv>) -> Self {
//         AppState {
//             terv: Rc::new(state),
//         }
//     }
// }

pub enum Pages {
    Osszetevok,
    Recipes,
    Meals,
    ShoppingDays,
    Beszer,
}

impl ToString for Pages {
    fn to_string(&self) -> String {
        match self {
            Pages::Osszetevok => String::from("Összetevők"),
            Pages::Recipes => String::from("Receptek"),
            Pages::Meals => String::from("Étkezések"),
            Pages::ShoppingDays => String::from("Vásárnapok"),
            Pages::Beszer => String::from("Beszerlisták"),
        }
    }
}

pub struct App {
    pub current_page: Pages,
    pub terv: TervContext,
    pub version: u64,
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub version: u64,
}

// Display

pub enum AppMsg {
    Osszetevok,
    Recipes,
    Meals,
    ShoppingDays,
    Beszer,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_page: Pages::Osszetevok,
            terv: Rc::new(RefCell::new(Terv::new())),
            version: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Osszetevok => {
                self.current_page = Pages::Osszetevok;
                true
            },
            AppMsg::Recipes => {
                self.current_page = Pages::Recipes;
                true
            },
            AppMsg::Meals => {
                self.current_page = Pages::Meals;
                true
            },
            AppMsg::ShoppingDays => {
                self.current_page = Pages::ShoppingDays;
                true
            },
            AppMsg::Beszer => {
                self.current_page = Pages::Beszer;
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        //let state = use_state(|| Terv::new());
        let terv_context = Rc::clone(&self.terv);
        let terv = self.terv.borrow();
        

        html! {
            <div class="root">
                <p>{ "localhost test" }</p>
                <div class="menu">
                    <button onclick={link.callback(|_| AppMsg::Osszetevok)}>{ "Összetevők" }</button>
                    <button onclick={link.callback(|_| AppMsg::Recipes)}>{ "Receptek" }</button>
                    <button onclick={link.callback(|_| AppMsg::Meals)}>{ "Étkezések" }</button>
                    <button onclick={link.callback(|_| AppMsg::ShoppingDays)}>{ "Vásárnapok" }</button>
                    <button onclick={link.callback(|_| AppMsg::Beszer)}>{ "Beszerlisták" }</button>
                </div>
                <p>{ self.current_page.to_string() }</p>
                <div class="container">
                    <ContextProvider<TervContext> context={terv_context}>
                    <div>
                        <Socket />
                    </div>
                    {match self.current_page {
                        Pages::Osszetevok => {
                            html! {<OsszetevoPage version={terv.version} />}
                        },
                        Pages::Recipes => {
                            html! {<RecipePage version={terv.version} />}
                        },
                        Pages::Meals => {
                            html! {<MealPage version={terv.version} />}
                        },
                        Pages::ShoppingDays => {
                            html! {<ShopPage version={terv.version} />}
                        },
                        Pages::Beszer => {
                            html! {<BeszerPage version={terv.version} />}
                        },
                        _ => {html! {<p>{ "Ismeretlen 3" }</p>}}
                    }}
                    </ContextProvider<TervContext>>
                </div>
            </div>
            
        }
    }
}