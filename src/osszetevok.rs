use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::ops::{Deref, DerefMut};

use crate::terv::TervContext;


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

#[derive(PartialEq, Clone)]
pub struct Osszetevok(pub Vec<Osszetevo>);

impl Osszetevok {
    pub fn exist(&self, name: &str) -> bool {
        for osszetevo in self.iter() {
            if osszetevo.name == name {
                return true;
            }
        }
        return false;
    }

    pub fn get(&mut self, name: &str) -> Option<&mut Osszetevo> {
        for osszetevo in self.iter_mut() {
            if osszetevo.name == name {
                return Some(osszetevo);
            }
        }
        return None
    }

    pub fn get_def(&mut self) -> Option<&mut Osszetevo> {
        self.get("default")
    }

    pub fn get_or_def(&mut self, name: &str) -> Option<&mut Osszetevo> {
        if self.exist(name) {
            return self.get(name);
        } else {
            return self.get_def();
        }
    }
}

impl Deref for Osszetevok {
    type Target = Vec<Osszetevo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Osszetevok {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        //let terv = use_context::<TervContext>().expect("Terv not found");
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();

        match msg {
            OsszetevoMsg::Add => {
                terv.osszetevok.push(Osszetevo::new());
                true
            },
            OsszetevoMsg::Remove(index) => {
                terv.osszetevok.remove(index);
                true
            }
            OsszetevoMsg::UpdateName(index, name) => {
                if let Some(imput) = terv.osszetevok.get_mut(index) {
                    imput.name = name;
                }
                true
            },
            OsszetevoMsg::UpdateUnit(index, unit) => {
                if let Some(imput) = terv.osszetevok.get_mut(index) {
                    imput.unit = unit;
                }
                true
            },
            OsszetevoMsg::UpdateTime(index, time) => {
                if let Some(imput) = terv.osszetevok.get_mut(index) {
                    if let Ok(time) = time.parse() {
                        imput.time = time;
                    }
                }
                true
            },
            OsszetevoMsg::UpdateUnitPrice(index, unit_price) => {
                if let Some(imput) = terv.osszetevok.get_mut(index) {
                    if let Ok(unit_price) = unit_price.parse() {
                        imput.unit_price = unit_price;
                    }
                }
                true
            },
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        //let terv = use_context::<TervContext>().expect("Terv not found");
        let terv = link.context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();
        
        html! {
            <div class="osszetevok">
                <div class="table">
                    <table>
                        <tr>
                            <th>{ "Name" }</th><th>{ "Unit" }</th><th>{ "Time" }</th><th>{ "Unit price" }</th>
                        </tr>
                        { for terv.osszetevok.iter().enumerate().map(|(index, value)| {
                            let update_name = link.callback(move |e: Event| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                OsszetevoMsg::UpdateName(index, input.value())
                            });

                            let update_unit = link.callback(move |e: Event| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                OsszetevoMsg::UpdateUnit(index, input.value())
                            });

                            let update_time = link.callback(move |e: Event| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                OsszetevoMsg::UpdateTime(index, input.value())
                            });

                            let update_unit_price = link.callback(move |e: Event| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                OsszetevoMsg::UpdateUnitPrice(index, input.value())
                            });

                            html! {
                                <tr>
                                    <th><input type="text" value={value.name.clone()} onchange={update_name} /></th>
                                    <th><input type="text" value={value.unit.clone()} onchange={update_unit} /></th>
                                    <th><input type="number" min="0"
                                        value={value.time.to_string()}
                                        onchange={update_time} /></th>
                                    <th><input type="number" step="any" value={value.unit_price.to_string()} onchange={update_unit_price} /></th>
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