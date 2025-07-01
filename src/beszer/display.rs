use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::collections::HashMap;


use crate::terv::AppContext;
use crate::shop::{Shopping, ShopDay, Shoppings};
use crate::backend::matrix::{Sub, Subs};
use crate::terv::display::TervProps;
use crate::backend::round::RoundD;


pub struct BeszerPage {
    pub error: Option<String>,
    pub current_beszer: Option<usize>,
}

pub enum BeszerMsg {
    Calculate,
    SearchBeszer(String),
}

impl Component for BeszerPage {
    type Message = BeszerMsg;
    type Properties = TervProps;

    fn create(ctx: &Context<Self>) -> Self {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();
        
        BeszerPage {
            error: terv.make_beszerek(),
            current_beszer: Some(0),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();

        self.error = terv.make_beszerek();
        if terv.beszerek.len() > 0 {
            self.current_beszer = Some(0);
        } else {
            self.current_beszer = None;
        }

        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();
        match msg {
            BeszerMsg::Calculate => {
                terv.make_beszerek();
                true
            },
            BeszerMsg::SearchBeszer(beszer_index) => {
                if let Ok(beszer_index) = beszer_index.parse() {
                    self.current_beszer = Some(beszer_index);
                }
                true
            },
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let app_data = link.context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();
        let shoppingdays = terv.shoppingdays.clone();

        let select_beszer = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            BeszerMsg::SearchBeszer(input.value())
        });

        html! {
            <div class="beszer">
                if let Some(error) = &self.error {
                    <p>{ format!("error: {}", error) }</p>
                } else if let None = self.current_beszer {
                    <p>{ "Nincs beszerlista" }</p>
                } else { 
                if let Some(beszer) = terv.beszerek.get(self.current_beszer.unwrap()) {
                    <p>{ "BeszerLista Selector" }</p>
                    <select onchange={select_beszer}>
                        { for terv.beszerek.iter().enumerate().map(|(index, beszer)| html! {
                            <option value={index.to_string()} selected={Some(index) == self.current_beszer}>
                                {format!("{}, {}", beszer.name, beszer.time)}
                            </option>
                        })}
                    </select>

                    //<button onclick={link.callback(move |_| BeszerMsg::Calculate)}>{ "Calculate" }</button>
                    <table>
                        <tr>
                            <th>{ beszer.name.clone() }</th>
                            <th>{ beszer.time.clone() }</th>
                            <th>{ beszer.recipes.clone() }</th>
                        </tr>
                        <tr>
                            <th>{ "név (egységár)" }</th>
                            <th>{ "recept (létszám)" }</th>
                            <th>{ "[részeredmény (/fő)], mennyiség (/fő) mértékegység" }</th>
                            <th>{ "[részeredmény (/fő)], ár (/fő)" }</th>
                        </tr>
                        { for beszer.items.iter().map(|item| {
                            html! {
                                <tr>
                                    <td>{ item.name.clone() }</td>
                                    <td>{ item.recipes.clone() }</td>
                                    <td>{ item.quantities.clone() }</td>
                                    <td>{ item.prices.clone() }</td>
                                </tr>
                            }
                        })}
                    </table>
                }}
                    // { for terv.matrix.iter().enumerate().map(|(index, (day, hash))| {
                    //     let subs_arr: Vec<&Subs> = hash.values().collect();
                    //     let mut names: Vec<&String> = subs_arr.iter().map(|subs| {
                    //         subs.iter().map(|sub| {
                    //             &sub.recipe
                    //         })
                    //     }).flatten().collect();
                    //     names.dedup();

                    //     let format_recipes: String = names.iter().enumerate().map(|(index, recipe)| {
                    //         format!("{}. {}", index + 1, recipe)
                    //     }).collect::<Vec<String>>().join(", ");

                    //     html! {
                    //         <table>
                    //             <tr>
                    //                 <th>{ shoppingdays.get_by_day(day).unwrap().name.clone() }</th>
                    //                 <th>{ day.to_string() }</th>
                    //                 <th>{ format_recipes.clone() }</th>
                    //             </tr>
                    //             <tr>
                    //                 <th>{ "Összetevő" }</th><th>{ "Recept (fő)" }</th><th>{ "[részeredmény], mennyiség, mértékegység" }</th>
                    //             </tr>
                    //             {for hash.iter().map(|(name, subs)| {
                    //                 html! {
                    //                     <tr>
                    //                         <td>{ name.clone() }</td>
                    //                         <td>{ subs.iter().map(|sub| {
                    //                             format!("{} ({})", names.iter().position(|x| **x == sub.recipe).unwrap() + 1, sub.number)
                    //                         }).collect::<Vec<String>>().join(", ") }</td>
                    //                         <td>{ format_quantities(subs, &terv.osszetevok.by_name(name).unwrap().unit) }</td>
                    //                     </tr>
                    //                 }
                    //             })
                    //             }
                            
                    //         </table>
                    //     }
                    // })}
                    //</div>
                
            </div>
        }
    }
}

pub fn format_quantities(subs: &Subs, unit: &str) -> String {
    let fquantity = |quantity: f64, number: u32| format!("{} ({})", quantity.roundd(2), (quantity / number as f64).roundd(2));
    let sum = fquantity(
        subs.iter().map(|sub| { sub.quantity }).sum::<f64>(),
        subs.iter().map(|sub| { sub.number }).sum::<u32>());
    if subs.len() > 1 {
         return format!("{}: {} {}", 
            subs.iter().map(|sub| { 
                fquantity(sub.quantity, sub.number)
            }).collect::<Vec<String>>().join(" + "), 
            sum,
            unit);
    } else {
        return format!("{} {}",
            sum,
            unit);
    }
}

pub fn format_quantities2(subs: &Subs, unit: &str) -> String {
    let sum: f64 = subs.iter().map(|sub| sub.quantity).sum();
    format!("{} {}", sum.roundd(3), unit)
}

pub fn format_prices(subs: &Subs) -> String {
    let fprice = |price: f64, number: u32| format!("{} ({})", price.roundd(2), (price / number as f64).roundd(2));
    let sum = fprice(
        subs.iter().map(|sub| { sub.price }).sum::<f64>(),
        subs.iter().map(|sub| { sub.number }).sum::<u32>());
    if subs.len() > 1 {
         return format!("{}: {}", 
            subs.iter().map(|sub| { 
                fprice(sub.price, sub.number)
            }).collect::<Vec<String>>().join(" + "), 
            sum);
    } else {
        return format!("{}",
            sum);
    }
}



pub fn round(value: f64, decimal: i32) -> f64 {
    let factor = 10f64.powi(decimal);
    (value * factor).round() / factor
}

#[test]
fn test_decimal_display() {
    let a: f64 = 1.2;
    let b: f64 = 1.23;
    let c: f64 = 1.234;
    let d: f64 = 1.2345;

    println!("{} {} {} {}", a, b, round(c, 2), d);
    
    
    //panic!();
}