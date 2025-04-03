
pub struct Meal {
    pub recipe: String,
    pub number: u32,
    pub day: i32,
}

impl Meal {
    pub fn new(recipe: String, number: u32, day: i32) -> Self {
        Meal {
            recipe,
            number,
            day,
        }
    }
}