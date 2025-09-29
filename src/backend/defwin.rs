use web_sys::ClipboardEvent;
use yew::{props, Callback, Context, Event, KeyboardEvent};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::NodeRef;

use super::keyboard::TableFocusNavigator;
use crate::backend::paste::handle_paste;
use crate::backend::paste::PasteCell;


pub enum DefWinMsg {
    KeyPressed(usize, usize, KeyboardEvent),
    MouseClick,
    HandlePaste(usize, usize, Event),
}

pub fn handle_default_msg<T, D>(msg: DefWinMsg, focus_nav: &mut TableFocusNavigator, ctx: &Context<T>, data_vec: &mut Vec<D>) -> bool
where 
    T: yew::Component,
    D: PasteCell + Default,
{
    match msg {
        DefWinMsg::KeyPressed(row, col, e) => {
            focus_nav.handle_key(row, col, e);
            false
        },
        DefWinMsg::MouseClick => {
            focus_nav.set_edit();
            false
        }
        DefWinMsg::HandlePaste(row, col, e) => {
            if let Some(clipboard_event) = e.dyn_ref::<ClipboardEvent>() {
                if let Some(data_transfer) = clipboard_event.clipboard_data() {
                    match data_transfer.get_data("text") {
                        Ok(text) => {
                            if text == "" { return false; } 
                            else if !text.contains("\n") && !text.contains("\t") {
                                return false;
                            } else {
                                e.prevent_default();
                                handle_paste(&text, row, col, data_vec, focus_nav);
                                true
                                //OsszetevoMsg::HandlePaste(text, row, col)
                            }
                        }
                        Err(err) => {
                            log!("Failed to retrieve pasted text: {}", err.as_string().unwrap_or_else(|| "Unknown error".to_string()));
                            return false;
                        }
                    }
                } else {
                    log!("Clipboard data is unavailable.");
                    return false;
                }
            } else {
                log!("Event is not a ClipboardEvent.");
                return false
            }
        }
        _ => false,
    }
}



#[macro_export]
macro_rules! win_comp_def {
    ( $msg:expr, $link:expr ) => {
        let onpaste = |row, col| $link.callback(move |e: Event| $msg(DefWinMsg::HandlePaste(row, col, e)));
        let onkeydown = |row, col| $link.callback(move |e: KeyboardEvent| {
            $msg(DefWinMsg::KeyPressed(row, col, e))
        });
        let onclick = $link.callback(move |_| {
            $msg(DefWinMsg::MouseClick)
        });
    };
}

// #[derive(Properties, PartialEq)]
// struct DefProps {
//     onkeydown: Callback<KeyboardEvent>,
//     r#ref: NodeRef,
//     onclick: Callback<Event>,
//     onpaste: Callback<Event>,
// }

#[macro_export]
macro_rules! win_comp_use {
    ( $row:expr, $col:expr, $focus_nav:expr ) => {
        (
            onkeydown={onkeydown($row, $col)};
            node_ref={$focus_nav.refs[$row][$col].clone()};
            onclick={onclick.clone()};
            onpaste={onpaste($row, $col).clone()};
        )
    };
}

#[macro_export]
macro_rules! input_with_defs {
    ( { $($props:tt)* },  $row:expr, $col:expr, $focus_nav:expr ) => {
        html! {
            <input 
                $($props)*
                onkeydown={onkeydown($row, $col)};
                node_ref={$focus_nav.refs[$row][$col].clone()};
                onclick={onclick.clone()};
                onpaste={onpaste($row, $col).clone()};
            />
        }
    };
}