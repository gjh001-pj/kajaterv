
use crate::recipe::Recipe;
use crate::osszetevok::Osszetevo;
use crate::meal::Meal;

#[derive(PartialEq, Clone)]
pub struct Terv {
    pub osszetevok: Vec<Osszetevo>,
    pub recipes: Vec<Recipe>,
    pub meals: Vec<Meal>,
    pub shoppingdays: Vec<i32>,
}

impl Terv {
    pub fn new() -> Self {
        Terv {
            osszetevok: Vec::new(),
            recipes: Vec::new(),
            meals: Vec::new(),
            shoppingdays: Vec::new(),
        }
    }
}