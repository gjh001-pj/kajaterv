
use std::ops::{Deref, DerefMut};


use crate::{backend::time::Time, shop::{ShopDay, Shopping}};
use crate::create_vec_wrapper;

pub mod display;

#[derive(PartialEq, Clone, Debug)]
pub enum Meal {
    Common(CommonMeal),
    Troup(TroupMeal),
}

impl Meal {
    pub fn as_common(&self) -> &CommonMeal {
        if let Self::Common(common) = self {
            common
        } else {
            panic!("Not a Common Meal: {:?}", self)
        }
    }

    pub fn as_troup(&self) -> &TroupMeal {
        if let Self::Troup(troup) = self {
            troup
        } else {
            panic!("Not a Troup Meal: {:?}", self)
        }
    }

    pub fn as_common_mut(&mut self) -> &mut CommonMeal {
        if let Self::Common(common) = self {
            common
        } else {
            panic!("Not a Common Meal: {:?}", self)
        }
    }

    pub fn as_troup_mut(&mut self) -> &mut TroupMeal {
        if let Self::Troup(troup) = self {
            troup
        } else {
            panic!("Not a Troup Meal: {:?}", self)
        }
    }
}

impl Default for Meal {
    fn default() -> Self {
        Self::Common(CommonMeal::default())
    }
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct SingleTroupMeal {
    pub recipe: String,
    pub number: u32,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct TroupMeal {
    pub day: ShopDay,
    pub troups: Vec<SingleTroupMeal>
}

impl TroupMeal {
    pub fn new(day: ShopDay, troupmeals: Vec<SingleTroupMeal>) -> Self {
        Self {
            day,
            troups: troupmeals,
        }
    }
}


#[derive(PartialEq, Clone, Debug, Default)]
pub struct CommonMeal {
    pub recipe: String,
    pub number: u32,
    pub day: ShopDay,
}

impl CommonMeal {
    pub fn new(recipe: String, number: u32, day: ShopDay) -> Self {
        CommonMeal {
            recipe,
            number,
            day,
        }
    }
}

create_vec_wrapper!(Meals, Meal);

