

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

    pub fn convert(&self, to: &str) -> Option<f64> {
        if let Some(factor) = get_factor(&self.unit, to) {
            return Some(factor * self.quantity);
        }
        None
    }
}

pub fn get_factor(from: &str, to: &str) -> Option<f64> {
    for &(a1, b1, f1) in CONVS.iter() {
        let (mid, factor1) = if from == a1 {
            (b1, f1)
        } else if from == b1 {
            (a1, 1.0 / f1)
        } else {
            continue;
        };
        
        if mid == to {
            return Some(factor1);
        }

        for &(a2, b2, f2) in CONVS.iter() {
            let factor2 = if mid == a2 && to == b2 {
                f2
            } else if mid == b2 && to == a2 {
                1.0 / f2
            } else {
                continue;
            };

            return Some(factor1 * factor2)
        }
    }

    None
}

const CONVS: &[(&str, &str, f64)] = &[
    ("g", "kg", 0.001),
    ("dkg", "kg", 0.01),
    ("ml", "l", 0.001),
    ("dl", "l", 0.1),
    ("cl", "l", 0.01)
];