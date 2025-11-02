use std::str::FromStr;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};



use crate::backend::time::Time;
use crate::create_vec_wrapper;

pub mod sensitivity;
use self::sensitivity::*;

pub mod action;
use self::action::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sensitive {
    pub name: String,
    pub sensitivities: Sensitivities,
    pub actions: Actions,
}

impl Sensitive {
    pub fn is_here(&self, time: &Time) -> bool {
        let mut here = false;
        for action in self.actions.iter() {
            if time < &action.time {
                break;
            }
            match action.ty {
                ActionType::Meg => here = true,
                ActionType::El => here = false,
            }
        }
        here
    }
}

// impl Sensitive {
//     pub fn print_sensitivities(&self) -> String {
//         Sensitivity::print_vec(&self.sensitivities)
//     }
//     pub fn print_actions(&self) -> String {
//         Action::print_vec(&self.actions)
//     }
// }

create_vec_wrapper!(Sensitives, Sensitive);
