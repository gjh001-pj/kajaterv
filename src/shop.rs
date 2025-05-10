use std::ops::{Deref, DerefMut};
use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::meal::{MealMsg, MealPage};
use crate::terv::TervContext;
use crate::display::AppProps;

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum ShopDay {
    Day(i32),
    Name(String),
}

impl ShopDay {
    pub fn from_str(day: &str) -> Self {
        if let Ok(day) = day.parse() {
            Self::Day(day)
        } else {
            Self::Name(day.to_string())
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ShopDay::Day(number) => number.to_string(),
            ShopDay::Name(name) => name.clone(),
        }
    }

    pub fn as_day(&self) -> &i32 {
        if let ShopDay::Day(day) = self {
            return day;
        } else {
            panic!("Not Day {:?}", self);
        }
    }

    pub fn as_mut_day(&mut self) -> &mut i32 {
        if let ShopDay::Day(day) = self {
            return day;
        } else {
            panic!("Not Day {:?}", self);
        }
    }

    pub fn as_name(&self) -> &String {
        if let ShopDay::Name(name) = self {
            return name;
        } else {
            panic!("Not Name {:?}", self);
        }
    }

    pub fn as_mut_name(&mut self) -> &mut String {
        if let ShopDay::Name(name) = self {
            return name;
        } else {
            panic!("Not Name {:?}", self);
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct Shopping {
    pub day: ShopDay,
    pub name: String,
}

impl Shopping {
    pub fn new() -> Self {
        Shopping {
            day: ShopDay::Name(String::from("")),
            name: String::from(""),
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
    UpdateName(usize, String),
    UpdateShop(usize, String),
    Remove(usize),
}

impl Component for ShopPage {
    type Message = ShopMsg;
    type Properties = AppProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ShopPage {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();
        match msg {
            ShopMsg::Add => {
                terv.shoppingdays.push(Shopping::new());
                true
            },
            ShopMsg::UpdateName(index, name) => {
                terv.shoppingdays.get_mut(index).unwrap().name = name;
                true
            },
            ShopMsg::UpdateShop(index, dayname) => {
                if let Ok(day) = dayname.parse() {
                    terv.shoppingdays.get_mut(index).unwrap().day = ShopDay::Day(day);
                } else {
                    terv.shoppingdays.get_mut(index).unwrap().day = ShopDay::Name(dayname);
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
                    <tr>
                        <th>{ "Name" }</th>
                        <th>{ "Day / Name" }</th>
                    </tr>
                    { for terv.shoppingdays.iter().enumerate().map(|(index, value)| {
                        let update_name = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            ShopMsg::UpdateName(index, input.value())
                        });

                        let update_shop = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            ShopMsg::UpdateShop(index, input.value())
                        });

                        html! {
                            <tr>
                                <th><input type="text" value={value.name.clone()} onchange={update_name}/></th>
                                <th><input type="text" value={value.day.to_string()} onchange={update_shop}/></th>
                                <th><button onclick={link.callback(move |_| ShopMsg::Remove(index))}>{ "Remove" }</button></th>
                            </tr>
                        }
                    })}
                </table>
            </div>
        }
    }
}