use yew::prelude::*;
use web_sys::HtmlInputElement;


use crate::display::AppState;

#[derive(PartialEq, Clone)]
pub struct Osszetevo {
    pub name: String,
    unit: String,
    time: u32,
    unit_price: f64,
}


impl Osszetevo {
    pub fn new() -> Self {
        Osszetevo {
            name: String::new(),
            unit: String::new(),
            time: 0,
            unit_price: 0.0,
        }
    }
}

pub struct OsszetevoPage {}

impl OsszetevoPage {
    pub fn new() -> Self {
        OsszetevoPage {}
    }
}

// Display

pub enum OsszetevoMsg {
    UpdateName(usize, String),
    UpdateUnit(usize, String),
    UpdateTime(usize, String),
    UpdateUnitPrice(usize, String),
    Add,
    Remove(usize),
}

impl Component for OsszetevoPage {
    type Message = OsszetevoMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        OsszetevoPage::new()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<AppState>(Callback::noop()).unwrap().0.terv;
        
        match msg {
            OsszetevoMsg::Add => {
                let mut new_terv = (**terv).clone();
                new_terv.osszetevok.push(Osszetevo::new());
                terv.set(new_terv);
                true
            },
            OsszetevoMsg::Remove(index) => {
                let mut new_terv = (**terv).clone();
                new_terv.osszetevok.remove(index);
                terv.set(new_terv);
                true
            }
            OsszetevoMsg::UpdateName(index, name) => {
                let mut new_terv = (**terv).clone();
                if let Some(imput) = new_terv.osszetevok.get_mut(index) {
                    imput.name = name;
                }
                terv.set(new_terv);
                true
            },
            OsszetevoMsg::UpdateUnit(index, unit) => {
                let mut new_terv = (**terv).clone();
                if let Some(imput) = new_terv.osszetevok.get_mut(index) {
                    imput.unit = unit;
                }
                terv.set(new_terv);
                true
            },
            OsszetevoMsg::UpdateTime(index, time) => {
                let mut new_terv = (**terv).clone();
                if let Some(imput) = new_terv.osszetevok.get_mut(index) {
                    if let Ok(time) = time.parse() {
                        imput.time = time;
                    }
                }
                terv.set(new_terv);
                true
            },
            OsszetevoMsg::UpdateUnitPrice(index, unit_price) => {
                let mut new_terv = (**terv).clone();
                if let Some(imput) = new_terv.osszetevok.get_mut(index) {
                    if let Ok(unit_price) = unit_price.parse() {
                        imput.unit_price = unit_price;
                    }
                }
                terv.set(new_terv);
                true
            },
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let terv = ctx.link().context::<AppState>(Callback::noop()).unwrap().0.terv;
        
        html! {
            <div class="osszetevok">
                <div class="table">
                    <table>
                        <tr>
                            <th>{ "Name" }</th><th>{ "Unit" }</th><th>{ "Time" }</th><th>{ "Unit price" }</th>
                        </tr>
                        { for terv.osszetevok.iter().enumerate().map(|(index, value)| {
                            let update_name = link.callback(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                OsszetevoMsg::UpdateName(index, input.value())
                            });

                            let update_unit = link.callback(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                OsszetevoMsg::UpdateUnit(index, input.value())
                            });

                            let update_time = link.callback(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                OsszetevoMsg::UpdateTime(index, input.value())
                            });

                            let update_unit_price = link.callback(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                OsszetevoMsg::UpdateUnitPrice(index, input.value())
                            });

                            html! {
                                <tr>
                                    <th><input type="text" value={value.name.clone()} oninput={update_name} /></th>
                                    <th><input type="text" value={value.unit.clone()} oninput={update_unit} /></th>
                                    <th><input type="number" min="0"
                                        value={value.time.to_string()}
                                        oninput={update_time} /></th>
                                    <th><input type="number" step="any" value={value.unit_price.to_string()} oninput={update_unit_price} /></th>
                                    <th><button onclick={link.callback(move |_| OsszetevoMsg::Remove(index))}>{ "Remove" }</button></th>
                                </tr>
                            }
                        })}
                    </table>
                </div>
                <div class="others">
                    <button onclick={link.callback(|_| OsszetevoMsg::Add)}>{ "Add" }</button>
                    //<button onclick={link.callback(|_| OsszetevoMsg::Add)}>{ "Remove" }</button>
                </div>
                
            </div>
        }
    }
}