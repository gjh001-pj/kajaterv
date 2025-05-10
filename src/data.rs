use serde::{Serialize, Deserialize};
use gloo::console::log;

use crate::recipe::Recipe;
use crate::shop::{ShopDay, Shopping, Shoppings};
use crate::{recipe::Ingredient, terv::Terv};
use crate::osszetevok::{Osszetevo, Osszetevok};
use crate::meal::{Meal, Meals};

pub mod com {
    pub const OSSZ: u8 = 0b000_0001;
    pub const REC : u8 = 0b000_0010;
    pub const MEAL: u8 = 0b000_0100;
    pub const SHOP: u8 = 0b000_1000;
    pub const BESZ: u8 = 0b001_0000;
    pub const CONV: u8 = 0b010_0000;
    pub const ALL : u8 = 0b011_1111;
    pub const SEND: u8 = 0b100_0000;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub command: u8,
    pub osszetevok: String,
    pub recipes: String,
    pub meals: String,
    pub shoppings: String,
    pub beszer: String,
    pub conv: String,
}

impl Data {
    pub fn new() -> Self {
        Self {
            command: 0,
            osszetevok: String::new(),
            recipes: String::new(),
            meals: String::new(),
            shoppings: String::new(),
            beszer: String::new(),
            conv: String::new(),
        }
    }

    pub fn convert_string(&mut self, terv: &Terv, command: u8) {
        self.command = command;
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
    }

    pub fn convert_string_ossz(&mut self, terv: &Terv) {
        self.osszetevok = terv.osszetevok.iter().map(|ossz| {
            format!("{}\t{}\t{}\t{}", ossz.name, ossz.unit, ossz.time, ossz.unit_price)
        }).collect::<Vec<String>>().join("\n");
    }
    pub fn convert_string_rec(&mut self, terv: &Terv) {
        let max_len = terv.recipes.iter()
            .map(|x| x.ingredients.len()).max().unwrap();
        self.recipes = (0..max_len + 1).map(|row| {
            if row == 0 {
                terv.recipes.iter().map(|recipe| {
                    format!("{}\t{}\t\t\t", recipe.name, recipe.number)
                }).collect::<Vec<String>>().join("")
            } else {
                terv.recipes.iter().map(|recipe| {
                    if let Some(ingredient) = recipe.ingredients.get(row - 1) {
                        format!("{}\t{}\t{}\t\t", ingredient.name, ingredient.quantity, ingredient.unit)
                    } else {
                        String::from("\t\t\t\t")
                    }
                }).collect::<Vec<String>>().join("")
            }
        }).collect::<Vec<String>>().join("\n");
    }
    pub fn convert_string_meal(&mut self, terv: &Terv) {
        self.meals = terv.meals.iter().map(|meal| {
            format!("{}\t{}\t{}", meal.recipe, meal.number, meal.day.to_string())
        }).collect::<Vec<String>>().join("\n");
    }
    pub fn convert_string_shop(&mut self, terv: &Terv) {
        self.shoppings = terv.shoppingdays.iter().map(|shopping| {
            format!("{}\t{}", shopping.name, shopping.day.to_string())
        }).collect::<Vec<String>>().join("\n");
    }
    pub fn convert_string_besz(&mut self, terv: &Terv) {}
    pub fn convert_string_conv(&mut self, terv: &Terv) {}

    pub fn convert_data(&self, terv: &mut Terv){
        if self.command & com::OSSZ > 0 {
            self.convert_data_ossz(terv);
        }
        if self.command & com::REC > 0 {
            self.convert_data_rec(terv);
        }
        if self.command & com::MEAL > 0 {
            self.convert_data_meal(terv);
        }
        if self.command & com::SHOP > 0 {
            self.convert_data_shop(terv);
        }
        if self.command & com::BESZ > 0 {}
        if self.command & com::CONV > 0 {
            self.convert_data_conv(terv);
        }
    }

    pub fn convert_data_ossz(&self, terv: &mut Terv) {
        if self.osszetevok == "" { return; }
        log!("osszetevok: ", &self.osszetevok);

        terv.osszetevok.0 = self.osszetevok.split("\n").map(|row| {
            let cells: Vec<&str> = row.split("\t").collect();
            log!(format!("cells: {:?}", cells));
            Osszetevo {
                name: cells[0].to_string(),
                unit: cells[1].to_string(),
                time: cells[2].parse().unwrap(),
                unit_price: cells[3].parse().unwrap(),
            }
        }).collect();
    }
    pub fn convert_data_rec(&self, terv: &mut Terv) {
        if self.recipes == "" { return; }

        let rows: Vec<&str> = self.recipes.split("\n").collect();
        let mut recipes = Vec::new(); // (0..rows[0].split("\t").count() / 4).map(|_| Recipe::new()).collect();
        for (ing_index, row) in rows.iter().enumerate() {
            for (rec_index, slice) in row.split("\t").collect::<Vec<&str>>().chunks(4).enumerate() {
                if ing_index == 0 {
                    recipes.push(Recipe {
                        name: slice[0].to_string(),
                        number: slice[1].parse().unwrap(),
                        ingredients: Vec::new(),
                    })
                } else {
                    let recipe = recipes.get_mut(rec_index).unwrap();
                    recipe.ingredients.push(Ingredient {
                        name: slice[0].to_string(),
                        quantity: slice[1].parse().unwrap(),
                        unit: slice[2].to_string(),
                    })
                }
            }
        }
    }
    pub fn convert_data_meal(&self, terv: &mut Terv) {
        if self.meals == "" { return; }
        terv.meals.0 = self.meals.split("\n").map(|row| {
            let cells: Vec<&str> = row.split("\t").collect();
            Meal {
                recipe: cells[0].to_string(),
                number: cells[1].parse().unwrap(),
                day: ShopDay::from_str(cells[2]),
            }
        }).collect();
    }
    pub fn convert_data_shop(&self, terv: &mut Terv) {
        if self.shoppings == "" { return; }

        terv.shoppingdays.0 = self.shoppings.split("\n").map(|row| {
            let cells: Vec<&str> = row.split("\t").collect();
            Shopping {
                name: cells[0].to_string(),
                day: ShopDay::from_str(cells[1]),
            }
        }).collect();
    }
    
    pub fn convert_data_conv(&self, terv: &mut Terv) {
        if self.conv == "" { return; }
    }
}

pub fn copy_data_to_clipboard(){}

pub fn read_data_from_clibboard(){}

pub fn copy_beszerlista(){}

#[test]
fn test3() {
    panic!("n: {:?}", String::from("").split("\n").collect::<Vec<&str>>());
}