use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::collections::HashMap;

use crate::terv::TervContext;
use crate::shop::{Shopping, ShopDay, Shoppings};
use crate::backend::matrix::{Sub, Subs};
use crate::terv::display::TervProps;


pub struct BeszerPage {
    pub error: Option<String>,
}

pub enum BeszerMsg {
    Calculate,
}

impl Component for BeszerPage {
    type Message = BeszerMsg;
    type Properties = TervProps;

    fn create(ctx: &Context<Self>) -> Self {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();
        
        BeszerPage {
            error: terv.calculate_matrix(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();
        match msg {
            BeszerMsg::Calculate => {
                terv.calculate_matrix();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let terv = link.context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();
        let shoppingdays = terv.shoppingdays.clone();

        html! {
            <div class="beszer">
                if let Some(error) = &self.error {
                    <p>{ format!("error: {}", error) }</p>
                } else {
                    //<button onclick={link.callback(move |_| BeszerMsg::Calculate)}>{ "Calculate" }</button>
                    <div style="display: flex">
                    { for terv.matrix.iter().enumerate().map(|(index, (day, hash))| {
                        let subs_arr: Vec<&Subs> = hash.values().collect();
                        let mut names: Vec<&String> = subs_arr.iter().map(|subs| {
                            subs.iter().map(|sub| {
                                &sub.recipe
                            })
                        }).flatten().collect();
                        names.dedup();

                        let format_recipes: String = names.iter().enumerate().map(|(index, recipe)| {
                            format!("{}. {}", index + 1, recipe)
                        }).collect::<Vec<String>>().join(", ");

                        html! {
                            <table>
                                <tr>
                                    <th>{ shoppingdays.get_by_day(day).unwrap().name.clone() }</th>
                                    <th>{ day.to_string() }</th>
                                    <th>{ format_recipes.clone() }</th>
                                </tr>
                                <tr>
                                    <th>{ "Összetevő" }</th><th>{ "Recept (fő)" }</th><th>{ "[részeredmény], mennyiség, mértékegység" }</th>
                                </tr>
                                {for hash.iter().map(|(name, subs)| {
                                    html! {
                                        <tr>
                                            <td>{ name.clone() }</td>
                                            <td>{ subs.iter().map(|sub| {
                                                format!("{} ({})", names.iter().position(|x| **x == sub.recipe).unwrap() + 1, sub.number)
                                            }).collect::<Vec<String>>().join(", ") }</td>
                                            <td>{ formatted_quantities(name, subs, &terv.osszetevok.by_name(name).unwrap().unit) }</td>
                                        </tr>
                                    }
                                })
                                }
                            
                            </table>
                        }
                    })}
                    </div>
                }
            </div>
        }
    }
}

fn formatted_quantities(name: &str, subs: &Subs, unit: &str) -> String {
    let sum = subs.iter().map(|sub| { sub.quantity })
        .sum::<f64>().to_string();
    if subs.len() > 1 {
         return format!("{}: {} {}", 
            subs.iter().map(|sub| { sub.quantity.to_string() })
                .collect::<Vec<String>>().join(" "), 
            sum,
            unit);
    } else {
        return format!("{} {}",
            sum,
            unit);
    }
}