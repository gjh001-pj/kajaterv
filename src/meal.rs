
use std::ops::{Deref, DerefMut};


use crate::shop::{Shopping, ShopDay};


pub mod display;


#[derive(PartialEq, Clone, Debug)]
pub struct Meal {
    pub recipe: String,
    pub number: u32,
    pub day: ShopDay,
}

impl Meal {
    pub fn new() -> Self {
        Meal {
            recipe: String::new(),
            number: 0,
            day: ShopDay::Day(0),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Meals(pub Vec<Meal>);

impl Deref for Meals {
    type Target = Vec<Meal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Meals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
