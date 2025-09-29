
use web_sys::console::warn;

use crate::ew::*;
use crate::create_vec_wrapper;


#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubRecipe {
    pub name: String,
    pub scale: f64,
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

create_vec_wrapper!(SubRecipes, SubRecipe, GetEWs);

