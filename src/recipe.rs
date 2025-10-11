
use core::num;
use std::{fmt::Display, ops::{Deref, DerefMut}, str::FromStr};
use crate::{backend::paste::PasteCell, ew::{EWs, GetEWs, EW}, recipe::subrecipe::SubRecipes, troop::sensitive::sensitivity::Sensitivities};
use crate::create_vec_wrapper;

//use crate::osszetevok::Osszetevo;


pub mod ingredient;
pub mod subrecipe;
pub mod display;

use ingredient::{Ingredient, Ingredients};

#[derive(Debug, PartialEq, Clone, Default)]
pub enum SensMode {
    FromOssz,
    FromRec,
    Both,
    #[default]
    None,
}
impl Display for SensMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::FromOssz => "FO",
            Self::FromRec => "FR",
            Self::Both => "B",
            Self::None => "N",
        })
    }
}
impl FromStr for SensMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FO" => Ok(Self::FromOssz),
            "FR" => Ok(Self::FromRec),
            "B" => Ok(Self::Both),
            "N" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Recipe {
    pub name: String,
    pub number: u32,
    pub ingredients: Ingredients,
    pub sub_recipes: SubRecipes,
    pub sens_mode: SensMode,
    pub sens: Sensitivities,
}


impl Recipe {
    // pub fn new() -> Self {
    //     Recipe {
    //         name: String::new(),
    //         number: 0,
    //         ingredients: Ingredients::default(),
    //         sub_recipes: SubRecipes::default(),
    //     }
    // }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_number(&mut self, number: u32) {
        self.number = number;
    }

    pub fn set_ingredients(&mut self, ingredients: Ingredients) {
        self.ingredients = ingredients;
    }
}

impl GetEWs for Recipe {
    fn get_errors(&self, terv: &crate::terv::Terv) -> EWs<crate::ew::Err> {
        let mut errors = self.ingredients.get_errors(terv);

        if self.name == "" {
            errors.push(EW::from("Nincs név"));
        }

        errors
    }

    fn get_warnings(&self, terv: &crate::terv::Terv) -> EWs<crate::ew::Warn> {
        let mut warnings = self.ingredients.get_warnings(terv);

        if self.number == 0 {
            warnings.push(EW::from("A létszám nulla"));
        }

        warnings
    }

    fn get_from(&self) -> String {
        self.name.clone()
    }
}

create_vec_wrapper!(Recipes, Recipe);

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

impl GetEWs for Recipes {
    fn get_errors(&self, terv: &crate::terv::Terv) -> EWs<crate::ew::Err> {
        self.iter().filter_map(|recipe| {
            let ews = recipe.get_errors(terv);
            if ews.len() == 0 {
                None
            } else {
                Some(EW::Child { 
                    from: recipe.name.clone(), 
                    ews, 
                })
            }
        }).collect::<Vec<_>>().into()
    }

    fn get_warnings(&self, terv: &crate::terv::Terv) -> EWs<crate::ew::Warn> {
        self.iter().filter_map(|recipe| {
            let ews = recipe.get_warnings(terv);
            if ews.len() == 0 {
                None
            } else {
                Some(EW::Child { 
                    from: recipe.name.clone(), 
                    ews, 
                })
            }
        }).collect::<Vec<_>>().into()
    }

    fn get_from(&self) -> String {
        String::new()
    }
}
