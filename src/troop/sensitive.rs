use std::str::FromStr;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};



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

// impl Sensitive {
//     pub fn print_sensitivities(&self) -> String {
//         Sensitivity::print_vec(&self.sensitivities)
//     }
//     pub fn print_actions(&self) -> String {
//         Action::print_vec(&self.actions)
//     }
// }

create_vec_wrapper!(Sensitives, Sensitive);
