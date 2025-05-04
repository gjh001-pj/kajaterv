use std::ops::{Deref, DerefMut};
use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::meal::{MealMsg, MealPage};
use crate::terv::TervContext;

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Shopping {
    Day(i32),
    Name(String),
}

impl Shopping {
    pub fn to_string(&self) -> String {
        match self {
            Shopping::Day(number) => number.to_string(),
            Shopping::Name(name) => name.clone(),
        }
    }

    pub fn as_day(&self) -> &i32 {
        if let Shopping::Day(day) = self {
            return day;
        } else {
            panic!("Not Day {:?}", self);
        }
    }

    pub fn as_mut_day(&mut self) -> &mut i32 {
        if let Shopping::Day(day) = self {
            return day;
        } else {
            panic!("Not Day {:?}", self);
        }
    }

    pub fn as_name(&self) -> &String {
        if let Shopping::Name(name) = self {
            return name;
        } else {
            panic!("Not Name {:?}", self);
        }
    }

    pub fn as_mut_name(&mut self) -> &mut String {
        if let Shopping::Name(name) = self {
            return name;
        } else {
            panic!("Not Name {:?}", self);
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct Shoppings(pub Vec<Shopping>);

impl Shoppings {
    pub fn new() -> Self {
        Shoppings (Vec::new())
    }
}

impl Deref for Shoppings {
    type Target = Vec<Shopping>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Shoppings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct ShopPage {}

pub enum ShopMsg {
    Add,
    UpdateShop(usize, String),
    Remove(usize),
}

impl Component for ShopPage {
    type Message = ShopMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ShopPage {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();
        match msg {
            ShopMsg::Add => {
                terv.shoppingdays.push(Shopping::Name(String::from("")));
                true
            },
            ShopMsg::UpdateShop(index, dayname) => {
                if let Ok(day) = dayname.parse() {
                    *terv.shoppingdays.get_mut(index).unwrap() = Shopping::Day(day);
                } else {
                    *terv.shoppingdays.get_mut(index).unwrap() = Shopping::Name(dayname);
                }
                true
            },
            ShopMsg::Remove(index) => {
                terv.shoppingdays.remove(index);
                true
            },

        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let terv = link.context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();

        html! {
            <div class="shop" >
                <button onclick={link.callback(move |_| ShopMsg::Add)}>{ "Add" }</button>
                <table>
                    <tr><th>{ "Day / Name" }</th></tr>
                    { for terv.shoppingdays.iter().enumerate().map(|(index, value)| {
                        let update_shop = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            ShopMsg::UpdateShop(index, input.value())
                        });

                        html! {
                            <tr>
                                <th><input type="text" value={value.to_string()} onchange={update_shop}/></th>
                                <th><button onclick={link.callback(move |_| ShopMsg::Remove(index))}>{ "Remove" }</button></th>
                            </tr>
                        }
                    })}
                </table>
            </div>
        }
    }
}