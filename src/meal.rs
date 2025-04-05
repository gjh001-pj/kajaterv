
#[derive(PartialEq, Clone)]
pub struct Meal {
    pub recipe: String,
    pub number: u32,
    pub day: i32,
}

impl Meal {
    pub fn new() -> Self {
        Meal {
            recipe: String::new(),
            number: 0,
            day: 0,
        }
    }
}