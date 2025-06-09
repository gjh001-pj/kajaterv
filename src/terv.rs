use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use gloo::console::log;

use crate::convert::{Conversation, Conversations, Convert};
use crate::recipe::{Recipes, Recipe, ingredient::Ingredient};
use crate::osszetevok::{Osszetevo, Osszetevok};
use crate::meal::{Meal, Meals};
use crate::backend::matrix::{Matrix, Subs, Sub};
use crate::shop::{Shoppings, Shopping, ShopDay};
use crate::backend::time::Time;

pub mod display;

#[derive(PartialEq, Clone, Debug)]
pub struct Terv {
    pub osszetevok: Osszetevok,
    pub recipes: Recipes,
    pub meals: Meals,
    pub shoppingdays: Shoppings,
    pub matrix: Matrix,
    pub convs: Conversations,
    pub error: Option<String>,
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
            convs: Conversations(vec![
                Conversation{ from: "g".to_string(), to: "kg".to_string(), factor: 0.001},
                Conversation{ from: "dkg".to_string(), to: "kg".to_string(), factor: 0.01},
                Conversation{ from: "ml".to_string(), to: "l".to_string(), factor: 0.001},
                Conversation{ from: "dl".to_string(), to: "l".to_string(), factor: 0.1},
                Conversation{ from: "cl".to_string(), to: "l".to_string(), factor: 0.01},
            ]),
            error: None,
            version: 0,
        }
    }

    pub fn calculate_matrix(&mut self) -> Option<String> {
        let mut meals = self.meals.clone();

        self.matrix.clear();

        let mut vdays: Vec<Time> = Vec::new();

        for shoppingday in self.shoppingdays.iter() {
            match &shoppingday.day {
                ShopDay::Day(day) => vdays.push(*day),
                ShopDay::Name(name) => {
                    for (index, meal) in meals.clone().iter().enumerate() {
                        if let ShopDay::Name(meal_day) = &meal.day{
                            if meal_day == name {
                                meals.remove(index);
                                let hash = self.matrix.entry(meal.day.clone()).or_insert(HashMap::new());
                                let recipe = match self.recipes.get_recipe(&meal.recipe) {
                                    Some(v) => v,
                                    None => {
                                        return Some(format!("Nem található ilyen recept: {}, étkezés ideje: {}", meal.recipe, meal.day.to_string()));
                                    }
                                };
                                for ingredient in recipe.ingredients.iter() {
                                    let to = match self.osszetevok.by_name(&ingredient.name) {
                                        Some(osszetevo) => &osszetevo.unit,
                                        None => {
                                            return Some(format!("Összetevő nem található: {}", ingredient.name));
                                        }
                                    };
                                    let converted = match ingredient.convert(&to, &self.convs) {
                                        Some(quantity) => quantity,
                                        None => {
                                            return Some(format!("Nem lehet {}-t {}-ra/re átváltani.", ingredient.unit, to));
                                        }
                                    };
                                    let sub = Sub{
                                        quantity: converted * meal.number as f64 / recipe.number as f64, 
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


        let mut ossz_vasar: HashMap<String, Vec<Time>> = HashMap::new();
        for meal in meals.iter_mut() {
            let day = meal.day.as_day().clone();
            let recipe = match self.recipes.get_recipe(&meal.recipe) {
                Some(v) => v,
                None => {
                    return Some(format!("Nem található ilyen recept: {}, étkezés ideje: {}", meal.recipe, meal.day.to_string()))
                }
            };
            for ingredient in recipe.ingredients.iter() {
                ossz_vasar.entry(ingredient.name.clone()).or_insert(Vec::new()).push(day);
            }
        }


        let mut vasar: HashMap<Time, HashMap<String, bool>> = HashMap::new();
        for day in vdays.iter() {
            vasar.insert(*day, HashMap::new());
        }


        println!("ossz_vasar: {:?}", ossz_vasar);
        println!("vdays: {:?}", vdays);

        for (ingredient, idays) in ossz_vasar.iter_mut() {
            idays.sort();
            let time = match self.osszetevok.by_name(ingredient) {
                Some(ingredient) => ingredient.time.as_day(),
                None => {
                    return Some(format!("Összetevő nem található: {}", ingredient));
                }
            };
            // let mut i = 0;
            println!("time: {:?}", time);
            loop {
                println!("idays: {:?}", idays);
                match idays.first() {
                    Some(iday) => {
                        let vday = match vdays.iter().filter(|&x| x <= iday).max() {
                            Some(vday) => vday,
                            None => {
                                return Some(format!("Nem lehet venni {}-t, az alábbi napokra: {:?}, elállási idő: {:?}, vásárnapok: {:?}",
                                    ingredient, idays, time, vdays));
                            }
                        };
                        println!("vday: {:?}", vday);
                        let hash = vasar.get_mut(vday).expect("Hiba a kódban, korábban az összes vday-t belepakoltuk vasar-ba.");
                        hash.insert(ingredient.clone(), true);
                        if *idays.iter().min().expect("A match-nál ellenőriztük, hogy van a listának első eleme.") >= *vday + *time {
                            return Some(format!("nem lehet megvenni {} összetevőt ekkorra: {:?}, előtte lévő vásárlás: {:?}, elállási idő: {:?}",
                                ingredient, iday, vday, time));
                        }
                        idays.retain(|&jday| jday >= *vday + *time);
                    },
                    None => break,
                }
                // if i > 10 {
                //     break;
                // } else {
                //     i += 1;
                // }

            }
            
            // println!("ing_name: {}, vdays: {:?}", ingredient, vdays);
            // idays.sort();
            // let time = self.osszetevok.by_name(ingredient).unwrap().time as i32;
            // let terjedelem = idays.iter().max().unwrap() - idays.iter().min().unwrap() + 1;
            // let count = (terjedelem as f32 / time as f32).ceil() as i32;
            // println!("time: {}, terj: {}, count: {}", time, terjedelem, count);
            // for _ in 0..count {
            //     match idays.first() {
            //         Some(iday) => {
            //             let iday = *iday;
            //             println!("vasar: {:?}, vdays: {:?}, iday: {}, idays: {:?}", vasar, vdays, iday, idays);
            //             let vday = vdays.iter().filter(|&x| *x <= iday).max().unwrap();
            //             let hash = vasar.get_mut(&vday).unwrap();
            //             hash.insert(ingredient.clone(), true);
            //             idays.retain(|jday| *jday >= vday + time)
            //         },
            //         None => break,
            //     }
            // }
        }


        //println!("{:?}", vasar.get(&1));

        for meal in meals.iter() {
            let day = meal.day.as_day().clone();
            println!("meal.recipe: {}, recipe: {}", meal.recipe, self.recipes[0].name);
            let recipe = match self.recipes.get_recipe(&meal.recipe) {
                Some(recipe) => recipe,
                None => {
                    return Some(format!("Nem található ilyen recept: {}, étkezés ideje: {}", meal.recipe, meal.day.to_string()))
                }
            };
            for ingredient in recipe.ingredients.iter() {
                println!("vasar: {:?}, name: {}, res: {:?}", vasar, ingredient.name, get_shopping_days(&vasar, &ingredient.name));
                let vday = match get_shopping_days(&vasar, &ingredient.name).iter().filter(|&x| *x <= day).max() {
                    Some(vday) => vday.clone(),
                    None => {
                        return Some(format!("Hiba a kódban! Elméletileg van vásárnap"));
                    }
                };
                let hash = self.matrix.entry(ShopDay::Day(vday)).or_insert(HashMap::new());
                let to = match self.osszetevok.by_name(&ingredient.name) {
                    Some(osszetevo) => &osszetevo.unit,
                    None => {
                        return Some(format!("Összetevő nem található: {}", ingredient.name));
                    }
                };
                let converted = match ingredient.convert(&to, &self.convs) {
                    Some(quantity) => quantity,
                    None => {
                        return Some(format!("Nem lehet {}-t {}-ra/re átváltani.", ingredient.unit, to));
                    }
                };
                let sub = Sub{
                    quantity: converted * meal.number as f64 / recipe.number as f64, 
                    recipe: meal.recipe.clone(), 
                    number: meal.number
                };
                hash.entry(ingredient.name.clone()).or_insert(Subs::new()).push(sub);
            }
        }

        None
    }
}

pub type TervContext = Rc<RefCell<Terv>>;

fn get_shopping_days(vasar: &HashMap<Time, HashMap<String, bool>>, ingredient: &str) -> Vec<Time> {
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
        osszetevok: Osszetevok (vec![
            Osszetevo {
                name: String::from("aaa"), 
                unit: String::from("m"), 
                time: ShopDay::Day(Time::from_str("1").unwrap()), 
                unit_price: 100.0
            },
            Osszetevo {
                name: String::from("bbb"), 
                unit: String::from("m"), 
                time: ShopDay::Day(Time::from_str("2").unwrap()), 
                unit_price: 200.0
            },
            Osszetevo {
                name: String::from("ccc"), 
                unit: String::from("m"), 
                time: ShopDay::Day(Time::from_str("3").unwrap()), 
                unit_price: 300.0
            }
        ]),
        recipes: Recipes (vec![Recipe {
            name: String::from("alma"),
            number: 10,
            ingredients: vec![
                Ingredient {
                    name: String::from("aaa"),
                    quantity: 1.0,
                    unit: String::from("m"),
                },
                Ingredient {
                    name: String::from("bbb"),
                    quantity: 1.0,
                    unit: String::from("m"),
                }
            ],
        }]),
        meals: Meals(vec![
            Meal {
            recipe: String::from("alma"),
            number: 20,
            day: ShopDay::Day(Time::from_str("1").unwrap())
            },
            Meal {
                recipe: String::from("alma"),
                number: 10,
                day: ShopDay::Day(Time::from_str("2").unwrap())
            }
        ]),
        shoppingdays: Shoppings(vec![
            Shopping {
                day: ShopDay::Day(Time::from_str("1").unwrap()),
                name: String::from("egy"),
            },
            // Shopping {
            //     day: ShopDay::Day(Time::from_str("2").unwrap()),
            //     name: String::from("kettő"),
            // }
        ]),
        matrix: Matrix::new(),
        convs: Conversations(vec![
            Conversation{ from: "g".to_string(), to: "kg".to_string(), factor: 0.001},
            Conversation{ from: "dkg".to_string(), to: "kg".to_string(), factor: 0.01},
            Conversation{ from: "ml".to_string(), to: "l".to_string(), factor: 0.001},
            Conversation{ from: "dl".to_string(), to: "l".to_string(), factor: 0.1},
            Conversation{ from: "cl".to_string(), to: "l".to_string(), factor: 0.01},
        ]),
        error: None,
        version: 0,
    };

    if let Some(err) = terv.calculate_matrix() {
        panic!("{}", err);
    }

    print!("matrix: {:#?}", terv.matrix);
    panic!("siker");
}