
use std::ops::{Deref, DerefMut};


use crate::{backend::time::Time, shop::{ShopDay, Shopping}};
use crate::create_vec_wrapper;

pub mod display;

pub mod common;
use common::CommonMeal;

pub mod troup;
use troup::TroupMeal;

pub mod sensitive;

#[derive(PartialEq, Clone, Debug)]
pub enum MealType {
    Common(CommonMeal),
    Troup(TroupMeal),
}

impl Default for MealType {
    fn default() -> Self {
        Self::Common(CommonMeal::default())
    }
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Meal {
    pub day: ShopDay,
    pub ty: MealType,
}

impl Meal {
    pub fn as_common(&self) -> &CommonMeal {
        match &self.ty {
            MealType::Common(v) => v,
            other => panic!("Not a Common Meal: {:?}", other),
        }
    }

    pub fn as_troup(&self) -> &TroupMeal {
        match &self.ty {
            MealType::Troup(v) => v,
            other => panic!("Not a Troup Meal: {:?}", other),
        }
    }

    pub fn as_common_mut(&mut self) -> &mut CommonMeal {
        match &mut self.ty {
            MealType::Common(v) => v,
            other => panic!("Not a Common Meal: {:?}", other),
        }
    }

    pub fn as_troup_mut(&mut self) -> &mut TroupMeal {
        match &mut self.ty {
            MealType::Troup(v) => v,
            other => panic!("Not a Troup Meal: {:?}", other),
        }
    }
}

create_vec_wrapper!(Meals, Meal);

