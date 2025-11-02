
use yew::prelude::*;
use web_sys::{DataTransfer, HtmlInputElement};
use gloo::console::log;
use gloo::net::websocket::events::CloseEvent;
use web_sys::js_sys::JSON;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{MessageEvent, Window, ClipboardEvent};
use wasm_bindgen::closure::Closure;

use crate::backend::defwin::{handle_default_msg, DefWinMsg};
use crate::terv::AppContext;
use crate::backend::keyboard::TableFocusNavigator;
use crate::terv::display::TervProps;
use crate::backend::time::Time;
use crate::backend::paste::handle_paste;
use crate::win_comp_def;
use crate::input_with_defs;

use super::*;

use crate::tr;

pub struct OsszetevoPage {
    pub focus_nav: TableFocusNavigator,
    pub dragged_index: Option<usize>,
    pub dragover_index: Option<usize>,
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
    HandlePaste(String, usize, usize),
    DoNothing,
    DefWinBeh(DefWinMsg),

    ReDraw,

    DragStart(usize),
    DragEnter(usize),
    Drop(usize),
}

impl Component for OsszetevoPage {
    type Message = OsszetevoMsg;
    type Properties = TervProps;

    fn create(ctx: &Context<Self>) -> Self {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        Self {
            focus_nav: TableFocusNavigator::new(terv.osszetevok.len(), 4),
            dragged_index: None,
            dragover_index: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();
        self.focus_nav.build(terv.osszetevok.len(), 4);
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        //let terv = use_context::<AppContext>().expect("Terv not found");
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();

        match msg {
            OsszetevoMsg::Add => {
                terv.osszetevok.add_new();
                self.focus_nav.build(self.focus_nav.rows + 1, 4);
                true
            },
            OsszetevoMsg::Remove(index) => {
                terv.osszetevok.remove(index);
                self.focus_nav.build(self.focus_nav.rows - 1, 4);
                true
            }
            OsszetevoMsg::UpdateName(index, name) => {
                terv.osszetevok.set_name(&name, index);
                true
            },
            OsszetevoMsg::UpdateUnit(index, unit) => {
                terv.osszetevok.set_unit(&unit, index);
                true
            },
            OsszetevoMsg::UpdateTime(index, time) => {
                terv.osszetevok.set_time(&time, index);
                true
            },
            OsszetevoMsg::UpdateUnitPrice(index, unit_price) => {
                terv.osszetevok.set_unit_price(&unit_price, index);
                true
            },
            // OsszetevoMsg::KeyPressed(row, col, e) => {
            //     self.focus_nav.handle_key(row, col, e);
            //     false
            // },
            // OsszetevoMsg::MouseClick => {
            //     self.focus_nav.set_edit();
            //     false
            // },
            // OsszetevoMsg::HandlePaste(text, row_index, column_index) => {
            //     log!("row:", row_index, "col:", column_index, "text:", &text);
            //     handle_paste(&text, row_index, column_index, &mut terv.osszetevok, &mut self.focus_nav);
            //     true
            // },
            OsszetevoMsg::DoNothing => false,
            OsszetevoMsg::DefWinBeh(dwm) => {
                handle_default_msg(dwm, &mut self.focus_nav, ctx, &mut terv.osszetevok)
            },

            OsszetevoMsg::DragStart(index) => {
                self.dragged_index = Some(index);
                //log!(format!("Dragged row: {}", index));
                true
            },
            OsszetevoMsg::DragEnter(target_index) => {
                //log!(format!("Dragging over row {}", target_index));
                self.dragover_index = Some(target_index);
                true
            },
            OsszetevoMsg::Drop(target_index) => {
                let src_index = self.dragged_index.unwrap();
                let render = if src_index == target_index {false} else {
                    // terv.osszetevok.swap(src_index, target_index);
                    let item = terv.osszetevok.remove(src_index);
                    terv.osszetevok.insert(target_index, item);

                    true
                };
                self.dragged_index = None;
                self.dragover_index = None;
                render
            },
            OsszetevoMsg::ReDraw => true,
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let app_data = link.context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow_mut();
        
        let mut all_osszetevo_name_list: Vec<&String> = terv.recipes.iter().map(|recipe| {
            recipe.ingredients.iter().map(|ingredient| {
                &ingredient.name
            })
        }).flatten().collect();
        all_osszetevo_name_list.sort();
        all_osszetevo_name_list.dedup();

        let osszetevo_name_list = all_osszetevo_name_list.iter().map(|&rec_ossz| {
            if !terv.osszetevok.iter().map(|x| &x.name).collect::<Vec<&String>>().contains(&rec_ossz) {
                html! {<option value={rec_ossz.clone()} />}
            } else {
                html! {}
            }
        });
        
        let onpaste = |row, col| link.callback(move |e: Event| 
            OsszetevoMsg::DefWinBeh(DefWinMsg::HandlePaste(row, col, e)
        ));
        let onkeydown = |row, col| link.callback(move |e: KeyboardEvent| {
            OsszetevoMsg::DefWinBeh(DefWinMsg::KeyPressed(row, col, e))
        });
        let onclick = link.callback(move |_| {
            OsszetevoMsg::DefWinBeh(DefWinMsg::MouseClick)
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
                        { for terv.osszetevok.iter().enumerate().map(|(index, osszetevo)| {
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

                            // html!{
                            //     {tr!{
                            //         OsszetevoMsg::ReDraw,
                            //         self.dragover_index, self.dragged_index,
                            //         link, index, terv.osszetevok, 
                            //         {<td><input type="text" list="osszetevo_name_list" value={value.name.clone()} 
                            //                 onchange={update_name} 
                            //                 onkeydown={onkeydown(index, 0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} onpaste={onpaste(index, 0).clone()}
                            //              /></td>
                            //         <td><input type="text" value={value.unit.clone()} onchange={update_unit} 
                            //             onkeydown={onkeydown(index, 1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} onpaste={onpaste(index, 1).clone()} /></td>
                            //         <td><input value={value.time.to_string()} onchange={update_time} 
                            //             onkeydown={onkeydown(index, 2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} onpaste={onpaste(index, 2).clone()} /></td>
                            //         <td><input type="number" step="any" value={value.unit_price.to_string()} onchange={update_unit_price} 
                            //             onkeydown={onkeydown(index, 3)} ref={self.focus_nav.refs[index][3].clone()} onclick={onclick.clone()} onpaste={onpaste(index, 3).clone()} /></td>
                            //         <td><button onclick={link.callback(move |_| OsszetevoMsg::Remove(index))}>{ "Remove" }</button></td>
                            //         if index != 0 && value.name != "" && terv.osszetevok.get(0..index).unwrap().iter().filter(|&osszetevo| osszetevo.name == value.name).next() != None {
                            //             <p class="warn">{ format!("{} már létezik", value.name) }</p>
                            //         }}
                            //     }}
                            // }

                            let row_class = if self.dragged_index == Some(index) {"dragged-row"}
                            else if self.dragover_index == Some(index) {"drag-over-row"} 
                            else {""};

                            html! {
                                <tr 
                                    key={index}
                                    draggable={"true"}
                                    class={row_class}
                                    ondragstart={link.callback(move |e: DragEvent| {
                                        e.data_transfer().unwrap().set_drag_image(&web_sys::HtmlImageElement::new().unwrap(), 0, 0);
                                        OsszetevoMsg::DragStart(index)
                                    })}
                                    ondragenter={link.callback(move |_| OsszetevoMsg::DragEnter(index))}
                                    ondragover={Callback::from(move |e: DragEvent| e.prevent_default())}
                                    ondrop={link.callback(move |_| OsszetevoMsg::Drop(index))}
                                >
                                    <td><input type="text" list="osszetevo_name_list" value={osszetevo.name.clone()} 
                                            onchange={update_name} 
                                            onkeydown={onkeydown(index, 0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} onpaste={onpaste(index, 0).clone()}
                                         /></td>
                                    <td><input type="text" value={osszetevo.unit.clone()} onchange={update_unit} 
                                        onkeydown={onkeydown(index, 1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} onpaste={onpaste(index, 1).clone()} /></td>
                                    <td><input value={osszetevo.time.to_string()} onchange={update_time} 
                                        onkeydown={onkeydown(index, 2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} onpaste={onpaste(index, 2).clone()} /></td>
                                    <td><input type="number" step="any" value={osszetevo.unit_price.to_string()} onchange={update_unit_price} 
                                        onkeydown={onkeydown(index, 3)} ref={self.focus_nav.refs[index][3].clone()} onclick={onclick.clone()} onpaste={onpaste(index, 3).clone()} /></td>
                                    <td><button onclick={link.callback(move |_| OsszetevoMsg::Remove(index))}>{ "Remove" }</button></td>
                                    {for osszetevo.get_errors(&terv).iter().map(|error| html!{
                                        <td class="err">{ match error {
                                            EW::Owned(text) => text.clone(),
                                            _ => "hiba a kódban".to_string(),
                                        } }</td>
                                    })}
                                    {for osszetevo.get_warnings(&terv).iter().map(|warning| html!{
                                        <td class="warn">{ match warning {
                                            EW::Owned(text) => text.clone(),
                                            _ => "hiba a kódban".to_string(),
                                        } }</td>
                                    })}
                                    // if index != 0 && value.name != "" && terv.osszetevok.get(0..index).unwrap().iter().filter(|&osszetevo| osszetevo.name == value.name).next() != None {
                                    //     <p class="warn">{ format!("{} már létezik", value.name) }</p>
                                    // }
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