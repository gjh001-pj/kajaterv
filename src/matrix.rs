use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use crate::shop::Shopping;

#[derive(PartialEq, Clone, Debug)]
pub struct Sub {
    pub quantity: f64,
    pub recipe: String,
    pub number: u32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Subs(pub Vec<Sub>);

impl Subs {
    pub fn new() -> Subs {
        Subs (Vec::new())
    }
}

impl Deref for Subs {
    type Target = Vec<Sub>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Subs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix {
    pub mat: HashMap<Shopping, HashMap<String, Subs>>
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            mat: HashMap::new(),
        }
    }
}

impl Deref for Matrix {
    type Target = HashMap<Shopping, HashMap<String, Subs>>;

    fn deref(&self) -> &Self::Target {
        &self.mat
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mat
    }
}