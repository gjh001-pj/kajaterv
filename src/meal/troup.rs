use crate::{backend::time::Time, shop::{ShopDay, Shopping}};


#[derive(PartialEq, Clone, Debug, Default)]
pub struct SingleTroupMeal {
    pub recipe: String,
    pub number: u32,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct TroupMeal {
    pub troups: Vec<SingleTroupMeal>
}

impl TroupMeal {
    pub fn new(troupmeals: Vec<SingleTroupMeal>) -> Self {
        Self {
            troups: troupmeals,
        }
    }
}