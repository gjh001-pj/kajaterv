use std::ops::{Deref, DerefMut};


use crate::create_vec_wrapper;
use crate::convert::Convert;
use crate::ew::{GetEWs, EW, EWs};
use crate::terv::Terv;
use crate::backend::paste::PasteCell;

#[derive(Debug, PartialEq, Clone, Default)]
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

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_quantity(&mut self, quantity: f64) {
        self.quantity = quantity;
    }

    pub fn set_unit(&mut self, unit: String) {
        self.unit = unit;
    }

    pub fn quantity_from_str(&mut self, quantity: &str) {
        if let Ok(quantity) = quantity.parse() {
            self.quantity = quantity;
        }
    }

}

impl GetEWs for Ingredient {
    fn get_errors(&self, terv: &Terv) -> crate::ew::EWs<crate::ew::Err> {
        let mut errors = EWs::new();
        
        match terv.osszetevok.by_name(&self.name) {
            None => errors.push(EW::from("Az összetevő nem található")),
            Some(osszetevo) => {
                if let None = self.convert(&osszetevo.unit, &terv.convs) {
                    errors.push(EW::from(
                        format!("'{}' nem váltható át '{}'-ra/re", self.unit, osszetevo.unit
                    )));
                }
            }
        };

        errors
    }

    fn get_warnings(&self, terv: &Terv) -> EWs<crate::ew::Warn> {
        let mut warnings = EWs::new();

        if self.quantity == 0.0 {
            warnings.push(EW::from("A mennyiség nulla"));
        }
        if self.unit == "" {
            warnings.push(EW::from("Nincs megadva mértékegység"));
        }

        warnings
    }

    fn get_from(&self) -> String {
        self.name.clone()
    }
}

impl PasteCell for Ingredient {
    fn paste(&mut self, cell: &str, index: usize) {
        match index {
            0 => self.set_name(cell.to_string()),
            1 => self.quantity_from_str(cell),
            2 => self.set_unit(cell.to_string()),
            _ => (),
        };
    }
}

create_vec_wrapper!(Ingredients, Ingredient, GetEWs);

// impl AsRef<Vec<Ingredient>> for Ingredients {
//     fn as_ref(&self) -> &Vec<Ingredient> {
//         &self.0
//     }
// }

// impl AsMut<Vec<Ingredient>> for Ingredients {
//     fn as_mut(&mut self) -> &mut Vec<Ingredient> {
//         &mut self.0
//     }
// }

// impl GetEWs for Ingredients {
//     fn get_errors(&self, terv: &Terv) -> EWs<crate::ew::Err> {
//         self.iter().filter_map(|ing| {
//             let ews = ing.get_errors(terv);
//             if ews.len() == 0 {
//                 None
//             } else {
//                 Some(EW::Child { 
//                     from: ing.name.clone(), 
//                     ews, 
//                 })
//             }
//         }).collect::<Vec<_>>().into()
//     }

//     fn get_warnings(&self, terv: &Terv) -> EWs<crate::ew::Warn> {
//         self.iter().filter_map(|ing| {
//             let ews = ing.get_warnings(terv);
//             if ews.len() == 0 {
//                 None
//             } else {
//                 Some(EW::Child { 
//                     from: ing.name.clone(), 
//                     ews, 
//                 })
//             }
//         }).collect::<Vec<_>>().into()
//     }
// }

