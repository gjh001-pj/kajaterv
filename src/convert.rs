use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::recipe::ingredient::Ingredient;

pub mod display;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub from: String,
    pub to: String,
    pub factor: f64,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            from: String::new(),
            to: String::new(),
            factor: 0.0,
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct Conversations(pub Vec<Conversation>);

impl Deref for Conversations {
    type Target = Vec<Conversation>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Conversations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait Convert {
    fn convert(&self, to: &str, convs: &Conversations) -> Option<f64>;
}

pub fn get_factor(from: &str, to: &str, convs: &Conversations) -> Option<f64> {
    if from == to {
        return Some(1.0);
    }
    for conv1 in convs.iter() {
        let (mid, factor1): (&str, f64) = if from == conv1.from {
            (&conv1.to, conv1.factor)
        } else if from == conv1.to {
            (&conv1.from, 1.0 / conv1.factor)
        } else {
            continue;
        };
        
        if mid == to {
            return Some(factor1);
        }

        for conv2 in convs.iter() {
            let factor2 = if mid == conv2.from && to == conv2.to {
                conv2.factor
            } else if mid == conv2.to && to == conv2.from {
                1.0 / conv2.factor
            } else {
                continue;
            };

            return Some(factor1 * factor2)
        }
    }
    None
}

impl Convert for Ingredient {
    fn convert(&self, to: &str, convs: &Conversations) -> Option<f64> {
        if let Some(factor) = get_factor(&self.unit, to, convs) {
            return Some(factor * self.quantity);
        }
        None
    }
}



#[test]
fn conv_test1() {
    let convs: Conversations = Conversations(vec![
        Conversation{ from: "g".to_string(), to: "kg".to_string(), factor: 0.001},
        Conversation{ from: "dkg".to_string(), to: "kg".to_string(), factor: 0.01},
        Conversation{ from: "ml".to_string(), to: "l".to_string(), factor: 0.001},
        Conversation{ from: "dl".to_string(), to: "l".to_string(), factor: 0.1},
        Conversation{ from: "cl".to_string(), to: "l".to_string(), factor: 0.01},
    ]);

    assert_eq!(get_factor("g", "kg", &convs), Some(0.001));
    assert_eq!(get_factor("kg", "g", &convs), Some(1000.0));
    assert_eq!(get_factor("g", "dkg", &convs), Some(0.1));
    assert_eq!(get_factor("dkg", "g", &convs), Some(10.0));
    assert_eq!(get_factor("g", "l", &convs), None);
    // assert_eq!(get_factor("g", "kg"), Some(0.001));
    // assert_eq!(get_factor("g", "kg"), Some(0.001));
}