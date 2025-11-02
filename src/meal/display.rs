use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo::console::log;

use crate::backend::keyboard::TableFocusNavigator;
use crate::terv::AppContext;
use crate::terv::display::TervProps;
use crate::backend::paste::handle_paste;
use crate::ew::{EW, GetEWs};

use super::*;



pub struct MealPage {
    pub focus_nav: TableFocusNavigator,
}

pub enum MealMsg {
    AddMeal,
    UpdateRecipe(usize, String),
    UpdateNumber(usize, String),
    UpdateDay(usize, String),

    RemoveMeal(usize),
    KeyPressed(usize, usize, KeyboardEvent),
    MouseClick,
    HandlePaste(String, usize, usize),
    DoNothing,
}

impl Component for MealPage {
    type Message = MealMsg;
    type Properties = TervProps;

    fn create(ctx: &Context<Self>) -> Self {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        MealPage {
            focus_nav: TableFocusNavigator::new(terv.meals.len(), 3),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();
        self.focus_nav.build(terv.meals.len(), 3);
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();

        match msg {
            MealMsg::AddMeal => {
                terv.meals.push(Meal::default());
                self.focus_nav.build(terv.meals.len(), 3);
                true
            },
            MealMsg::UpdateRecipe(index, recipe) => {
                terv.meals.get_mut(index).unwrap().as_common_mut().main_recipe = recipe;
                true
            },
            MealMsg::UpdateNumber(index, number) => {
                if let Ok(number) = number.parse() {
                    terv.meals.get_mut(index).unwrap().as_common_mut().number = number;
                }
                true
            },
            MealMsg::UpdateDay(index, day) => {
                if let Ok(day) = day.parse() {
                    terv.meals.get_mut(index).unwrap().day = ShopDay::Day(day);
                } else {
                    terv.meals.get_mut(index).unwrap().day = ShopDay::Name(day);
                }
                true
            },
            MealMsg::RemoveMeal(index) => {
                terv.meals.remove(index);
                self.focus_nav.build(terv.meals.len(), 3);
                true
            }
            MealMsg::KeyPressed(row, col, e) => {
                self.focus_nav.handle_key(row, col, e);
                false
            },
            MealMsg::MouseClick => {
                self.focus_nav.set_edit();
                false
            },
            MealMsg::HandlePaste(text, row_index, column_index) => {
                log!("row:", row_index, "col:", column_index, "text:", &text);
                handle_paste(&text, row_index, column_index, &mut terv.osszetevok, &mut self.focus_nav);
                true
            },
            MealMsg::DoNothing => false,
            _ => {true}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let app_data = link.context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        let recipe_list = terv.recipes.iter().map(|recipe| &recipe.name);
        let mut recipe_list: Vec<_> = recipe_list.chain(terv.meals.iter().map(|meal| &meal.as_common().main_recipe)).collect();
        recipe_list.sort();
        recipe_list.dedup();

        html! {
            <div class="meals">
                <datalist id="recipe_list">
                    { for recipe_list.iter().map(|&value| {
                        html! {<option value={value.clone()} />}
                    })}
                </datalist>
                <table>
                    <tr>
                        <th>{ "Recipe" }</th><th>{ "Létszám" }</th><th>{ "Nap" }</th>
                    </tr>
                    { for terv.meals.iter().enumerate().map(|(index, meal)| {
                        let update_recipe = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            MealMsg::UpdateRecipe(index, input.value())
                        });

                        let update_number = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            MealMsg::UpdateNumber(index, input.value())
                        });

                        let update_day = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            MealMsg::UpdateDay(index, input.value())
                        });

                        let onkeydown = |col| link.callback(move |e: KeyboardEvent| {
                            MealMsg::KeyPressed(index, col, e)
                        });

                        let onclick = link.callback(move |_| {
                            MealMsg::MouseClick
                        });

                        //let value = value.as_common();
                        match &meal.ty {
                            MealType::Common(common) => html! {
                                <tr>
                                    <td><input type="text" list="recipe_list" value={common.main_recipe.clone()} onchange={update_recipe}
                                        onkeydown={onkeydown(0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} /></td>
                                    <td><input type="number" min="0" value={if common.number != 0 {common.number.to_string()} else {"".to_string()}} onchange={update_number}
                                        onkeydown={onkeydown(1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} /></td>
                                    <td><input value={meal.day.to_string()} onchange={update_day}
                                        onkeydown={onkeydown(2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} /></td>
                                    <td><button onclick={link.callback(move |_| MealMsg::RemoveMeal(index))}>{ "Remove" }</button></td>
                                    {for common.get_errors(&terv).iter().map(|error| html!{
                                        <td class="err">{ match error {
                                            EW::Owned(text) => text.clone(),
                                            _ => "hiba a kódban".to_string(),
                                        } }</td>
                                    })}
                                    {for common.get_warnings(&terv).iter().map(|warning| html!{
                                        <td class="warn">{ match warning {
                                            EW::Owned(text) => text.clone(),
                                            _ => "hiba a kódban".to_string(),
                                        } }</td>
                                    })}
                                </tr>
                            },
                            MealType::Troup(troup) => html!{},
                        }
                    })}
                </table>
                <button onclick={link.callback(move |_| MealMsg::AddMeal)}>{ "Add Meal" }</button>
            </div>
        }
    }
}