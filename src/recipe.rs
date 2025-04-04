use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::osszetevok::Osszetevo;
use crate::display::AppState;
use crate::terv::Terv;

#[derive(Debug, PartialEq, Clone)]
struct Ingredient {
    name: String,
    quantity: f64,
    unit: String,
}


impl Ingredient {
    pub fn new() -> Self {
        Ingredient {
            name: String::new(),
            quantity: 0.0,
            unit: String::new(),
        }
    }
    pub fn get_component<'a, 'b>(&'a self, osszetevok: &'b Vec<Osszetevo>) -> Option<&'b Osszetevo> {
        for osszetevo in osszetevok {
            if osszetevo.name == self.name {
                return Some(osszetevo);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Recipe {
    name: String,
    number: u32,
    ingredients: Vec<Ingredient>,
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

#[derive(Debug)]
pub struct RecipePage {
    current_recipe: Option<usize>,
}

impl RecipePage {
    pub fn new() -> Self {
        RecipePage {
            current_recipe: None,
        }
    }
    pub fn search_recipe(&mut self, name: &str, terv: &Terv) -> Result<(), String> {
        for (index, recipe) in terv.recipes.iter().enumerate() {
            if recipe.name.contains(name) {
                self.current_recipe = Some(index);
                return Ok(());
            }
        }
        Err(format!("Nem található ilyen recept: {name}"))
    }

    fn get_recipe<'a, 'b>(&'a mut self, terv: &'b mut Terv) -> &'b mut Recipe {
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
        RecipePage::new()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<AppState>(Callback::noop()).unwrap().0.terv;

        match msg {
            RecipeMsg::AddRecipe => {
                let mut new_terv = (**terv).clone();
                new_terv.recipes.push(Recipe::new());
                self.current_recipe = Some(new_terv.recipes.len() - 1);
                terv.set(new_terv);
                true
            },
            RecipeMsg::SearchRecipe(name) => {
                let _ = self.search_recipe(name.as_str(), &terv);
                true
            },
            RecipeMsg::UpdateName(name) => {
                let mut new_terv = (**terv).clone();
                let recipe = self.get_recipe(&mut new_terv);
                recipe.name = name;
                println!("{:?}", recipe);
                terv.set(new_terv);
                true
            },
            RecipeMsg::UpdateNumber(number) => {
                let mut new_terv = (**terv).clone();
                let recipe = self.get_recipe(&mut new_terv);
                if let Ok(number) = number.parse() {
                    recipe.number = number;
                }
                terv.set(new_terv);
                true
            },
            RecipeMsg::UpdateIngredientName(index, name) => {
                let mut new_terv = (**terv).clone();
                let recipe = self.get_recipe(&mut new_terv);
                recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 006").name = name;
                terv.set(new_terv);
                true
            },
            RecipeMsg::UpdateQuantity(index, quantity) => {
                let mut new_terv = (**terv).clone();
                let recipe = self.get_recipe(&mut new_terv);
                if let Ok(quantity) = quantity.parse() {
                    recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 003").quantity = quantity;
                }
                terv.set(new_terv);
                true
            },
            RecipeMsg::UpdateUnit(index, unit) => {
                let mut new_terv = (**terv).clone();
                let recipe = self.get_recipe(&mut new_terv);
                recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 002").unit = unit;
                terv.set(new_terv);
                true
            },
            RecipeMsg::RemoveRecipe => {
                let mut new_terv = (**terv).clone();
                new_terv.recipes.remove(self.current_recipe.expect("Nem lehet rossz! 001"));
                terv.set(new_terv);
                true
            },
            RecipeMsg::AddIngredient => {
                let mut new_terv = (**terv).clone();
                let recipe = self.get_recipe(&mut new_terv);
                recipe.ingredients.push(Ingredient::new());
                terv.set(new_terv);
                true
            },
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let terv = ctx.link().context::<AppState>(Callback::noop()).unwrap().0.terv;
        
        html! {
            <div class="recipes">
                <div class="toolbar">
                    <button onclick={link.callback(|_| RecipeMsg::AddRecipe)}>{ "Add recipe" }</button>
                    //<label for="recipe_search">{ "Kerressen rá egy receptre" }</label>
                    <input type="text" list="recipe_list" id="recipe_search" name="recipe_search" oninput={
                        link.callback(move |e: InputEvent| {
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
                                    <th><input type="text" value={recipe.name.clone()} oninput={link.callback(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateName(input.value())})} /></th>
                                </tr>
                                <tr>
                                    <th>{ "Létszám:" }</th>
                                    <th><input type="number" min="0" step="1" value={recipe.number.to_string()} 
                                        oninput={link.callback(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateNumber(input.value())})} /></th>
                                </tr>
                                <tr>
                                    <th>{ "Összetevő" }</th><th>{ "Mennyiség" }</th><th>{ "Mértékegység" }</th>
                                </tr>
                                { for recipe.ingredients.iter().enumerate().map(|(index, value)| {
                                    let update_name = link.callback(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateIngredientName(index, input.value())
                                    });
        
                                    let update_quantity = link.callback(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateQuantity(index, input.value())
                                    });
        
                                    let update_unit = link.callback(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        RecipeMsg::UpdateUnit(index, input.value())
                                    });
        
                                    html! {
                                        <tr>
                                            <th><input type="text" value={value.name.clone()} oninput={update_name} /></th>
                                            <th><input type="number" step="any" value={value.quantity.to_string()} oninput={update_quantity} /></th>
                                            <th><input type="text" value={value.unit.clone()} oninput={update_unit} /></th>
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