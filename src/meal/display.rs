use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::backend::keyboard::TableFocusNavigator;
use crate::terv::AppContext;
use crate::terv::display::TervProps;

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
        let app_data = link.context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        let recipe_list = terv.recipes.iter().map(|recipe| &recipe.name);
        let mut recipe_list: Vec<_> = recipe_list.chain(terv.meals.iter().map(|meal| &meal.recipe)).collect();
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
                                <td><input type="text" list="recipe_list" value={value.recipe.clone()} onchange={update_recipe}
                                    onkeydown={onkeydown(0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} /></td>
                                <td><input type="number" min="0" value={if value.number != 0 {value.number.to_string()} else {"".to_string()}} onchange={update_number}
                                    onkeydown={onkeydown(1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} /></td>
                                <td><input value={value.day.to_string()} onchange={update_day}
                                    onkeydown={onkeydown(2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} /></td>
                                <td><button onclick={link.callback(move |_| MealMsg::RemoveMeal(index))}>{ "Remove" }</button></td>
                                if !terv.recipes.exist(&value.recipe) {
                                    <td class="err">{ "A recept nem található" }</td>
                                }
                                if value.number == 0 {
                                    <td class="warn">{ "A létszám nulla" }</td>
                                }
                                if value.day == ShopDay::Day(Time::new()) || value.day == ShopDay::Name(String::new()) {
                                    <td class="warn">{ "A nap nulla / nem változott" }</td>
                                }
                            </tr>
                        }
                    })}
                </table>
                <button onclick={link.callback(move |_| MealMsg::AddMeal)}>{ "Add Meal" }</button>
            </div>
        }
    }
}