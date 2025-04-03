pub mod recipe;
pub mod osszetevok;
pub mod data;
pub mod terv;
pub mod meal;
pub mod display;


use display::App;

pub fn init() {
    
    yew::start_app::<App>();
}

#[cfg(test)]
mod tests {
    
    
}

