use crate::troop::sensitive::sensitivity::Sensitivities;

use crate::create_vec_wrapper;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct SensitiveMeal {
    pub recipe: String,
}


#[derive(PartialEq, Clone, Debug, Default)]
pub struct SensitiveIgnore {
    pub reason: String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CaseType {
    Recipe(SensitiveMeal),
    Ignore(SensitiveIgnore),
}

impl Default for CaseType {
    fn default() -> Self {
        Self::Recipe(SensitiveMeal::default())
    }
}


#[derive(PartialEq, Clone, Debug, Default)]
pub struct SensitiveCase {
    pub sensitivities: Sensitivities,
    pub ty: CaseType,
}

create_vec_wrapper!{SensitiveCases, SensitiveCase}

