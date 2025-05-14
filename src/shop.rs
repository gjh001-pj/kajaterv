use std::ops::{Deref, DerefMut};

pub mod display;

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum ShopDay {
    Day(i32),
    Name(String),
}

impl ShopDay {
    pub fn from_str(day: &str) -> Self {
        if let Ok(day) = day.parse() {
            Self::Day(day)
        } else {
            Self::Name(day.to_string())
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ShopDay::Day(number) => number.to_string(),
            ShopDay::Name(name) => name.clone(),
        }
    }

    pub fn as_day(&self) -> &i32 {
        if let ShopDay::Day(day) = self {
            return day;
        } else {
            panic!("Not Day {:?}", self);
        }
    }

    pub fn as_mut_day(&mut self) -> &mut i32 {
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
