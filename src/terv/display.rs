
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use gloo::console::log;

use crate::terv::{Terv, TervContext};
use crate::osszetevok::display::OsszetevoPage;
use crate::recipe::display::RecipePage;
use crate::meal::display::MealPage;
use crate::shop::display::ShopPage;
use crate::beszer::display::BeszerPage;
use crate::socket::Socket;

// #[derive(Clone, PartialEq)]
// pub struct TervState{
//     pub terv: Rc<UseStateHandle<Terv>>,
// }

// impl TervState {
//     pub fn new(state: UseStateHandle<Terv>) -> Self {
//         TervState {
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

pub struct TervPage {
    pub current_page: Pages,
    pub terv: TervContext,
    pub version: u64,
}

#[derive(Properties, PartialEq)]
pub struct TervProps {
    pub version: u64,
}

// Display

pub enum TervMsg {
    Osszetevok,
    Recipes,
    Meals,
    ShoppingDays,
    Beszer,
}

impl Component for TervPage {
    type Message = TervMsg;
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
            TervMsg::Osszetevok => {
                self.current_page = Pages::Osszetevok;
                true
            },
            TervMsg::Recipes => {
                self.current_page = Pages::Recipes;
                true
            },
            TervMsg::Meals => {
                self.current_page = Pages::Meals;
                true
            },
            TervMsg::ShoppingDays => {
                self.current_page = Pages::ShoppingDays;
                true
            },
            TervMsg::Beszer => {
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
                    <button onclick={link.callback(|_| TervMsg::Osszetevok)}>{ "Összetevők" }</button>
                    <button onclick={link.callback(|_| TervMsg::Recipes)}>{ "Receptek" }</button>
                    <button onclick={link.callback(|_| TervMsg::Meals)}>{ "Étkezések" }</button>
                    <button onclick={link.callback(|_| TervMsg::ShoppingDays)}>{ "Vásárnapok" }</button>
                    <button onclick={link.callback(|_| TervMsg::Beszer)}>{ "Beszerlisták" }</button>
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