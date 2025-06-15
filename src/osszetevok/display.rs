use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo::console::log;
use gloo::net::websocket::events::CloseEvent;
use web_sys::js_sys::JSON;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{MessageEvent, Window};
use wasm_bindgen::closure::Closure;

use crate::terv::TervContext;
use crate::backend::keyboard::TableFocusNavigator;
use crate::terv::display::TervProps;
use crate::backend::time::Time;

use super::*;



pub struct OsszetevoPage {
    pub focus_nav: TableFocusNavigator,
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

    fn create(ctx: &Context<Self>) -> Self {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();

        let window = web_sys::window().unwrap();

        let closure = Closure::wrap(Box::new(move |event: Event| {
            let event = event.dyn_ref::<web_sys::CloseEvent>().unwrap();
            // You can cast to `web_sys::BeforeUnloadEvent` if needed
            log!("beforeunload triggered");
            panic!("beforeunload triggered");
            //event.set_return_value(Some("You have unsaved changes. Do you really want to leave?"));
    
            // Optionally cancel the event
            // event.prevent_default(); // Not always necessary
            // You can try to set returnValue here if you want a prompt
            // but browsers may ignore it.
    
        }) as Box<dyn FnMut(_)>);

        window.add_event_listener_with_callback("beforeunload", closure.as_ref().unchecked_ref()).unwrap();

        let listener2 = EventListener::new(&window, "beforeunload", move |event| {
            let event = event.dyn_ref::<web_sys::CloseEvent>().unwrap();
            log!("closeevent triggered");
            panic!("closeevent triggered");
            //event.set_return_value(Some("You have unsaved changes. Do you really want to leave?"));
        });

        Self {
            focus_nav: TableFocusNavigator::new(terv.osszetevok.len(), 4),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let terv = terv.borrow();
        self.focus_nav.build(terv.osszetevok.len(), 4);
        true
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
                        imput.time = ShopDay::Day(time);
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
                                    <td><input type="text" list="osszetevo_name_list" value={value.name.clone()} onchange={update_name} 
                                        onkeydown={onkeydown(0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} /></td>
                                    <td><input type="text" value={value.unit.clone()} onchange={update_unit} 
                                        onkeydown={onkeydown(1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} /></td>
                                    <td><input value={value.time.to_string()} onchange={update_time} 
                                        onkeydown={onkeydown(2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} /></td>
                                    <td><input type="number" step="any" value={value.unit_price.to_string()} onchange={update_unit_price} 
                                        onkeydown={onkeydown(3)} ref={self.focus_nav.refs[index][3].clone()} onclick={onclick.clone()} /></td>
                                    <td><button onclick={link.callback(move |_| OsszetevoMsg::Remove(index))}>{ "Remove" }</button></td>
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