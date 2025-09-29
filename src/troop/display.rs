use yew::prelude::*;
use web_sys::{DataTransfer, HtmlInputElement};
use gloo::console::log;
use gloo::net::websocket::events::CloseEvent;
use web_sys::js_sys::JSON;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{MessageEvent, Window, ClipboardEvent};
use wasm_bindgen::closure::Closure;

use crate::backend::defwin::{handle_default_msg, DefWinMsg};
use crate::terv::AppContext;
use crate::backend::keyboard::TableFocusNavigator;
use crate::terv::display::TervProps;
use crate::backend::time::Time;
use crate::backend::paste::handle_paste;
use crate::win_comp_def;
use crate::input_with_defs;
use super::*;




pub struct TroopPage {
    //pub focus_nav: TableFocusNavigator,
    pub current_troop: Option<usize>,
}

// Display

pub enum TroopMsg {
    AddTroop,
    SearchTroop(String),
    RemoveTroop,
    UpdateName(String),
    UpdateSensitiveName(usize, String),
    UpdateSensitivity(usize, String),
    UpdateActions(usize, String),
    RemoveSensitive(usize),
    AddSensitive,

    DoNothing,
    DefWinBeh(DefWinMsg),
}

impl Component for TroopPage {
    type Message = TroopMsg;
    type Properties = TervProps;

    fn create(ctx: &Context<Self>) -> Self {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        Self { 
            current_troop: if terv.troops.len() > 0 { Some(0) } else { None },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        //let terv = use_context::<AppContext>().expect("Terv not found");
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();

        match msg {
            TroopMsg::AddTroop => {
                terv.troops.push(Troop::default());
                self.current_troop = Some(terv.troops.len() - 1);
                true
            },
            TroopMsg::SearchTroop(name) => {
                if let Some(index) = terv.troops.iter().enumerate()
                    .filter_map(|(index, troop)| 
                        if troop.name == name {Some(index)} else {None}).next() {
                    self.current_troop = Some(index);
                    true
                } else {
                    false
                }
            },
            TroopMsg::RemoveTroop => {
                if let Some(index) = self.current_troop {
                    terv.troops.swap_remove(index);
                    //terv.troops.sort_by(|a, b| a.name.cmp(&b.name));
                    true
                } else {
                    false
                }
            },
            TroopMsg::UpdateName(name) => {
                terv.troops[self.current_troop.unwrap()].name = name;
                true
            },
            TroopMsg::UpdateSensitiveName(index, name) => {
                terv.troops[self.current_troop.unwrap()].sensitives[index].name = name;
                true
            },
            TroopMsg::UpdateSensitivity(index, sensitivities) => {
                terv.troops[self.current_troop.unwrap()].sensitives[index]
                    .sensitivities = sensitivities.as_str().into();
                true
            },
            TroopMsg::UpdateActions(index, actions_str) => {
                terv.troops[self.current_troop.unwrap()].sensitives[index]
                    .actions = actions_str.as_str().into();
                true
            },
            TroopMsg::RemoveSensitive(index) => {
                terv.troops[self.current_troop.unwrap()].sensitives.remove(index);
                self.current_troop = if terv.troops.len() > 0 
                    { Some(0) } else { None };
                true
            },
            TroopMsg::AddSensitive => {
                terv.troops[self.current_troop.unwrap()].sensitives.push(Sensitive::default());
                true
            },
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        // let mut osszetevo_name_list: Vec<VNode> = Vec::new();
        // if let Some(recipe_index) = self.current_recipe {
        //     if let Some(recipe) = terv.recipes.get(recipe_index) {
        //         let mut all_osszetevo_name_list: Vec<&String> = terv.osszetevok.iter().map(|ossz| &ossz.name).collect();
        //         terv.recipes.iter().for_each(|recipe| {
        //             recipe.ingredients.iter().for_each(|ingredient| all_osszetevo_name_list.push(&ingredient.name))
        //         });
        //         all_osszetevo_name_list.sort();
        //         all_osszetevo_name_list.dedup();
        //         osszetevo_name_list = all_osszetevo_name_list.iter().map(|&rec_ossz| {
        //             if !recipe.ingredients.iter().map(|x| &x.name).collect::<Vec<&String>>().contains(&rec_ossz) {
        //                 html! {<option value={rec_ossz.clone()} />}
        //             } else {
        //                 html! {}
        //             }
        //         }).collect();
        //     }
        // }

        // let recipe_list: Vec<_> = terv.recipes.iter().map(|value| {
        //     let ec = value.get_errors(&terv).count();
        //     let wc = value.get_warnings(&terv).count();
        //     html! {<option value={value.name.clone()} >
        //         <p>{ value.name.clone() }</p>
        //         if ec > 0 {<p class="err">{ format!(" {}e", ec)}</p>}
        //         if wc > 0 {<p class="warn">{format!(" {}w", wc)}</p>}
        //         </option>}
        // }).collect();
        let troop_list = terv.troops.iter().map(|troop| {
            html!{<option>{ troop.name.clone() }</option>}
        }).collect::<Vec<_>>();
        
        html! {
            <div class="troops">
                <div class="toolbar">
                    <button onclick={link.callback(|_| TroopMsg::AddTroop)}>{ "Add troup" }</button>
                    //<label for="recipe_search">{ "Kerressen rá egy receptre" }</label>
                    <input type="text" list="troop_list" id="troop_search" oninput={
                        link.callback(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        TroopMsg::SearchTroop(input.value())})} />
                    <datalist id="troop_list">
                        { troop_list.clone() }
                    </datalist>
                    <select list="recipe_list" onchange={
                            link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            TroopMsg::SearchTroop(input.value())})} >
                            { troop_list }
                    </select>
                </div>
                <div class="current_troop">
                    if let Some(troop_index) = self.current_troop {
                        if let Some(troop) = terv.troops.get(troop_index) {
                            <p>{ format!("Kiválasztott örs: {}", troop.name) }</p>
                            <button onclick={link.callback(move |_| TroopMsg::RemoveTroop)}>{ "Remove Troop" }</button>
                            <table>
                                <tr>
                                    <th>{ "Örs:" }</th>
                                    <th><input type="text" value={troop.name.clone()} onchange={link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        TroopMsg::UpdateName(input.value())})} /></th>
                                </tr>
                                <tr>
                                    <th>{ "Név" }</th><th>{ "Érzékenységek" }</th><th>{ "Jelenlét" }</th>
                                </tr>
                                { for troop.sensitives.iter().enumerate().map(|(index, sensitive)| {
                                    let update_name = link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        TroopMsg::UpdateSensitiveName(index, input.value())
                                    });
        
                                    let update_sensitivity = link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        TroopMsg::UpdateSensitivity(index, input.value())
                                    });
        
                                    let update_actions = link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        TroopMsg::UpdateActions(index, input.value())
                                    });

                                    // let onkeydown = |col| link.callback(move |e: KeyboardEvent| {
                                    //     RecipeMsg::KeyPressed(index, col, e)
                                    // });
        
                                    // let onclick = link.callback(move |_| {
                                    //     RecipeMsg::MouseClick
                                    // });

                                    //let osszetevo = terv.osszetevok.by_name(&value.name);
        
                                    html! {
                                        <tr>
                                            <td><input type="text" value={sensitive.name.clone()} onchange={update_name}
                                                /*onkeydown={onkeydown(0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()}*/ /></td>
                                            <td><input type="text" value={sensitive.sensitivities.to_string()} onchange={update_sensitivity}
                                                /*onkeydown={onkeydown(1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()}*/ /></td>
                                            <td><input type="text" value={sensitive.actions.to_string()} onchange={update_actions}
                                                /*onkeydown={onkeydown(2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()}*/ /></td>
                                            //<td>{ format!("{} {}", round(value.quantity / recipe.number as f64, 3), value.unit) }</td>
                                            <td><button onclick={link.callback(move |_| TroopMsg::RemoveSensitive(index))}>{ "Remove" }</button></td>
                                            // {for value.get_errors(&terv).iter().map(|error| html!{
                                            //     <td class="err">{ match error {
                                            //         EW::Owned(text) => text.clone(),
                                            //         _ => "hiba a kódban".to_string(),
                                            //     } }</td>
                                            // })}
                                            
                                            // // {match osszetevo {
                                            // //     None => html!{<td class="err">{ "Az összetevő nem található" }</td>},
                                            // //     Some(osszetevo) => {
                                            // //         if let None = value.convert(&osszetevo.unit, &terv.convs) {
                                            // //             html!{<td class="err">{ format!("'{}' nem váltható át '{}'-ra/re", value.unit, osszetevo.unit) }</td>}
                                            // //         } else {html!{}}
                                            // //     },
                                            // // } }
                                            // if value.quantity == 0.0 {
                                            //     <td class="warn">{ "A mennyiség nulla" }</td>
                                            // }
                                            // if value.unit == "" {
                                            //     <td class="warn">{ "Nincs mértékegység" }</td>
                                            // }
                                        </tr>
                                    }
                                })}
                                <button onclick={link.callback(move |_| TroopMsg::AddSensitive)}>{ "Add Sensitive" }</button>
                            </table>
                        } else {
                            <p>{ "Nincs kiválasztott örs." }</p>
                        }
                    }
                </div>
            </div>
        }
    }
}