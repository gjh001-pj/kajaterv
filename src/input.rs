use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::rc::Rc;


use yew::virtual_dom::AttrValue;
use yew::{Callback, Component, Event, KeyboardEvent, NodeRef, Properties};
use yew::html;
use yew::InputEvent;
use web_sys::ClipboardEvent;
use wasm_bindgen::JsCast;
use gloo::console::log;


use crate::backend::keyboard::TableFocusNavigator;
use crate::backend::paste::{handle_paste, PasteCell};
use crate::terv::{AppData, Terv};

pub trait Item: PasteCell + Default + PartialEq + Clone {}
impl<T: PasteCell + Default + PartialEq + Clone> Item for T {}

// pub trait GetDataVec<I>: for<'a> Fn(&'a RefCell<Terv>) -> std::cell::RefMut<'a, Vec<I>> + 'static {}
// impl<T: for<'a> Fn(&'a RefCell<Terv>) -> std::cell::RefMut<'a, Vec<I>> + 'static, I> GetDataVec<I> for T {}

pub type GetDataVec<I: Item> = Rc<dyn for<'a> Fn(&'a RefCell<Terv>) -> RefMut<'a, Vec<I>> + 'static>;

pub fn create_gdv<I, F>(f: F) -> GetDataVec<I>
where
    I: Item,
    F: for<'a> Fn(&'a mut Terv) -> &'a mut Vec<I> + 'static
{
    Rc::new(move |terv: &RefCell<Terv>| RefMut::map(terv.borrow_mut(), &f))
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Input<I> 
where
    I: Item,
    // T: PasteCell + Default + PartialEq,
    // F: Fn(&RefCell<Terv>) -> std::cell::RefMut<Vec<T>> + 'static,
{
    //pub in_grid: bool,
    pub node_ref: Option<NodeRef>,
    i: PhantomData<I>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum InputMsg {
    OnKeyDown(KeyboardEvent),
    HandlePaste(Event),
}

#[derive(Clone)]
pub struct FNP<I>
where
    I: Item,
    // T: PasteCell + Default + PartialEq,
    // F: Fn(&RefCell<Terv>) -> std::cell::RefMut<Vec<T>> + 'static,
{
    pub row: usize,
    pub colomn: usize,
    pub focus_nav: Rc<RefCell<TableFocusNavigator>>,
    pub get_data_vec: GetDataVec<I>, 
    pub app_data: Rc<AppData>,
}

impl<I> FNP<I> 
where
    I: Item
{
    pub fn new(row: usize, colomn: usize, focus_nav: Rc<RefCell<TableFocusNavigator>>, get_data_vec: GetDataVec<I>, app_data: Rc<AppData>) -> Self {
        Self {
            row,
            colomn,
            focus_nav,
            get_data_vec,
            app_data,
        }
    }

    pub fn clone_set_colomn(&self, colomn: usize) -> Self {
        let mut res = (*self).clone();
        res.colomn = colomn;
        res
    }
}

impl<I> PartialEq for FNP<I>
where
    I: Item,
    // T: PasteCell + Default + PartialEq,
    // F: Fn(&RefCell<Terv>) -> std::cell::RefMut<Vec<T>> + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row 
        && self.colomn == other.colomn
        && self.focus_nav == other.focus_nav
        && self.app_data == other.app_data
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Properties, PartialEq)]
pub struct InputProps<I> 
where
    I: Item,
    // T: PasteCell + Default + PartialEq,
    // F: Fn(&RefCell<Terv>) -> std::cell::RefMut<Vec<T>> + 'static,
{
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub r#type: Option<AttrValue>,
    #[prop_or_default]
    pub step: Option<AttrValue>,
    #[prop_or_default]
    pub list: Option<AttrValue>,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: Option<bool>,

    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,

    // #[prop_or_default]
    // pub row: Option<usize>,
    // #[prop_or_default]
    // pub colomn: Option<usize>,
    // #[prop_or_default]
    // pub focus_nav: Option<RefCell<TableFocusNavigator>>,
    #[prop_or_default]
    pub fnp: Option<FNP<I>>,
}

// impl<T, F> PartialEq for InputProps<T, F>
// where
//     T: Item,
//     F: GetDataVec<T>,
//     // T: PasteCell + Default + PartialEq,
//     // F: Fn(&RefCell<Terv>) -> std::cell::RefMut<Vec<T>> + 'static,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.value == other.value 
//         && self.r#type == other.r#type
//         && self.list == other.list
//         && self.placeholder == other.placeholder
//         && self.disabled == other.disabled
//         && self.onchange == other.onchange
//         && self.oninput == other.oninput
//         && self.fnp == other.fnp
//     }
//     fn ne(&self, other: &Self) -> bool {
//         !self.eq(other)
//     }
// }

impl<I> Component for Input<I>
where
    I: Item + 'static,
    // T: PasteCell + Default + PartialEq + 'static,
    // F: Fn(&RefCell<Terv>) -> std::cell::RefMut<Vec<T>> + 'static,
{
    type Message = InputMsg;
    type Properties = InputProps<I>;
    fn create(ctx: &yew::Context<Self>) -> Self {
        let node_ref = if let Some(fnp) = &ctx.props().fnp {
            Some(fnp.focus_nav.borrow().refs[fnp.row][fnp.colomn].clone())
        } else {
            None
        };

        Input::<I> { 
            node_ref,
            i: PhantomData,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            InputMsg::OnKeyDown(e) => {
                let fnp = match &props.fnp {
                    Some(fnp) => fnp,
                    None => return false,
                };
                let mut focus_nav = fnp.focus_nav.borrow_mut();
                focus_nav.handle_key(fnp.row, fnp.colomn, e);
                false
            },
            InputMsg::HandlePaste(e) => {
                let fnp = match &props.fnp {
                    Some(fnp) => fnp,
                    None => return false,
                };
                if let Some(clipboard_event) = e.dyn_ref::<ClipboardEvent>() {
                    if let Some(data_transfer) = clipboard_event.clipboard_data() {
                        match data_transfer.get_data("text") {
                            Ok(text) => {
                                if text == "" { return false; } 
                                else if !text.contains("\n") && !text.contains("\t") {
                                    return false;
                                } else {
                                    e.prevent_default();
                                    let mut focus_nav = fnp.focus_nav.borrow_mut();
                                    let mut data_vec = (fnp.get_data_vec)(&fnp.app_data.terv);
                                    handle_paste(&text, fnp.row, fnp.colomn, &mut data_vec, &mut focus_nav);
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
                    return false;
                }
            }
            _ => todo!(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        let link = ctx.link();

        let (onkeydown, onpaste) = if let Some(_) = &props.fnp {
            (link.callback(move |e: KeyboardEvent| {
                InputMsg::OnKeyDown(e)
            }), 
            link.callback(move |e: Event| InputMsg::HandlePaste(e)))
        } else {
            (Callback::noop(),
            Callback::noop())
        };

        html!{
            <input 
                value={ props.value.clone() }
                type={ props.r#type.clone() }
                step={ props.step.clone() }
                list={ props.list.clone() }
                placeholder={ props.placeholder.clone() }
                disabled={ props.disabled.unwrap_or(false) }
                onchange={ props.onchange.clone() }
                oninput={ props.oninput.clone() }


                {onkeydown}
                {onpaste}
                ref={ if let Some(nr) = &self.node_ref {nr.clone()} else {NodeRef::default()} }
            />
        }
    }
}