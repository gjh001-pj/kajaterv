use std::ops::{Deref, DerefMut};

pub mod display;

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub recipes: String,
    pub quantities: String,
    pub prices: String,
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct BeszerLista {
    pub name: String,
    pub time: String,
    pub recipes: String,
    pub items: Vec<Item>,
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct BeszerListak(pub Vec<BeszerLista>);

impl Deref for BeszerListak {
    type Target = Vec<BeszerLista>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BeszerListak {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}