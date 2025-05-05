use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::ops::{Deref, DerefMut};

use crate::keyboard::TableFocusNavigator;
use crate::terv::TervContext;
use crate::shop::{Shopping, ShopDay};


#[derive(PartialEq, Clone, Debug)]
pub struct Meal {
    pub recipe: String,
    pub number: u32,
    pub day: ShopDay,
}

impl Meal {
    pub fn new() -> Self {
        Meal {
            recipe: String::new(),
            number: 0,
            day: ShopDay::Day(0),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Meals(pub Vec<Meal>);

impl Deref for Meals {
    type Target = Vec<Meal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Meals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
}

impl Component for MealPage {
    type Message = MealMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MealPage {
            focus_nav: TableFocusNavigator::new(1, 3),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();
        match msg {
            MealMsg::AddMeal => {
                terv.meals.push(Meal::new());
                self.focus_nav.build(terv.meals.len(), 3);
                true
            },
            MealMsg::UpdateRecipe(index, recipe) => {
                terv.meals.get_mut(index).unwrap().recipe = recipe;
                true
            },
            MealMsg::UpdateNumber(index, number) => {
                if let Ok(number) = number.parse() {
                    terv.meals.get_mut(index).unwrap().number = number;
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
            _ => {true}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let terv = link.context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();

        html! {
            <div class="meals">
                <button onclick={link.callback(move |_| MealMsg::AddMeal)}>{ "Add Meal" }</button>
                <datalist id="recipe_list">
                    { for terv.recipes.iter().map(|value| {
                        html! {<option value={value.name.clone()} />}
                    })}
                </datalist>
                <table>
                    <tr>
                        <th>{ "Recipe" }</th><th>{ "Létszám" }</th><th>{ "Nap" }</th>
                    </tr>
                    { for terv.meals.iter().enumerate().map(|(index, value)| {
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

                        html! {
                            <tr>
                                <th><input type="text" list="recipe_list" value={value.recipe.clone()} onchange={update_recipe}
                                    onkeydown={onkeydown(0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} /></th>
                                <th><input type="number" min="0" value={value.number.to_string()} onchange={update_number}
                                    onkeydown={onkeydown(1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} /></th>
                                <th><input type="number" value={value.day.to_string()} onchange={update_day}
                                    onkeydown={onkeydown(2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} /></th>
                                <th><button onclick={link.callback(move |_| MealMsg::RemoveMeal(index))}>{ "Remove" }</button></th>
                                if !terv.recipes.exist(&value.recipe) {
                                    <th>{ "A recept nem található" }</th>
                                }
                            </tr>
                        }
                    })}
                </table>
            </div>
        }
    }
}