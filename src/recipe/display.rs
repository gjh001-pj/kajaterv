use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::HtmlInputElement;
use gloo::console::log;

use crate::terv::{Terv, TervContext};
use crate::keyboard::TableFocusNavigator;
use crate::terv::display::TervProps;

use super::*;


#[derive(Debug)]
pub struct RecipePage {
    pub current_recipe: Option<usize>,
    pub focus_nav: TableFocusNavigator,
}

impl RecipePage {
    pub fn search_recipe<'a, 'b, 'c>(&'a mut self, name: &'b str, terv: &'c Terv) -> Result<&'c Recipe, String> {
        for (index, recipe) in terv.recipes.iter().enumerate() {
            if recipe.name.contains(name) {
                self.current_recipe = Some(index);
                return Ok(recipe);
            }
        }
        Err(format!("Nem található ilyen recept: {name}"))
    }

    fn get_curr_recipe<'a, 'b>(&'a mut self, terv: &'b mut Terv) -> &'b mut Recipe {
        terv.recipes.get_mut(self.current_recipe.expect("Nem lehet rossz! 004")).expect("Nem lehet rossz! 005")
    }
}

// Display

pub enum RecipeMsg {
    AddRecipe,
    SearchRecipe(String),
    UpdateName(String),
    UpdateNumber(String),
    UpdateIngredientName(usize, String),
    UpdateQuantity(usize, String),
    UpdateUnit(usize, String),
    RemoveIngredient(usize),
    AddIngredient,
    RemoveRecipe,
    KeyPressed(usize, usize, KeyboardEvent),
    MouseClick,
}

impl Component for RecipePage {
    type Message = RecipeMsg;
    type Properties = TervProps;

    fn create(ctx: &Context<Self>) -> Self {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();
        let mut focus_nav = TableFocusNavigator::new(0, 3);
        if let Some(recipe) = terv.recipes.get(0) {
            focus_nav.build(recipe.ingredients.len(), 3);
        }
        RecipePage {
            current_recipe: Some(0),
            focus_nav,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();

        match msg {
            RecipeMsg::AddRecipe => {
                terv.recipes.push(Recipe::new());
                self.current_recipe = Some(terv.recipes.len() - 1);
                self.focus_nav.build(0, 3);
                true
            },
            RecipeMsg::SearchRecipe(name) => {
                if let Ok(recipe) = self.search_recipe(name.as_str(), &terv) {
                    self.focus_nav.build(recipe.ingredients.len(), 3);
                }
                true
            },
            RecipeMsg::UpdateName(name) => {
                let recipe = self.get_curr_recipe(&mut terv);
                recipe.name = name;
                println!("{:?}", recipe);
                true
            },
            RecipeMsg::UpdateNumber(number) => {
                if let Ok(number) = number.parse() {
                    let recipe = self.get_curr_recipe(&mut terv);
                    recipe.number = number;
                }
                true
            },
            RecipeMsg::UpdateIngredientName(index, name) => {
                let recipe = self.get_curr_recipe(&mut terv);
                recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 006").name = name;
                true
            },
            RecipeMsg::UpdateQuantity(index, quantity) => {
                if let Ok(quantity) = quantity.parse() {
                    let recipe = self.get_curr_recipe(&mut terv);
                    recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 003").quantity = quantity;
                }
                true
            },
            RecipeMsg::UpdateUnit(index, unit) => {
                let recipe = self.get_curr_recipe(&mut terv);
                recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 002").unit = unit;
                true
            },
            RecipeMsg::AddIngredient => {
                let recipe = self.get_curr_recipe(&mut terv);
                recipe.ingredients.push(Ingredient::new());
                self.focus_nav.build(recipe.ingredients.len(), 3);
                true
            },
            RecipeMsg::RemoveIngredient(index) => {
                let recipe = self.get_curr_recipe(&mut terv);
                recipe.ingredients.remove(index);
                self.focus_nav.build(recipe.ingredients.len(), 3);
                true
            },
            RecipeMsg::RemoveRecipe => {
                terv.recipes.remove(self.current_recipe.expect("Nem lehet rossz! 001"));
                if terv.recipes.len() > 0 {
                    self.current_recipe = Some(0);
                } else {
                    self.current_recipe = None;
                }
                true
            },
            RecipeMsg::KeyPressed(row, col, e) => {
                self.focus_nav.handle_key(row, col, e);
                false
            },
            RecipeMsg::MouseClick => {
                self.focus_nav.set_edit();
                false
            },
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();

        let mut osszetevo_name_list: Vec<VNode> = Vec::new();
        if let Some(recipe_index) = self.current_recipe {
            if let Some(recipe) = terv.recipes.get(recipe_index) {
                let all_osszetevo_name_list: Vec<&String> = terv.osszetevok.iter().map(|ossz| &ossz.name).collect();
                osszetevo_name_list = all_osszetevo_name_list.iter().map(|&rec_ossz| {
                    if !recipe.ingredients.iter().map(|x| &x.name).collect::<Vec<&String>>().contains(&rec_ossz) {
                        html! {<option value={rec_ossz.clone()} />}
                    } else {
                        html! {}
                    }
                }).collect();
            }
        }
        
        html! {
            <div class="recipes">
                <div class="toolbar">
                    <button onclick={link.callback(|_| RecipeMsg::AddRecipe)}>{ "Add recipe" }</button>
                    //<label for="recipe_search">{ "Kerressen rá egy receptre" }</label>
                    <input type="text" list="recipe_list" id="recipe_search" name="recipe_search" onchange={
                        link.callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        RecipeMsg::SearchRecipe(input.value())})} />
                    <datalist id="recipe_list">
                        { for terv.recipes.iter().map(|value| {
                            html! {<option value={value.name.clone()} />}
                        })}
                    </datalist>
                    <datalist id="osszetevo_name_list">
                        { for osszetevo_name_list }
                    </datalist>
                    <p>{ format!("Kiválasztott recept: {}", match self.current_recipe {
                        Some(index) => {terv.recipes.get(index).unwrap().name.clone()},
                        None => {String::from("Nem található ilyen recept!")}
                    }) }</p>
                </div>
                <div class="current_recipe">
                    if let Some(recipe_index) = self.current_recipe {
                        if let Some(recipe) = terv.recipes.get(recipe_index) {
                            <table>
                                <tr>
                                    <th>{ "Név:" }</th>
                                    <th><input type="text" value={recipe.name.clone()} onchange={link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateName(input.value())})} /></th>
                                </tr>
                                <tr>
                                    <th>{ "Létszám:" }</th>
                                    <th><input type="number" min="0" step="1" value={recipe.number.to_string()} 
                                        onchange={link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateNumber(input.value())})} /></th>
                                </tr>
                                <tr>
                                    <th>{ "Összetevő" }</th><th>{ "Mennyiség" }</th><th>{ "Mértékegység" }</th>
                                </tr>
                                { for recipe.ingredients.iter().enumerate().map(|(index, value)| {
                                    let update_name = link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateIngredientName(index, input.value())
                                    });
        
                                    let update_quantity = link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateQuantity(index, input.value())
                                    });
        
                                    let update_unit = link.callback(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateUnit(index, input.value())
                                    });

                                    let onkeydown = |col| link.callback(move |e: KeyboardEvent| {
                                        RecipeMsg::KeyPressed(index, col, e)
                                    });
        
                                    let onclick = link.callback(move |_| {
                                        RecipeMsg::MouseClick
                                    });
        
                                    html! {
                                        <tr>
                                            <th><input type="text" list="osszetevo_name_list" value={value.name.clone()} onchange={update_name}
                                                onkeydown={onkeydown(0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} /></th>
                                            <th><input type="number" step="any" value={value.quantity.to_string()} onchange={update_quantity}
                                                onkeydown={onkeydown(1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} /></th>
                                            <th><input type="text" value={value.unit.clone()} onchange={update_unit}
                                                onkeydown={onkeydown(2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} /></th>
                                            <th><button onclick={link.callback(move |_| RecipeMsg::RemoveIngredient(index))}>{ "Remove" }</button></th>
                                        </tr>
                                    }
                                })}
                                <button onclick={link.callback(move |_| RecipeMsg::AddIngredient)}>{ "Add Ingredient" }</button>
                                <button onclick={link.callback(move |_| RecipeMsg::RemoveRecipe)}>{ "Remove Recipe" }</button>
                            </table>
                    }}
                </div>
            </div>
        }
    }
}