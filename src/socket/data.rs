use serde::{Serialize, Deserialize};
use gloo::console::log;
use std::str::FromStr;

use crate::backend::time::Time;
use crate::convert::Conversation;
use crate::recipe::subrecipe::{SubRecipe, SubRecipes};
use crate::recipe::{Recipe, Recipes};
use crate::shop::{ShopDay, Shopping, Shoppings};
use crate::recipe::ingredient::{Ingredient, Ingredients};
use crate::terv::Terv;
use crate::osszetevok::{Osszetevo, Osszetevok};
use crate::meal::{CommonMeal, Meal, Meals};
use crate::troop::*;
use crate::troop::sensitive::Sensitive;

pub mod com {
    pub const OSSZ : u8 = 0b0000_0001;
    pub const REC  : u8 = 0b0000_0010;
    pub const MEAL : u8 = 0b0000_0100;
    pub const SHOP : u8 = 0b0000_1000;
    pub const BESZ : u8 = 0b0001_0000;
    pub const CONV : u8 = 0b0010_0000;
    pub const TROOP: u8 = 0b0100_0000;
    pub const ALL  : u8 = 0b0111_1111;
    //pub const SEND: u8 = 0b100_0000;
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Data {
    pub fields: u8,
    pub osszetevok: String,
    pub recipes: String,
    pub meals: String,
    pub shoppings: String,
    pub beszer: String,
    pub conv: String,
    pub troops: String,
}

impl Data {
    pub fn new() -> Self {
        Self {
            fields: 0,
            osszetevok: String::new(),
            recipes: String::new(),
            meals: String::new(),
            shoppings: String::new(),
            beszer: String::new(),
            conv: String::new(),
            troops: String::new(),
        }
    }

    pub fn convert_string(&mut self, terv: &Terv, command: u8) {
        self.fields = command;
        if command & com::OSSZ > 0 {
            self.convert_string_ossz(terv);
        }
        if command & com::REC > 0 {
            self.convert_string_rec(terv);
        }
        if command & com::MEAL > 0 {
            self.convert_string_meal(terv);
        }
        if command & com::SHOP > 0 {
            self.convert_string_shop(terv);
        }
        if command & com::BESZ > 0 {
            self.convert_string_besz(terv);
        }
        if command & com::CONV > 0 {
            self.convert_string_conv(terv);
        }
        if command & com::TROOP > 0 {
            self.convert_string_troop(terv);
        }
    }

    pub fn convert_string_ossz(&mut self, terv: &Terv) {
        self.osszetevok = terv.osszetevok.iter().map(|ossz| {
            format!("{}\t{}\t{}\t{}", ossz.name, ossz.unit, ossz.time.to_string(), ossz.unit_price.to_string().replace(".", ","))
        }).collect::<Vec<String>>().join("\n");
    }
    pub fn convert_string_rec(&mut self, terv: &Terv) {
        let max_len = terv.recipes.iter()
            .map(|x| x.ingredients.len() + x.sub_recipes.len()).max().unwrap_or(0);
        self.recipes = (0..max_len + 1).map(|row| {
            terv.recipes.iter().map(|recipe| {
                if row == 0 {
                    return format!("{}\t{}\t\t\t", recipe.name, recipe.number);
                } else {
                    if recipe.sub_recipes.len() > 0 {
                       if row == 1 {
                           return String::from("subrecipes\t\t\t\t");
                       } else if let Some(sub_rec) = recipe.sub_recipes.get(row - 2) {
                           return format!("{}\t{}\t\t\t", sub_rec.name, sub_rec.scale);
                       }
                   }
                   if recipe.ingredients.len() > 0 {
                        let rows_above = if recipe.sub_recipes.len() > 0 {recipe.sub_recipes.len() + 2} else {1};
                        if row == rows_above {
                            return String::from("ingredients\t\t\t\t");
                        } else if let Some(ingredient) = recipe.ingredients.get(row - rows_above - 1) {
                            return format!("{}\t{}\t{}\t\t", ingredient.name, ingredient.quantity.to_string().replace(".", ","), ingredient.unit);
                        }
                   }
                   return String::from("\t\t\t\t");
                }
            }).collect::<Vec<_>>().join("")

            // if row == 0 {
            //     terv.recipes.iter().map(|recipe| {
            //         format!("{}\t{}\t\t\t", recipe.name, recipe.number)
            //     }).collect::<Vec<String>>().join("")
            // } else {
            //     terv.recipes.iter().map(|recipe| {
            //         if let Some(ingredient) = recipe.ingredients.get(row - 1) {
            //             format!("{}\t{}\t{}\t\t", ingredient.name, ingredient.quantity.to_string().replace(".", ","), ingredient.unit)
            //         } else {
            //             String::from("\t\t\t\t")
            //         }
            //     }).collect::<Vec<String>>().join("")
            // }
        }).collect::<Vec<String>>().join("\n");
    }
    pub fn convert_string_meal(&mut self, terv: &Terv) {
        self.meals = terv.meals.iter().map(|meal| {
            if let Meal::Common(meal) = meal {
                format!("{}\t{}\t{}", meal.recipe, meal.number, meal.day.to_string())
            } else {
                todo!()
            }
        }).collect::<Vec<String>>().join("\n");
    }
    pub fn convert_string_shop(&mut self, terv: &Terv) {
        self.shoppings = terv.shoppingdays.iter().map(|shopping| {
            format!("{}\t{}", shopping.name, shopping.day.to_string())
        }).collect::<Vec<String>>().join("\n");
    }
    pub fn convert_string_besz(&mut self, terv: &Terv) {
        if terv.beszerek.len() < 1 { self.beszer = String::new(); return }

        let max_item_cout = terv.beszerek.iter()
        .map(|beszer| { beszer.items.len() }).max().unwrap();

        self.beszer = (0..max_item_cout + 2).map(|row| {
            if row == 0 {
                terv.beszerek.iter().map(|beszer| {
                    format!("{}\t{}\t{}\t\t\t", beszer.name, beszer.time, beszer.recipes)
                }).collect::<Vec<_>>().join("")
            } else if row == 1 {
                (0..terv.beszerek.len()).map(|_| {
                    format!("{}\t{}\t{}\t{}\t\t",
                        "név (egységár)", 
                        "recept (létszám)", 
                        "[részeredmény (/fő)], mennyiség (/fő) mértékegység", 
                        "[részeredmény (/fő)], ár (/fő)")
                }).collect::<Vec<_>>().join("")
            } else {
                terv.beszerek.iter().map(|beszer| {
                    if let Some(item) = beszer.items.get(row - 2) {
                        format!("{}\t{}\t{}\t{}\t\t", item.name, item.recipes, item.quantities.replace(".", ","), item.prices)
                    } else {
                        String::from("\t\t\t\t\t")
                    }
                }).collect::<Vec<_>>().join("")
            }
        }).collect::<Vec<_>>().join("\n");
    }
    pub fn convert_string_conv(&mut self, terv: &Terv) {
        self.conv = terv.convs.iter().map(|conv| {
            format!("{}\t{}\t{}", conv.from, conv.to, conv.factor.to_string().replace(".", ","))
        }).collect::<Vec<_>>().join("\n");
    }
    pub fn convert_string_troop(&mut self, terv: &Terv) {
        let max_len = terv.troops.iter()
            .map(|x| x.sensitives.len()).max().unwrap();
        self.troops = (0..max_len + 1).map(|row| {
            if row == 0 {
                terv.troops.iter().map(|troop| {
                    format!("{}\t\t\t\t", troop.name)
                }).collect::<Vec<String>>().join("")
            } else {
                terv.troops.iter().map(|troop| {
                    if let Some(sensitive) = troop.sensitives.get(row - 1) {
                        format!("{}\t{}\t{}\t\t", sensitive.name, 
                            sensitive.sensitivities.to_string(), sensitive.actions.to_string())
                    } else {
                        String::from("\t\t\t\t")
                    }
                }).collect::<Vec<String>>().join("")
            }
        }).collect::<Vec<String>>().join("\n");
    }

    pub fn convert_data(&self, terv: &mut Terv){
        if self.fields & com::OSSZ > 0 {
            self.convert_data_ossz(terv);
        }
        if self.fields & com::REC > 0 {
            self.convert_data_rec(terv);
        }
        if self.fields & com::MEAL > 0 {
            self.convert_data_meal(terv);
        }
        if self.fields & com::SHOP > 0 {
            self.convert_data_shop(terv);
        }
        if self.fields & com::BESZ > 0 {}
        if self.fields & com::CONV > 0 {
            self.convert_data_conv(terv);
        }
        if self.fields & com::TROOP > 0 {
            self.convert_data_troop(terv);
        }
    }

    pub fn convert_data_ossz(&self, terv: &mut Terv) {
        if self.osszetevok == "" { return; }
        //log!("osszetevok: ", &self.osszetevok);

        terv.osszetevok.0 = self.osszetevok.split("\n").map(|row| {
            let cells: Vec<&str> = row.split("\t").collect();
            //log!(format!("cells: {:?}", cells));
            Osszetevo {
                name: cells[0].to_string(),
                unit: cells[1].to_string(),
                time: match cells[2].parse() {
                    Ok(t) => ShopDay::Day(t),
                    Err(_) => ShopDay::Name(cells[2].to_string()),
                },
                unit_price: cells[3].replace(",", ".").parse().unwrap_or(0.0),
            }
        }).collect();
    }
    pub fn convert_data_rec(&self, terv: &mut Terv) {
        if self.recipes == "" { return; }

        let rows: Vec<&str> = self.recipes.split("\n").collect();
        let mut recipes = Vec::new(); // (0..rows[0].split("\t").count() / 4).map(|_| Recipe::new()).collect();
        let mut next_sub_recipe = false;
        for (ing_index, row) in rows.iter().enumerate() {
            for (rec_index, slice) in row.split("\t").collect::<Vec<&str>>().chunks(4).enumerate() {
                if ing_index == 0 {
                    recipes.push(Recipe {
                        name: slice[0].to_string(),
                        number: slice[1].parse().unwrap_or(0),
                        ingredients: Ingredients::default(),
                        sub_recipes: SubRecipes::default(),
                    })
                } else {
                    if slice[0] == "" { continue; }
                    if slice[0] == "subrecipes" {
                        next_sub_recipe = true;
                        continue;
                    }
                    if slice[0] == "ingredients" {
                        next_sub_recipe = false;
                        continue;
                    }
                    let recipe = recipes.get_mut(rec_index).unwrap();
                    if next_sub_recipe == true {
                        recipe.sub_recipes.push(SubRecipe {
                            name: slice[0].to_string(),
                            scale: slice[1].parse().unwrap_or(0.0),
                        });
                    } else {
                        recipe.ingredients.push(Ingredient {
                            name: slice[0].to_string(),
                            quantity: slice[1].replace(",", ".").parse().unwrap_or(0.0),
                            unit: slice[2].to_string(),
                        });
                    }
                }
            }
        }
        terv.recipes = Recipes(recipes);
    }
    pub fn convert_data_meal(&self, terv: &mut Terv) {
        if self.meals == "" { return; }
        terv.meals.0 = self.meals.split("\n").map(|row| {
            let cells: Vec<&str> = row.split("\t").collect();
            Meal::Common(CommonMeal {
                recipe: cells[0].to_string(),
                number: cells[1].parse().unwrap_or(0),
                day: ShopDay::from(cells[2]),
            })
        }).collect();
    }
    pub fn convert_data_shop(&self, terv: &mut Terv) {
        if self.shoppings == "" { return; }

        terv.shoppingdays.0 = self.shoppings.split("\n").map(|row| {
            let cells: Vec<&str> = row.split("\t").collect();
            Shopping {
                name: cells[0].to_string(),
                day: ShopDay::from(cells[1]),
            }
        }).collect();
    }

    pub fn convert_data_conv(&self, terv: &mut Terv) {
        if self.conv == "" { return; }

        terv.convs.0 = self.conv.split("\n").map(|row| {
            let cells: Vec<_> = row.split("\t").collect();
            Conversation {
                from: cells[0].to_string(),
                to: cells[1].to_string(),
                factor: cells[2].replace(",", ".").parse().unwrap_or(1.0),
            }
        }).collect();
    }
    pub fn convert_data_troop(&self, terv: &mut Terv) {
        if self.troops == "" { return; }

        let rows: Vec<&str> = self.troops.split("\n").collect();
        let mut troops = Vec::new(); // (0..rows[0].split("\t").count() / 4).map(|_| Recipe::new()).collect();
        for (sens_index, row) in rows.iter().enumerate() {
            for (troop_index, slice) in row.split("\t").collect::<Vec<&str>>().chunks(4).enumerate() {
                if sens_index == 0 {
                    troops.push(Troop {
                        name: slice[0].to_string(),
                        sensitives: Vec::new().into(),
                    })
                } else {
                    if slice[0] == "" { continue; }
                    let troop = &mut troops[troop_index];
                    troop.sensitives.push(Sensitive {
                        name: slice[0].to_string(),
                        sensitivities: slice[1].into(),
                        actions: slice[2].into(),
                    })
                }
            }
        }
        terv.troops = troops.into();
    }

}

#[test]
fn test3() {
    //panic!("n: {:?}", String::from("").split("\n").collect::<Vec<&str>>());
}