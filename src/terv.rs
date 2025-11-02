use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use gloo::console::log;

use crate::beszer::{BeszerLista, BeszerListak, Item};
use crate::convert::{Conversation, Conversations, Convert};
use crate::ew::{EW, Err};
use crate::meal::MealType;
use crate::meal::sensitive::{CaseType, SensitiveCases};
use crate::recipe::subrecipe::SubRecipes;
use crate::recipe::SensMode;
use crate::recipe::{Recipes, Recipe, ingredient::Ingredient};
use crate::osszetevok::{self, Osszetevo, Osszetevok};
use crate::meal::{Meal, Meals, common::CommonMeal, troup::TroupMeal};
use crate::backend::matrix::{Matrix, Subs, Sub};
use crate::shop::{Shoppings, Shopping, ShopDay};
use crate::backend::time::Time;
use crate::beszer::display::{format_quantities, format_prices, format_quantities2};
use crate::socket::data::com;
use crate::troop::sensitive::Sensitives;
use crate::troop::sensitive::sensitivity::Sensitivities;
use crate::troop::{Troops, Troop};
use crate::ew::EWs;

pub mod display;

#[derive(PartialEq, Clone, Debug)]
pub struct Terv {
    pub osszetevok: Osszetevok,
    pub recipes: Recipes,
    pub meals: Meals,
    pub shoppings: Shoppings,
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
            osszetevok: Osszetevok(vec![Osszetevo::default()]),
            recipes: Recipes(vec![Recipe::default()]),
            meals: Meals(vec![Meal::default()]),
            shoppings: Shoppings(vec![Shopping::default()]),
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
            osszetevok: Osszetevok(self.osszetevok.iter().filter(|&v| v != &Osszetevo::default()).cloned().collect::<Vec<_>>()),
            recipes: Recipes(self.recipes.iter().filter(|&v| v != &Recipe::default()).cloned().collect::<Vec<_>>()),
            meals: Meals(self.meals.iter().filter(|&v| v != &Meal::default()).cloned().collect::<Vec<_>>()),
            shoppings: Shoppings(self.shoppings.iter().filter(|&v| v != &Shopping::new()).cloned().collect::<Vec<_>>()),
            matrix: Matrix::new(),
            beszerek: BeszerListak::new(),
            convs: Conversations(self.convs.iter().filter(|&v| v != &Conversation::new()).cloned().collect::<Vec<_>>()),
            troops: Troops(self.troops.iter().filter(|&v| v != &Troop::default()).cloned().collect()),
            error: self.error.clone(),
            version: self.version,
            
        }
    }

    pub fn pure(&mut self) {
        self.osszetevok.retain(|v| v != &Osszetevo::default());
        self.recipes.retain(|v| v != &Recipe::default());
        self.meals.retain(|v| v != &Meal::default());
        self.shoppings.retain(|v| v != &Shopping::new());
        self.matrix = Matrix::new();
        self.beszerek = BeszerListak::new();
        self.convs.retain(|v| v != &Conversation::new());
        self.error = None;
        self.version = self.version;
    }

    pub fn make_beszerek(&mut self) -> Option<String> {
        match self.calculate_matrix().map(|ew| ew.to_string()) {
            None => (), 
            other => return other,
        };
        match self.matrix_to_beszerek().map(|ew| ew.to_string()) {
            None => (), 
            other => return other,
        };
        None
        // if let Some(err) = self.calculate_matrix() {
        //     return Some(err);
        // };
        // if let Some(err) = self.matrix_to_beszerek() {
        //     return Some(err);
        // }
        // None
    }

    pub fn matrix_to_beszerek(&mut self) -> Option<EW<Err>> {
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
                name: self.shoppings.get_by_day(shopday).expect("innen eredt").name.clone(),
                time: shopday.to_string(),
                recipes: format_recipes,
                items,
            })
        }

        self.beszerek.sort_by_key(|beszerlista| beszerlista.time.parse::<Time>());

        //log!(format!("{:#?}", self.beszerek));

        None
    }

    pub fn handle_name_shopday(&mut self, meals: &mut Meals, shopday_name: &str) -> Option<EW<Err>> {
        let mut shopday_errs: EWs<Err> = EWs::new();
        for index in 0..meals.len() {
            let meal = &meals[index];
            if let ShopDay::Name(meal_day) = &meal.day {
                if meal_day != shopday_name {
                    continue;
                }
                let hash = self.matrix.entry(meal.day.clone()).or_insert(HashMap::new());
                
                // -------------------
                let mut meal_errs: EWs<Err> = EWs::new();
                match &meal.ty {
                    MealType::Common(common) => {
                        let recipe = match self.recipes.get_recipe(&common.main_recipe) {
                            Some(v) => v,
                            None => {
                                shopday_errs.push(EW::from(format!("Nem található ilyen recept: {}, étkezés ideje: {}", &common.main_recipe, meal.day)));
                                continue;
                            }
                        };

                        // there is no sensitivity check in name-ShopDayed meals

                        for ingredient in recipe.ingredients.iter() {
                            let osszetevo = match self.osszetevok.by_name(&ingredient.name) {
                                Some(v) => v,
                                None => {
                                    meal_errs.push(EW::from(format!("Összetevő nem található: {}", ingredient.name)));
                                    continue;
                                }
                            };
                            let converted = match ingredient.convert(&osszetevo.unit, &self.convs) {
                                Some(quantity) => quantity,
                                None => {
                                    meal_errs.push(EW::from(format!("Nem lehet {} összetevőt {}-ról/ről {}-ra/re átváltani.", ingredient.name, ingredient.unit, osszetevo.unit)));
                                    continue;
                                }
                            };
                            let quantity = converted * common.number as f64 / recipe.number as f64;
                            let sub = Sub {
                                quantity, 
                                price: osszetevo.unit_price * quantity,
                                recipe: recipe.name.clone(), 
                                number: common.number
                            };
                            hash.entry(ingredient.name.clone()).or_insert(Subs::new()).push(sub);
                        }
                        shopday_errs.push_opt(meal_errs.to_child(common.main_recipe.clone()));//EW::Child { from: common.main_recipe.clone(), ews: meal_errs });
                    },
                    MealType::Troup(troup) => todo!(),
                }

                // -------------------
                
                meals.remove(index);
            }
        }

        shopday_errs.to_child(shopday_name.to_owned())
    }

    pub fn calculate_matrix(&mut self) -> Option<EW<Err>> {//   Option<String> {
        let mut matrix_errs: EWs<Err> = EWs::new();
        let mut meals = self.meals.clone();

        self.matrix.clear();

        let mut vasar_days: Vec<Time> = Vec::new();

        // separate ShopDay::Name, list vasar napok
        let mut name_shopday_errs: EWs<Err> = EWs::new();

        for shopping in self.shoppings.clone().iter() {
            match &shopping.day {
                ShopDay::Day(day) => vasar_days.push(*day),
                ShopDay::Name(name) => {
                    name_shopday_errs.push_opt(self.handle_name_shopday(&mut meals, name));
                    // for (index, meal) in meals.clone().iter().enumerate() {
                    //     if let ShopDay::Name(meal_day) = &meal.as_common().day{
                    //         if meal_day == name {
                    //             meals.remove(index);
                    //             let hash = self.matrix.entry(meal.as_common().day.clone()).or_insert(HashMap::new());
                    //             let recipe = match self.recipes.get_recipe(&meal.as_common().main_recipe) {
                    //                 Some(v) => v,
                    //                 None => {
                    //                     return Some(format!("Nem található ilyen recept: {}, étkezés ideje: {}", meal.as_common().main_recipe, meal.as_common().day.to_string()));
                    //                 }
                    //             };
                    //             for ingredient in recipe.ingredients.iter() {
                    //                 let osszetevo = match self.osszetevok.by_name(&ingredient.name) {
                    //                     Some(osszetevo) => osszetevo,
                    //                     None => {
                    //                         return Some(format!("Összetevő nem található: {}", ingredient.name));
                    //                     }
                    //                 };
                    //                 let converted = match ingredient.convert(&osszetevo.unit, &self.convs) {
                    //                     Some(quantity) => quantity,
                    //                     None => {
                    //                         return Some(format!("Nem lehet {}-t {}-ra/re átváltani.", ingredient.unit, osszetevo.unit));
                    //                     }
                    //                 };
                    //                 let quantity = converted * meal.as_common().number as f64 / recipe.number as f64;
                    //                 let sub = Sub {
                    //                     quantity, 
                    //                     price: osszetevo.unit_price * quantity,
                    //                     recipe: recipe.name.clone(), 
                    //                     number: meal.as_common().number
                    //                 };
                    //                 hash.entry(ingredient.name.clone()).or_insert(Subs(vec![sub.clone()])).push(sub);
                    //             }
                    //     }}
                    // }
                }
            }
        }
        matrix_errs.push_opt(name_shopday_errs.to_child("from named ShopDays".to_string()));
        vasar_days.sort();

        // create ingredient - Times hashmap
        let mut meal_reicipe_errs: EWs<Err> = EWs::new();
        let mut ossz_vasar: HashMap<String, Vec<Time>> = HashMap::new();
        for meal in meals.iter_mut() {
            let day = meal.day.as_day();
            match &meal.ty {
                MealType::Common(common) => {
                    let recipe = match self.recipes.get_recipe(&common.main_recipe) {
                        Some(v) => v,
                        None => {
                            meal_reicipe_errs.push(EW::from(format!("Nem található ilyen recept: {}, étkezés ideje: {}", common.main_recipe, meal.day.to_string())));
                            continue;
                        }
                    };
                    for ingredient in recipe.ingredients.iter() {
                        ossz_vasar.entry(ingredient.name.clone()).or_insert(Vec::new()).push(day.clone());
                    }
                },
                MealType::Troup(troup) => todo!()
            }
        }
        matrix_errs.push_opt(meal_reicipe_errs.to_child("from not named meals".to_string()));

        // preapre time - ingredient hashmap
        let mut vasar: HashMap<Time, HashMap<String, bool>> = HashMap::new();
        for day in vasar_days.iter() {
            vasar.insert(*day, HashMap::new());
        }


        // println!("ossz_vasar: {:?}", ossz_vasar);
        // println!("vdays: {:?}", vdays);
        // vasar hashmap kitöltése
        let mut _errs: EWs<Err> = EWs::new();
        for (ingredient, szukseg_days) in ossz_vasar.iter_mut() {
            let time = match self.osszetevok.by_name(ingredient) {
                Some(ing) => match &ing.time {
                    ShopDay::Day(v) => v,
                    ShopDay::Name(name) => {
                        // handle String elállási idő
                        if let Some(hash) = self.matrix.get(&ing.time) {
                            todo!()
                        } else {
                            _errs.push(EW::from(format!("Nincs ilyen vásárnap: {} (összetevő: {})", name, ingredient)))
                        }
                        continue;
                    }
                },
                None => {
                    _errs.push(EW::from(format!("Összetevő nem található: {}", ingredient)));
                    continue;
                }
            };
            // let mut i = 0;
            let mut ing_errs: EWs<Err> = EWs::new();
            //println!("time: {}", time);
            szukseg_days.sort();
            szukseg_days.reverse();
            loop {
                //println!("szukseg_days: {:?}", szukseg_days);
                match szukseg_days.pop() {
                    Some(szukseg_day) => {
                        // legnagyobb szukseg_day-nél kisebb vasar_day
                        let vasar_day = match vasar_days.iter().filter(|&x| x <= &szukseg_day).max() {
                            Some(v) => v,
                            None => {
                                ing_errs.push(EW::from(format!("Nem lehet venni az alábbi napra: {}, elállási idő: {}, vásárnapok: {:?}",
                                szukseg_day, time, vasar_days)));
                                continue;
                            }
                        };
                        //println!("vday: {:?}", vasar_day);
                        
                        // A vasar_day-on meg tudjuk-e venni az ingredient-et (elállási idó)
                        if szukseg_day >= *vasar_day + *time {
                            ing_errs.push(EW::from(format!("Nem lehet megvenni ekkorra: {}, előtte lévő vásárlás: {}, elállási idő: {}",
                            szukseg_day, vasar_day, time)));
                            continue;
                        } else {
                            let hash = vasar.get_mut(vasar_day).expect("Hiba a kódban, korábban az összes vday-t belepakoltuk vasar-ba.");
                            hash.insert(ingredient.clone(), true);
                        }
                        szukseg_days.retain(|&jday| jday >= *vasar_day + *time);
                        
                        if *vasar_days.iter().max().expect("biztos, hogy van vasarnap") + *time < szukseg_day {
                            ing_errs.push(EW::from(format!("Nem lehet megvenni ekkorra: {}, előtte lévő vásárlás: {}, elállási idő: {}",
                                szukseg_day, vasar_day, time)));
                            break;
                        }
                    },
                    None => break,
                }
            }
            _errs.push_opt(ing_errs.to_child(ingredient.clone()));
        }
        matrix_errs.push_opt(_errs.to_child("ingredient-times -> time-ingredients".to_string()));

        //println!("{:?}", vasar.get(&1));

        // create subs for day elállási idő
        let mut meals_errs: EWs<Err> = EWs::new();
        for meal in meals.iter() {
            let day = meal.day.as_day().clone();
            //println!("meal.recipe: {}, recipe: {}", meal.as_common().main_recipe, self.recipes.get(0).unwrap().name);
            let mut meal_errs: EWs<Err> = EWs::new();
            match &meal.ty {
                MealType::Common(common) => {
                    let recipe = match self.recipes.get_recipe(&common.main_recipe) {
                        Some(r) => r,
                        None => {
                            meal_errs.push(EW::from(format!("Nem található ilyen recept: {}, étkezés ideje: {}", common.main_recipe, meal.day.to_string())));
                            continue;
                        }
                    };
                    let mut ing_errs: EWs<Err> = EWs::new();
                    for ingredient in recipe.ingredients.iter() {
                        println!("vasar: {:?}, name: {}, res: {:?}", vasar, ingredient.name, get_shopping_days(&vasar, &ingredient.name));

                        let vasar_day = match get_shopping_days(&vasar, &ingredient.name).iter().filter(|&x| *x <= day).max() {
                            Some(v) => v.clone(),
                            None => continue,
                        };
                            // .expect(&format!("ing: {}, rec: {}, day: {}, vasar: {:#?}, g_s_d: {:?}, matrix_errs: \n{}", ingredient.name, common.main_recipe, day, 
                            // vasar, get_shopping_days(&vasar, &ingredient.name), matrix_errs
                            // //"csak akkor kerül be valami a vasar-ba, ha minden jó"
                            // )).clone();

                        let hash = self.matrix.entry(ShopDay::Day(vasar_day.clone())).or_insert(HashMap::new());
                        let osszetevo = match self.osszetevok.by_name(&ingredient.name) {
                            Some(osszetevo) => osszetevo,
                            None => {
                                //return Some(format!("Összetevő nem található: {}", ingredient.name));
                                ing_errs.push(EW::from(format!("Nem található ilyen összetevő: {}", ingredient.name)));
                                continue;
                            }
                        };
                        let converted = match ingredient.convert(&osszetevo.unit, &self.convs) {
                            Some(quantity) => quantity,
                            None => {
                                //return Some(format!("Nem lehet {}-t {}-ra/re átváltani.", ingredient.unit, osszetevo.unit));
                                ing_errs.push(EW::from(format!("Nem lehet {}-t {}-ra/re átváltani. (összetevő: {})", ingredient.unit, osszetevo.unit, ingredient.name)));
                                continue;
                            }
                        };
                        let quantity = converted * common.number as f64 / recipe.number as f64;
                        let sub = Sub {
                            quantity, 
                            price: osszetevo.unit_price * quantity,
                            recipe: common.main_recipe.clone(), 
                            number: common.number
                        };
                        hash.entry(ingredient.name.clone()).or_insert(Subs::new()).push(sub);
                    }
                    meal_errs.push_opt(ing_errs.to_child(format!("étkezés ideje: {}", day)));
                },
                MealType::Troup(troup) => todo!()
            }
            meals_errs.push_opt(meal_errs.to_child(format!("from meal: {:?}", meal)));
        }

        //log!(format!("{:#?}", self.matrix));

        matrix_errs.to_child("matrix".to_owned())
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
                unit_price: 100.0,
                sens: Sensitivities::default(),
            },
            Osszetevo {
                name: String::from("bbb"), 
                unit: String::from("m"), 
                time: ShopDay::Day("2".parse().unwrap()), 
                unit_price: 200.0,
                sens: Sensitivities::default(),
            },
            Osszetevo {
                name: String::from("ccc"), 
                unit: String::from("m"), 
                time: ShopDay::Day("3".parse().unwrap()), 
                unit_price: 300.0,
                sens: Sensitivities::default(),
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
            sens_mode: SensMode::default(),
            sens: Sensitivities::default(),
        }]),
        meals: Meals(vec![
            Meal {
                day: ShopDay::Day("1".parse().unwrap()),
                ty: MealType::Common(CommonMeal {
                    number: 20,
                    main_recipe: String::from("alma"),
                    sens_cases: SensitiveCases::default(),
                })
            },
            Meal {
                day: ShopDay::Day("2".parse().unwrap()),
                ty: MealType::Common(CommonMeal {
                    number: 10,
                    main_recipe: String::from("alma"),
                    sens_cases: SensitiveCases::default(),
                })
            },
        ]),
        shoppings: Shoppings(vec![
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
        panic!("{:#?}", err);
    }

    print!("beszerek: {:#?}", terv.beszerek);
    //panic!("siker");
}