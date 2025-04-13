use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::ops::{Deref, DerefMut};

//use crate::osszetevok::Osszetevo;
use crate::terv::{Terv, TervContext};

#[derive(Debug, PartialEq, Clone)]
pub struct Ingredient {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
}


impl Ingredient {
    pub fn new() -> Self {
        Ingredient {
            name: String::new(),
            quantity: 0.0,
            unit: String::new(),
        }
    }
    // pub fn get_component<'a, 'b>(&'a self, osszetevok: &'b Vec<Osszetevo>) -> Option<&'b Osszetevo> {
    //     for osszetevo in osszetevok {
    //         if osszetevo.name == self.name {
    //             return Some(osszetevo);
    //         }
    //     }
    //     None
    // }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Recipe {
    pub name: String,
    pub number: u32,
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new() -> Self {
        Recipe {
            name: String::new(),
            number: 0,
            ingredients: Vec::new(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Recipes(pub Vec<Recipe>);

impl Recipes {
    pub fn new() -> Self {
        Recipes (Vec::new())
    }

    pub fn exist(&self, recipe_name: &str) -> bool {
        for recipe in self.iter() {
            if recipe.name == recipe_name {
                return true;
            }
        }
        false
    }

    pub fn get_recipe(&mut self, recipe_name: &str) -> Option<&mut Recipe> {
        for recipe in self.iter_mut() {
            if recipe.name == recipe_name {
                return Some(recipe);
            }
        }
        None
    }
}

impl Deref for Recipes {
    type Target = Vec<Recipe>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Recipes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct RecipePage {
    current_recipe: Option<usize>,
}

impl RecipePage {
    pub fn search_recipe(&mut self, name: &str, terv: &Terv) -> Result<(), String> {
        for (index, recipe) in terv.recipes.iter().enumerate() {
            if recipe.name.contains(name) {
                self.current_recipe = Some(index);
                return Ok(());
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
}

impl Component for RecipePage {
    type Message = RecipeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RecipePage {
            current_recipe: Some(0)
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();

        match msg {
            RecipeMsg::AddRecipe => {
                terv.recipes.push(Recipe::new());
                self.current_recipe = Some(terv.recipes.len() - 1);
                true
            },
            RecipeMsg::SearchRecipe(name) => {
                let _ = self.search_recipe(name.as_str(), &terv);
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
                true
            },
            RecipeMsg::RemoveIngredient(index) => {
                let recipe = self.get_curr_recipe(&mut terv);
                recipe.ingredients.remove(index);
                true
            },
            RecipeMsg::RemoveRecipe => {
                terv.recipes.remove(self.current_recipe.expect("Nem lehet rossz! 001"));
                true
            },
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();
        
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
        
                                    html! {
                                        <tr>
                                            <th><input type="text" value={value.name.clone()} onchange={update_name} /></th>
                                            <th><input type="number" step="any" value={value.quantity.to_string()} onchange={update_quantity} /></th>
                                            <th><input type="text" value={value.unit.clone()} onchange={update_unit} /></th>
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