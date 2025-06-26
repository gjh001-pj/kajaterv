use std::ops::{Deref, DerefMut};
use crate::shop::ShopDay;
use crate::backend::paste::PasteCell;


pub mod display;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Osszetevo {
    pub name: String,
    pub unit: String,
    pub time: ShopDay,
    pub unit_price: f64,
}


impl Osszetevo {
    pub fn new() -> Self {
        Osszetevo {
            name: String::new(),
            unit: String::new(),
            time: ShopDay::Name(String::new()),
            unit_price: 0.0,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn set_unit(&mut self, unit: &str) {
        self.unit = String::from(unit);
    }

    pub fn set_time(&mut self, time: &str) {
        if let Ok(time) = time.parse() {
            self.time = ShopDay::Day(time);
        } else {
            self.time = ShopDay::Name(String::from(time))
        }
    }

    pub fn set_unit_price(&mut self, unit_price: &str) {
        if let Ok(unit_price) = unit_price.parse() {
            self.unit_price = unit_price;
        }
    }
}

impl PasteCell for Osszetevo {
    fn paste(&mut self, cell: &str, index: usize) {
        match index {
            0 => self.set_name(cell),
            1 => self.set_unit(cell),
            2 => self.set_time(cell),
            3 => self.set_unit_price(cell),
            _ => (),
        };
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Osszetevok(pub Vec<Osszetevo>);

impl Osszetevok {
    pub fn exist(&self, name: &str) -> bool {
        for osszetevo in self.iter() {
            if osszetevo.name == name {
                return true;
            }
        }
        return false;
    }

    pub fn by_name(&self, name: &str) -> Option<& Osszetevo> {
        for osszetevo in self.iter() {
            if osszetevo.name == name {
                return Some(osszetevo);
            }
        }
        return None
    }

    pub fn by_name_def(&self) -> Option<&Osszetevo> {
        self.by_name("default")
    }

    pub fn by_name_or_def(&self, name: &str) -> Option<&Osszetevo> {
        if self.exist(name) {
            return self.by_name(name);
        } else {
            return self.by_name_def();
        }
    }

    pub fn by_name_mut(&mut self, name: &str) -> Option<&mut Osszetevo> {
        for osszetevo in self.iter_mut() {
            if osszetevo.name == name {
                return Some(osszetevo);
            }
        }
        return None
    }

    pub fn by_name_def_mut(&mut self) -> Option<&mut Osszetevo> {
        self.by_name_mut("default")
    }

    pub fn by_name_or_def_mut(&mut self, name: &str) -> Option<&mut Osszetevo> {
        if self.exist(name) {
            return self.by_name_mut(name);
        } else {
            return self.by_name_def_mut();
        }
    }

    pub fn add(&mut self, osszetevo: Osszetevo) {
        self.0.push(osszetevo);
    }

    pub fn add_new(&mut self) {
        self.add(Osszetevo::new());
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    pub fn set_name(&mut self, name: &str, index: usize) {
        if let Some(ossz) = self.0.get_mut(index) {
            ossz.name = String::from(name);
        }
    }

    pub fn set_unit(&mut self, unit: &str, index: usize) {
        if let Some(ossz) = self.0.get_mut(index) {
            ossz.unit = String::from(unit);
        }
    }

    pub fn set_time(&mut self, time: &str, index: usize) {
        if let Some(imput) = self.0.get_mut(index) {
            if let Ok(time) = time.parse() {
                imput.time = ShopDay::Day(time);
            }
        }
    }

    pub fn set_unit_price(&mut self, unit_price: &str, index: usize) {
        if let Some(imput) = self.0.get_mut(index) {
            if let Ok(unit_price) = unit_price.parse() {
                imput.unit_price = unit_price;
            }
        }
    }
}

impl Deref for Osszetevok {
    type Target = Vec<Osszetevo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Osszetevok {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
