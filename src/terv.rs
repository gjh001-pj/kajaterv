use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use gloo::console::log;

use crate::beszer::{BeszerLista, BeszerListak, Item};
use crate::convert::{Conversation, Conversations, Convert};
use crate::recipe::subrecipe::SubRecipes;
use crate::recipe::{Recipes, Recipe, ingredient::Ingredient};
use crate::osszetevok::{self, Osszetevo, Osszetevok};
use crate::meal::{Meal, Meals, CommonMeal, TroupMeal};
use crate::backend::matrix::{Matrix, Subs, Sub};
use crate::shop::{Shoppings, Shopping, ShopDay};
use crate::backend::time::Time;
use crate::beszer::display::{format_quantities, format_prices, format_quantities2};
use crate::troop::{Troops, Troop};

pub mod display;

#[derive(PartialEq, Clone, Debug)]
pub struct Terv {
    pub osszetevok: Osszetevok,
    pub recipes: Recipes,
    pub meals: Meals,
    pub shoppingdays: Shoppings,
    pub matrix: Matrix,
    pub beszerek: BeszerListak,
    pub convs: Conversations,
    pub troops: Troops,
    pub error: Option<String>,
    pub version: u64,
}

impl Terv {
    pub fn new() -> Self {
        Terv {
            osszetevok: Osszetevok(vec![Osszetevo::new()]),
            recipes: Recipes(vec![Recipe::new()]),
            meals: Meals(vec![Meal::default()]),
            shoppingdays: Shoppings(vec![Shopping::new()]),
            matrix: Matrix::new(),
            beszerek: BeszerListak(Vec::new()),
            convs: Conversations(vec![
                // Conversation{ from: "g".to_string(), to: "kg".to_string(), factor: 0.001},
                // Conversation{ from: "dkg".to_string(), to: "kg".to_string(), factor: 0.01},
                // Conversation{ from: "ml".to_string(), to: "l".to_string(), factor: 0.001},
                // Conversation{ from: "dl".to_string(), to: "l".to_string(), factor: 0.1},
                // Conversation{ from: "cl".to_string(), to: "l".to_string(), factor: 0.01},
            ]),
            troops: Troops::default(),
            error: None,
            version: 0,
        }
    }

    pub fn clone_pure(&self) -> Self {
        Self {
            osszetevok: Osszetevok(self.osszetevok.iter().filter(|&v| v != &Osszetevo::new()).cloned().collect::<Vec<_>>()),
            recipes: Recipes(self.recipes.iter().filter(|&v| v != &Recipe::new()).cloned().collect::<Vec<_>>()),
            meals: Meals(self.meals.iter().filter(|&v| v != &Meal::default()).cloned().collect::<Vec<_>>()),
            shoppingdays: Shoppings(self.shoppingdays.iter().filter(|&v| v != &Shopping::new()).cloned().collect::<Vec<_>>()),
            matrix: Matrix::new(),
            beszerek: BeszerListak::new(),
            convs: Conversations(self.convs.iter().filter(|&v| v != &Conversation::new()).cloned().collect::<Vec<_>>()),
            troops: Troops(self.troops.iter().filter(|&v| v != &Troop::default()).cloned().collect()),
            error: self.error.clone(),
            version: self.version,
            
        }
    }

    pub fn pure(&mut self) {
        self.osszetevok.retain(|v| v != &Osszetevo::new());
        self.recipes.retain(|v| v != &Recipe::new());
        self.meals.retain(|v| v != &Meal::default());
        self.shoppingdays.retain(|v| v != &Shopping::new());
        self.matrix = Matrix::new();
        self.beszerek = BeszerListak::new();
        self.convs.retain(|v| v != &Conversation::new());
        self.error = None;
        self.version = self.version;
    }

    pub fn make_beszerek(&mut self) -> Option<String> {
        if let Some(err) = self.calculate_matrix() {
            return Some(err);
        };
        if let Some(err) = self.matrix_to_beszerek() {
            return Some(err);
        }
        None
    }

    pub fn matrix_to_beszerek(&mut self) -> Option<String> {
        self.beszerek.clear();
        for (shopday, raw_items) in self.matrix.iter() {
            let mut recipes: Vec<&String> = raw_items.values().map(|subs| {
                subs.iter().map(|sub| {
                    &sub.recipe
                })
            }).flatten().collect();
            recipes.sort();
            recipes.dedup();

            let format_recipes = recipes.iter().enumerate().map(|(index, recipe)| {
                format!("{}. {}", index + 1, recipe)
            }).collect::<Vec<String>>().join(", ");

            let items: Vec<Item> = raw_items.iter().map(|(osszetevo_name, subs)| {
                let osszetevo = self.osszetevok.by_name(osszetevo_name).unwrap();
                
                let name = format!("{} ({:.2})", osszetevo_name, osszetevo.unit_price);
                //let name = format!("{}", osszetevo_name);
                
                let recipes = subs.iter().map(|sub| {
                    format!("{} ({})", recipes.iter().position(|&recipe| *recipe == sub.recipe).unwrap() + 1, sub.number)
                }).collect::<Vec<String>>().join(", ");
                
                let quantities = format_quantities(subs, &osszetevo.unit);
                //let quantities = format_quantities2(subs, &osszetevo.unit);

                let prices = format_prices(subs);

                Item {
                    name,
                    recipes,
                    quantities,
                    prices,
                }
            }).collect();

            self.beszerek.push(BeszerLista {
                name: self.shoppingdays.get_by_day(shopday).expect("innen eredt").name.clone(),
                time: shopday.to_string(),
                recipes: format_recipes,
                items,
            })
        }

        self.beszerek.sort_by_key(|beszerlista| beszerlista.time.parse::<Time>());

        None
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
                        if let ShopDay::Name(meal_day) = &meal.as_common().day{
                            if meal_day == name {
                                meals.remove(index);
                                let hash = self.matrix.entry(meal.as_common().day.clone()).or_insert(HashMap::new());
                                let recipe = match self.recipes.get_recipe(&meal.as_common().recipe) {
                                    Some(v) => v,
                                    None => {
                                        return Some(format!("Nem található ilyen recept: {}, étkezés ideje: {}", meal.as_common().recipe, meal.as_common().day.to_string()));
                                    }
                                };
                                for ingredient in recipe.ingredients.iter() {
                                    let osszetevo = match self.osszetevok.by_name(&ingredient.name) {
                                        Some(osszetevo) => osszetevo,
                                        None => {
                                            return Some(format!("Összetevő nem található: {}", ingredient.name));
                                        }
                                    };
                                    let converted = match ingredient.convert(&osszetevo.unit, &self.convs) {
                                        Some(quantity) => quantity,
                                        None => {
                                            return Some(format!("Nem lehet {}-t {}-ra/re átváltani.", ingredient.unit, osszetevo.unit));
                                        }
                                    };
                                    let quantity = converted * meal.as_common().number as f64 / recipe.number as f64;
                                    let sub = Sub {
                                        quantity, 
                                        price: osszetevo.unit_price * quantity,
                                        recipe: recipe.name.clone(), 
                                        number: meal.as_common().number
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
            let day = meal.as_common().day.as_day().clone();
            let recipe = match self.recipes.get_recipe(&meal.as_common().recipe) {
                Some(v) => v,
                None => {
                    return Some(format!("Nem található ilyen recept: {}, étkezés ideje: {}", meal.as_common().recipe, meal.as_common().day.to_string()))
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
            println!("time: {}", time);
            loop {
                println!("idays: {:?}", idays);
                match idays.first() {
                    Some(iday) => {
                        let vday = match vdays.iter().filter(|&x| x <= iday).max() {
                            Some(vday) => vday,
                            None => {
                                return Some(format!("Nem lehet venni {}-t, az alábbi napokra: {:?}, elállási idő: {}, vásárnapok: {:?}",
                                    ingredient, idays, time, vdays));
                            }
                        };
                        println!("vday: {:?}", vday);
                        let hash = vasar.get_mut(vday).expect("Hiba a kódban, korábban az összes vday-t belepakoltuk vasar-ba.");
                        hash.insert(ingredient.clone(), true);
                        if *idays.iter().min().expect("A match-nál ellenőriztük, hogy van a listának első eleme.") >= *vday + *time {
                            return Some(format!("nem lehet megvenni {} összetevőt ekkorra: {}, előtte lévő vásárlás: {}, elállási idő: {}",
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
            let day = meal.as_common().day.as_day().clone();
            println!("meal.recipe: {}, recipe: {}", meal.as_common().recipe, self.recipes.get(0).unwrap().name);
            let recipe = match self.recipes.get_recipe(&meal.as_common().recipe) {
                Some(recipe) => recipe,
                None => {
                    return Some(format!("Nem található ilyen recept: {}, étkezés ideje: {}", meal.as_common().recipe, meal.as_common().day.to_string()))
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
                let osszetevo = match self.osszetevok.by_name(&ingredient.name) {
                    Some(osszetevo) => osszetevo,
                    None => {
                        return Some(format!("Összetevő nem található: {}", ingredient.name));
                    }
                };
                let converted = match ingredient.convert(&osszetevo.unit, &self.convs) {
                    Some(quantity) => quantity,
                    None => {
                        return Some(format!("Nem lehet {}-t {}-ra/re átváltani.", ingredient.unit, osszetevo.unit));
                    }
                };
                let quantity = converted * meal.as_common().number as f64 / recipe.number as f64;
                let sub = Sub {
                    quantity, 
                    price: osszetevo.unit_price * quantity,
                    recipe: meal.as_common().recipe.clone(), 
                    number: meal.as_common().number
                };
                hash.entry(ingredient.name.clone()).or_insert(Subs::new()).push(sub);
            }
        }

        None
    }
}

// pub type TervContext = Rc<RefCell<Terv>>;

#[derive(PartialEq, Clone, Debug)]
pub struct AppData {
    pub terv: RefCell<Terv>,
    pub old_terv: RefCell<Terv>,
}

impl AppData {
    pub fn new() -> Self {
        let terv = Terv::new();
        Self {
            old_terv: RefCell::new(terv.clone()),
            terv: RefCell::new(terv),
        }
    }
}

pub type AppContext = Rc<AppData>;

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
    let mut terv = Terv {
        osszetevok: Osszetevok (vec![
            Osszetevo {
                name: String::from("aaa"), 
                unit: String::from("m"), 
                time: ShopDay::Day("1".parse().unwrap()), 
                unit_price: 100.0
            },
            Osszetevo {
                name: String::from("bbb"), 
                unit: String::from("m"), 
                time: ShopDay::Day("2".parse().unwrap()), 
                unit_price: 200.0
            },
            Osszetevo {
                name: String::from("ccc"), 
                unit: String::from("m"), 
                time: ShopDay::Day("3".parse().unwrap()), 
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
            ].into(),
            sub_recipes: SubRecipes::default(),
        }]),
        meals: Meals(vec![
            Meal::Common(CommonMeal {
            recipe: String::from("alma"),
            number: 20,
            day: ShopDay::Day("1".parse().unwrap())
            }),
            Meal::Common(CommonMeal {
                recipe: String::from("alma"),
                number: 10,
                day: ShopDay::Day("2".parse().unwrap())
            }),
        ]),
        shoppingdays: Shoppings(vec![
            Shopping {
                day: ShopDay::Day("1".parse().unwrap()),
                name: String::from("egy"),
            },
            Shopping {
                day: ShopDay::Day("2".parse().unwrap()),
                name: String::from("kettő"),
            }
        ]),
        matrix: Matrix::new(),
        beszerek: BeszerListak(Vec::new()),
        convs: Conversations(vec![
            Conversation{ from: "g".to_string(), to: "kg".to_string(), factor: 0.001},
            Conversation{ from: "dkg".to_string(), to: "kg".to_string(), factor: 0.01},
            Conversation{ from: "ml".to_string(), to: "l".to_string(), factor: 0.001},
            Conversation{ from: "dl".to_string(), to: "l".to_string(), factor: 0.1},
            Conversation{ from: "cl".to_string(), to: "l".to_string(), factor: 0.01},
        ]),
        troops: Troops::default(),
        error: None,
        version: 0,
    };

    if let Some(err) = terv.make_beszerek() {
        panic!("{}", err);
    }

    print!("beszerek: {:#?}", terv.beszerek);
    //panic!("siker");
}