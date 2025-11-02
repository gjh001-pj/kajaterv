
use web_sys::console::warn;

use crate::backend::paste::PasteCell;
use crate::ew::*;
use crate::create_vec_wrapper;


#[derive(Clone, Debug, PartialEq)]
pub struct SubRecipe {
    pub name: String,
    pub scale: f64,
}

impl PasteCell for SubRecipe {
    fn paste(&mut self, cell: &str, index: usize) {
        match index {
            0 => self.name = cell.to_string(),
            1 => {
                if let Ok(scale) = cell.parse() {
                    self.scale = scale;
                }
            },
            _ => (),
        };
    }
}

impl GetEWs for SubRecipe {
    fn get_errors(&self, terv: &crate::terv::Terv) -> EWs<Err> {
        let mut errors = EWs::new();
        if !terv.recipes.exist(&self.name) {
            errors.push(EW::from("Nincs ilyen recept."));
        }

        errors
    }
    fn get_warnings(&self, _terv: &crate::terv::Terv) -> EWs<Warn> {
        let mut warnings = EWs::new();
        if self.scale == 0.0 {
            warnings.push(EW::from("A szorzó nulla."));
        }

        warnings
    }
    fn get_from(&self) -> String {
        self.name.clone()
    }
}

impl Default for SubRecipe {
    fn default() -> Self {
        Self {
            scale: 1.0,
            name: String::new(),
        }
    }
}

create_vec_wrapper!(SubRecipes, SubRecipe, GetEWs);

