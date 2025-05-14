use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo::console::log;

use crate::terv::TervContext;
use crate::keyboard::TableFocusNavigator;
use crate::terv::display::TervProps;

use super::*;



pub struct OsszetevoPage {
    pub focus_nav: TableFocusNavigator,
}

impl OsszetevoPage {
    pub fn new() -> Self {
        OsszetevoPage {
            focus_nav: TableFocusNavigator::new(1, 4),
        }
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
    KeyPressed(usize, usize, KeyboardEvent),
    MouseClick,
}

impl Component for OsszetevoPage {
    type Message = OsszetevoMsg;
    type Properties = TervProps;

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
                self.focus_nav.build(self.focus_nav.rows + 1, 4);
                true
            },
            OsszetevoMsg::Remove(index) => {
                terv.osszetevok.remove(index);
                self.focus_nav.build(self.focus_nav.rows - 1, 4);
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
            OsszetevoMsg::KeyPressed(row, col, e) => {
                self.focus_nav.handle_key(row, col, e);
                false
            },
            OsszetevoMsg::MouseClick => {
                self.focus_nav.set_edit();
                false
            },
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let terv = link.context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();
        let _ = terv.version;

        let all_osszetevo_name_list: Vec<&String> = terv.recipes.iter().map(|recipe| {
            recipe.ingredients.iter().map(|ingredient| {
                &ingredient.name
            })
        }).flatten().collect();

        let osszetevo_name_list = all_osszetevo_name_list.iter().map(|&rec_ossz| {
            if !terv.osszetevok.iter().map(|x| &x.name).collect::<Vec<&String>>().contains(&rec_ossz) {
                html! {<option value={rec_ossz.clone()} />}
            } else {
                html! {}
            }
        });
        
        html! {
            <div class="osszetevok">
                <div class="table">
                    <datalist id="osszetevo_name_list">
                        { for osszetevo_name_list }
                    </datalist>
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

                            let onkeydown = |col| link.callback(move |e: KeyboardEvent| {
                                OsszetevoMsg::KeyPressed(index, col, e)
                            });

                            let onclick = link.callback(move |_| {
                                OsszetevoMsg::MouseClick
                            });

                            html! {
                                <tr>
                                    <th><input type="text" list="osszetevo_name_list" value={value.name.clone()} onchange={update_name} 
                                        onkeydown={onkeydown(0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} /></th>
                                    <th><input type="text" value={value.unit.clone()} onchange={update_unit} 
                                        onkeydown={onkeydown(1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} /></th>
                                    <th><input type="number" min="0"
                                        value={value.time.to_string()} onchange={update_time} 
                                        onkeydown={onkeydown(2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} /></th>
                                    <th><input type="number" step="any" value={value.unit_price.to_string()} onchange={update_unit_price} 
                                        onkeydown={onkeydown(3)} ref={self.focus_nav.refs[index][3].clone()} onclick={onclick.clone()} /></th>
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