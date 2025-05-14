
use yew::prelude::*;
use web_sys::HtmlInputElement;

//use crate::meal::display::{MealMsg, MealPage};
use crate::terv::TervContext;
use crate::terv::display::TervProps;

use super::*;


pub struct ShopPage {}

pub enum ShopMsg {
    Add,
    UpdateName(usize, String),
    UpdateShop(usize, String),
    Remove(usize),
}

impl Component for ShopPage {
    type Message = ShopMsg;
    type Properties = TervProps;

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