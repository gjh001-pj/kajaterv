use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum Shopping {
    Day(i32),
    Name(String),
}

impl Shopping {
    pub fn to_string(&self) -> String {
        match self {
            Shopping::Day(number) => number.to_string(),
            Shopping::Name(name) => name.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct Shoppings(pub Vec<Shopping>);

impl Shoppings {
    pub fn new() -> Self {
        Shoppings (Vec::new())
    }
}

impl Deref for Shoppings {
    type Target = Vec<Shopping>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Shoppings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}