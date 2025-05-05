
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

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

enum Pages {
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
    current_page: Pages,
    terv: TervContext,
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
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        //let state = use_state(|| Terv::new());

        html! {
            <div class="root">
                <div>
                    <Socket />
                </div>
                <div class="menu">
                    <button onclick={link.callback(|_| AppMsg::Osszetevok)}>{ "Összetevők" }</button>
                    <button onclick={link.callback(|_| AppMsg::Recipes)}>{ "Receptek" }</button>
                    <button onclick={link.callback(|_| AppMsg::Meals)}>{ "Étkezések" }</button>
                    <button onclick={link.callback(|_| AppMsg::ShoppingDays)}>{ "Vásárnapok" }</button>
                    <button onclick={link.callback(|_| AppMsg::Beszer)}>{ "Beszerlisták" }</button>
                </div>
                <p>{ self.current_page.to_string() }</p>
                <div class="container">
                    <ContextProvider<TervContext> context={self.terv.clone()}>
                    {match self.current_page {
                        Pages::Osszetevok => {
                            html! {<OsszetevoPage />}
                        },
                        Pages::Recipes => {
                            html! {<RecipePage />}
                        },
                        Pages::Meals => {
                            html! {<MealPage />}
                        },
                        Pages::ShoppingDays => {
                            html! {<ShopPage />}
                        },
                        Pages::Beszer => {
                            html! {<BeszerPage />}
                        },
                        _ => {html! {<p>{ "Ismeretlen 3" }</p>}}
                    }}
                    </ContextProvider<TervContext>>
                </div>
            </div>
            
        }
    }
}