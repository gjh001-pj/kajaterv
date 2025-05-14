use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::recipe::{Recipes, Recipe, ingredient::Ingredient};
use crate::osszetevok::{Osszetevo, Osszetevok};
use crate::meal::{Meal, Meals};
use crate::matrix::{Matrix, Subs, Sub};
use crate::shop::{Shoppings, Shopping, ShopDay};

pub mod display;

#[derive(PartialEq, Clone, Debug)]
pub struct Terv {
    pub osszetevok: Osszetevok,
    pub recipes: Recipes,
    pub meals: Meals,
    pub shoppingdays: Shoppings,
    pub matrix: Matrix,
    pub version: u64,
}

impl Terv {
    pub fn new() -> Self {
        Terv {
            osszetevok: Osszetevok(vec![Osszetevo::new()]),
            recipes: Recipes(vec![Recipe::new()]),
            meals: Meals(vec![Meal::new()]),
            shoppingdays: Shoppings(vec![Shopping::new()]),
            matrix: Matrix::new(),
            version: 0,
        }
    }

    pub fn calculate_matrix(&mut self) {
        let mut meals = self.meals.clone();

        self.matrix.clear();

        let mut vdays: Vec<i32> = Vec::new();

        for shoppingday in self.shoppingdays.iter() {
            match &shoppingday.day {
                ShopDay::Day(day) => vdays.push(*day),
                ShopDay::Name(name) => {
                    for (index, meal) in meals.clone().iter().enumerate() {
                        if let ShopDay::Name(meal_day) = &meal.day{
                            if meal_day == name {
                                meals.remove(index);
                                let hash = self.matrix.get_mut(&shoppingday.day).unwrap();
                                let recipe = self.recipes.get_recipe(&meal.recipe).unwrap();
                                for ingredient in recipe.ingredients.iter() {
                                    let sub = Sub{
                                        quantity: ingredient.quantity, 
                                        recipe: recipe.name.clone(), 
                                        number: meal.number
                                    };
                                    hash.entry(ingredient.name.clone()).or_insert(Subs(vec![sub.clone()])).push(sub);
                                }
                        }}
                    }
                }
            }
        }
        vdays.sort();

        let mut ossz_vasar: HashMap<String, Vec<i32>> = HashMap::new();
        for meal in meals.iter_mut() {
            let day = meal.day.as_day().clone();
            for ingredient in &self.recipes.get_recipe(&meal.recipe).unwrap().ingredients {
                ossz_vasar.entry(ingredient.name.clone()).or_insert(Vec::new()).push(day);
            }
        }

        let mut vasar: HashMap<i32, HashMap<String, bool>> = HashMap::new();
        for day in vdays.iter() {
            vasar.insert(*day, HashMap::new());
        }

        println!("ossz_vasar: {:?}", ossz_vasar);
        println!("vdays: {:?}", vdays);

        for (ingredient, idays) in ossz_vasar.iter_mut() {
            println!("ing_name: {}, vdays: {:?}", ingredient, vdays);
            idays.sort();
            let time = self.osszetevok.by_name(ingredient).unwrap().time as i32;
            let terjedelem = idays.iter().max().unwrap() - idays.iter().min().unwrap() + 1;
            let count = (terjedelem as f32 / time as f32).ceil() as i32;
            println!("time: {}, terj: {}, count: {}", time, terjedelem, count);
            for _ in 0..count {
                match idays.first() {
                    Some(iday) => {
                        let iday = *iday;
                        println!("vasar: {:?}, vdays: {:?}, iday: {}, idays: {:?}", vasar, vdays, iday, idays);
                        let vday = vdays.iter().filter(|&x| *x <= iday).max().unwrap();
                        let hash = vasar.get_mut(&vday).unwrap();
                        hash.insert(ingredient.clone(), true);
                        idays.retain(|jday| *jday >= vday + time)
                    },
                    None => break,
                }
            }
        }

        println!("{:?}", vasar.get(&1));

        for meal in meals.iter() {
            let day = meal.day.as_day().clone();
            println!("meal.recipe: {}, recipe: {}", meal.recipe, self.recipes[0].name);
            for ingredient in self.recipes.get_recipe(&meal.recipe).unwrap().ingredients.iter() {
                println!("vasar: {:?}, name: {}, res: {:?}", vasar, ingredient.name, get_shopping_days(&vasar, &ingredient.name));
                let vday = get_shopping_days(&vasar, &ingredient.name).iter().filter(|&x| *x <= day).max().unwrap().clone();
                let hash = match self.matrix.get_mut(&ShopDay::Day(vday)) {
                    Some(h) => h,
                    None => {
                        self.matrix.insert(ShopDay::Day(vday), HashMap::new());
                        self.matrix.get_mut(&ShopDay::Day(vday)).unwrap()
                    }
                };
                let sub = Sub{
                    quantity: ingredient.quantity, 
                    recipe: meal.recipe.clone(), 
                    number: meal.number
                };
                hash.entry(ingredient.name.clone()).or_insert(Subs::new()).push(sub);
            }
        }
    }
}

pub type TervContext = Rc<RefCell<Terv>>;

fn get_shopping_days(vasar: &HashMap<i32, HashMap<String, bool>>, ingredient: &str) -> Vec<i32> {
    let mut res = Vec::new();
    for (day, hash) in vasar.iter() {
        if hash.contains_key(ingredient) {
            res.push(*day);
        }
    }
    res
}

#[test]
fn test_calculate_matrix() {
    let a = vec![1, 2, 3, 4];
    let mut terv = Terv {
        osszetevok: Osszetevok (vec![Osszetevo {
            name: String::from("aaa"), 
            unit: String::from("m"), 
            time: 2, 
            unit_price: 100.0
        }]),
        recipes: Recipes (vec![Recipe {
            name: String::from("alma"),
            number: 10,
            ingredients: vec![Ingredient {
                name: String::from("aaa"),
                quantity: 1.0,
                unit: String::from("m"),
            }],
        }]),
        meals: Meals(vec![Meal {
            recipe: String::from("alma"),
            number: 10,
            day: ShopDay::Day(2)
        }]),
        shoppingdays: Shoppings(vec![Shopping {
            day: ShopDay::Day(2),
            name: String::from("kettő"),
        }]),
        matrix: Matrix::new(),
        version: 0,
    };

    terv.calculate_matrix();

    print!("matrix: {:#?}", terv.matrix);
}