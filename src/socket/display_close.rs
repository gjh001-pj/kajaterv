use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo::console::log;
use gloo::net::websocket::events::CloseEvent;
use web_sys::js_sys::JSON;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{MessageEvent, Window};
use wasm_bindgen::closure::Closure;

use crate::terv::{AppContext, Terv};
use crate::backend::keyboard::TableFocusNavigator;
use crate::terv::display::TervProps;
use crate::backend::time::Time;
use crate::terv::display::TervMsg;

use super::*;



pub struct ClosePage {
    // pub focus_nav: TableFocusNavigator,
    // pub visible: bool,
}

// Display

#[derive(Properties, PartialEq)]
pub struct CloseProps {
    pub version: u64,
    pub visible: bool,
    pub end: Callback<TervMsg>,
}

pub enum CloseMsg {
    AttemptClose
}

impl Component for ClosePage {
    type Message = CloseMsg;
    type Properties = CloseProps;

    fn create(ctx: &Context<Self>) -> Self {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        Self {
            // visible: false,
        }
    }

    // fn changed(&mut self, ctx: &Context<Self>) -> bool {
    //     let terv = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
    //     let terv = terv.borrow();
    //     self.focus_nav.build(terv.osszetevok.len(), 4);
    //     true
    // }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        //let terv = use_context::<AppContext>().expect("Terv not found");
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();

        match msg {
            
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let app_data = link.context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();
        let props = ctx.props();
        
        html! {
            if props.visible {
                <div class="close">
                    <p>{ "almaaaaaa" }</p>
                </div>
            } else {}
        }
    }
}