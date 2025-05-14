use std::ops::{Deref, DerefMut};


pub mod display;

#[derive(PartialEq, Clone, Debug)]
pub struct Osszetevo {
    pub name: String,
    pub unit: String,
    pub time: u32,
    pub unit_price: f64,
}


impl Osszetevo {
    pub fn new() -> Self {
        Osszetevo {
            name: String::new(),
            unit: String::new(),
            time: 0,
            unit_price: 0.0,
        }
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
