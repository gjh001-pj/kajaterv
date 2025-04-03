use std::rc::Rc;
use std::cell::RefCell;

use crate::recipe::{Recipe, RecipePage};
use crate::osszetevok::OsszetevoPage;
use crate::meal::Meal;

pub struct Terv {
    pub osszetevo_p: OsszetevoPage,
    pub recipe_p: RecipePage,
    pub meals: Vec<Meal>,
    pub shoppingdays: Vec<i32>,
}

impl Terv {
    pub fn new() -> Self {
        Terv {
            recipe_p: RecipePage::new(),
            osszetevo_p: OsszetevoPage::new(),
            meals: Vec::new(),
            shoppingdays: Vec::new(),
        }
    }
}