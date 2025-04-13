use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::recipe::{Recipes, Recipe};
use crate::osszetevok::{Osszetevo, Osszetevok};
use crate::meal::{Meal, Meals};
use crate::matrix::{Matrix, Subs, Sub};
use crate::shop::{Shoppings, Shopping};

#[derive(PartialEq, Clone)]
pub struct Terv {
    pub osszetevok: Osszetevok,
    pub recipes: Recipes,
    pub meals: Meals,
    pub shoppingdays: Shoppings,
    matrix: Matrix,
}

impl Terv {
    pub fn new() -> Self {
        Terv {
            osszetevok: Osszetevok(vec![Osszetevo::new()]),
            recipes: Recipes(vec![Recipe::new()]),
            meals: Meals(vec![Meal::new()]),
            shoppingdays: Shoppings::new(),
            matrix: Matrix::new(),
        }
    }

    pub fn calculate_matrix(&mut self) {
        let mut meals = self.meals.clone();

        self.matrix.clear();

        let mut days: Vec<i32> = Vec::new();

        for shoppingday in self.shoppingdays.iter() {
            match shoppingday {
                Shopping::Day(day) => days.push(*day),
                Shopping::Name(name) => {
                    for (index, meal) in meals.clone().iter().enumerate() {
                        if let Shopping::Name(meal_day) = &meal.day{
                            if meal_day == name {
                                meals.remove(index);
                                let hash = self.matrix.get_mut(shoppingday).unwrap();
                                let recipe = self.recipes.get_recipe(&meal.recipe).unwrap();
                                for ingredient in recipe.ingredients.iter() {
                                    let sub = Sub{quantity: ingredient.quantity, recipe: recipe.name.clone()};
                                    hash.entry(ingredient.name.clone()).or_insert(Subs(vec![sub.clone()])).push(sub);
                                }
                        }}
                    }
                }
            }
        }
        days.sort();

        let mut szukseg: HashMap<i32, HashMap<String, bool>> = HashMap::new();
        for day in days.iter() {
            szukseg.insert(day.clone(), HashMap::new());
        }

        for meal in meals.iter() {
            if let Shopping::Day(day) = meal.day {
                let mut szukseg_day = szukseg.get_mut(&day).unwrap();
                for ingredient in self.recipes.get_recipe(&meal.recipe).unwrap().ingredients.iter() {
                    szukseg_day.entry(ingredient.name.clone()).or_insert(true);
                }
            }
        }

        let def_osszetevo = self.osszetevok.get_def();
        for (index, day) in days.iter().enumerate() {
            
        }

        // for shoppingday in self.shoppingdays.iter() {
        //     let mut list: HashMap<String, Subs> = HashMap::new();
        //     for meal in meals.iter() {
        //         if let Some(recipe) = self.recipes.get_recipe(&meal.recipe) {

        //         }
        //     }
        // }
    }
}

pub type TervContext = Rc<RefCell<Terv>>;