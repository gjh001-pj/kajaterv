
use std::ops::{Deref, DerefMut};

//use crate::osszetevok::Osszetevo;


pub mod ingredient;
pub mod display;

use ingredient::Ingredient;

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
