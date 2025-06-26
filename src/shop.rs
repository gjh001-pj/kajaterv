use core::num;
use std::ops::{Deref, DerefMut};
use std::fmt::Display;

use crate::backend::time::Time;


pub mod display;

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum ShopDay {
    Day(Time),
    Name(String),
}

impl ShopDay {
    pub fn new() -> Self {
        Self::Name(String::from(""))
    }

    pub fn as_day(&self) -> &Time {
        if let ShopDay::Day(day) = self {
            return day;
        } else {
            panic!("Not Day {:?}", self);
        }
    }

    pub fn as_mut_day(&mut self) -> &mut Time {
        if let ShopDay::Day(day) = self {
            return day;
        } else {
            panic!("Not Day {:?}", self);
        }
    }

    pub fn as_name(&self) -> &String {
        if let ShopDay::Name(name) = self {
            return name;
        } else {
            panic!("Not Name {:?}", self);
        }
    }

    pub fn as_mut_name(&mut self) -> &mut String {
        if let ShopDay::Name(name) = self {
            return name;
        } else {
            panic!("Not Name {:?}", self);
        }
    }
}

impl Default for ShopDay {
    fn default() -> Self {
        Self::Name(String::new())
    }
}

impl From<&str> for ShopDay {
    fn from(s: &str) -> Self {
        if let Ok(day) = s.parse() {
            Self::Day(day)
        } else {
            Self::Name(s.to_string())
        }
    }
}

impl Display for ShopDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShopDay::Day(number) => write!(f, "{}", number),
            ShopDay::Name(name) => write!(f, "{}", name),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub struct Shopping {
    pub day: ShopDay,
    pub name: String,
}

impl Shopping {
    pub fn new() -> Self {
        Shopping {
            day: ShopDay::Name(String::from("")),
            name: String::from(""),
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

impl Shoppings {
    pub fn get_by_day(&self, day: &ShopDay) -> Option<&Shopping> {
        for shopping in self.iter() {
            if shopping.day == *day {
                return Some(shopping);
            }
        }
        None
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
