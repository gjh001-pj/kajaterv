

#[derive(Debug, PartialEq, Clone)]
pub struct Ingredient {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
}


impl Ingredient {
    pub fn new() -> Self {
        Ingredient {
            name: String::new(),
            quantity: 0.0,
            unit: String::new(),
        }
    }
}
