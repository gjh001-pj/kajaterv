use crate::backend::time::Time;
use crate::ew::{GetEWs, EW, EWs, Err, Warn};
use crate::meal::sensitive::SensitiveCases;
use crate::shop::{ShopDay, Shopping};
use crate::troop::sensitive::sensitivity::Sensitivities;



#[derive(PartialEq, Clone, Debug, Default)]
pub struct CommonMeal {
    pub number: u32,
    pub main_recipe: String,

    pub sens_cases: SensitiveCases,
}

impl CommonMeal {
    pub fn new(recipe: String, number: u32, sens_cases: SensitiveCases) -> Self {
        CommonMeal {
            main_recipe: recipe,
            number,
            sens_cases,
        }
    }
}

impl GetEWs for CommonMeal {
    fn get_errors(&self, terv: &crate::terv::Terv) -> crate::ew::EWs<crate::ew::Err> {
        let mut errs: EWs<Err> = EWs::new();
        if !terv.recipes.exist(&self.main_recipe) {
            errs.push(EW::from("Nincs ilyen recept."));
        }
        errs
    }

    fn get_warnings(&self, terv: &crate::terv::Terv) -> EWs<Warn> {
        let mut warns: EWs<Warn> = EWs::new();
        if self.number == 0 {
            warns.push(EW::from("A létszám nulla."));
        }
        //todo!();

        warns
    }

    fn get_from(&self) -> String {
        self.main_recipe.clone()
    }
}