use std::rc::Rc;
use std::cell::RefCell;

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
            osszetevok: vec![Osszetevo::new()],
            recipes: vec![Recipe::new()],
            meals: vec![Meal::new()],
            shoppingdays: Vec::new(),
        }
    }
}

pub type TervContext = Rc<RefCell<Terv>>;