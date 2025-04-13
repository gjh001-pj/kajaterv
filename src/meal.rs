use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::ops::{Deref, DerefMut};

use crate::terv::TervContext;
use crate::shop::Shopping;


#[derive(PartialEq, Clone)]
pub struct Meal {
    pub recipe: String,
    pub number: u32,
    pub day: Shopping,
}

impl Meal {
    pub fn new() -> Self {
        Meal {
            recipe: String::new(),
            number: 0,
            day: Shopping::Day(0),
        }
    }
}

#[derive(PartialEq, Clone)]
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

pub struct MealPage {}

pub enum MealMsg {
    AddMeal,
    UpdateRecipe(usize, String),
    UpdateNumber(usize, String),
    UpdateDay(usize, String),

    RemoveMeal(usize),
}

impl Component for MealPage {
    type Message = MealMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MealPage {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();
        match msg {
            MealMsg::AddMeal => {
                terv.meals.push(Meal::new());
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
                    terv.meals.get_mut(index).unwrap().day = Shopping::Day(day);
                } else {
                    terv.meals.get_mut(index).unwrap().day = Shopping::Name(day);
                }
                true
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

                        html! {
                            <tr>
                                <th><input type="text" list="recipe_list" value={value.recipe.clone()} onchange={update_recipe} /></th>
                                <th><input type="number" min="0" value={value.number.to_string()} onchange={update_number} /></th>
                                <th><input type="number" value={value.day.to_string()} onchange={update_day} /></th>
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