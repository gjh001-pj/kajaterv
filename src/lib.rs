pub mod recipe;
pub mod osszetevok;
//pub mod data;
pub mod meal;
pub mod shop;
//pub mod matrix;
pub mod beszer;
//pub mod keyboard;
pub mod socket;
pub mod convert;

pub mod terv;

pub mod backend;
pub mod close;
pub mod ew;
pub mod troop;
pub mod input;
pub mod tr;



use terv::display::TervPage;

pub fn init() {
    yew::start_app::<TervPage>();
}

#[cfg(test)]
mod tests {
    
    
}

