use std::{ops::{Deref, DerefMut}, str::FromStr};
use std::vec::IntoIter;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{backend::time::Time, create_vec_wrapper};

pub mod sensitive;
use self::sensitive::*;

use self::sensitive::action::*;
use self::sensitive::sensitivity::*;

pub mod display;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Troop {
    pub name: String,
    pub sensitives: Sensitives,
}

impl Troop {
    pub fn sensitives_here(&self, time: &Time) -> Sensitives {
        self.sensitives.iter().filter_map(|sensitive| {
            if sensitive.is_here(time) {
                Some(sensitive.clone())
            } else {
                None
            }
        }).collect::<Vec<_>>().into()
    }
}

create_vec_wrapper!(Troops, Troop);

#[test]
fn test_action_vec_from_str() {
    assert_eq!(Actions::from("meg: 0. 4:34, el: 4. 14:23"), vec![
        Action::new(ActionType::Meg, "0. 4:34".parse().unwrap()), 
        Action::new(ActionType::El, "4. 14:23".parse().unwrap())].into()
    );
}