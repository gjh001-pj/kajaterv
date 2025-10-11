use gloo::utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::Window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, MessageEvent, window};
use yew::Component;
use yew::html::Scope;
use gloo::console::log;

use crate::terv::display::{TervPage, TervMsg};
use crate::terv::Terv;

pub mod data;
pub mod display_socket;

use self::data::{Data, com as dcom};

pub mod commands {
    pub mod rust {
        pub const SAVE_DATA: &str = "save";
        pub const CLOSE_PAGE: &str = "close";
        pub const REQUEST_DATA: &str = "request data";
    }

    pub mod js {
        pub const LOAD_DATA: &str = "load";
        pub const DATA_SAVED: &str = "data saved";
    }
}

use self::commands::rust as rcom;
use self::commands::js as jcom;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Hash)]
pub struct Message {
    pub id: String,
    pub command: String,
    pub content: String,
}

pub fn load_data(data: Data, terv: &mut Terv, old_terv: &mut Terv) {
    data.convert_data(terv);
    *old_terv = terv.clone();
    log!("egyenlő: ", *terv == *old_terv);
}

pub enum ReDrawType {
    Main,
    All,
    None,
}

pub fn handle_message(message: &Message, terv: &mut Terv, old_terv: &mut Terv, link: &Scope<TervPage>) -> ReDrawType {
    //log!("message.content:", &message.content);
    match message.command.as_str() {
        jcom::LOAD_DATA => {
            match serde_json::from_str::<Data>(&message.content) {
                Ok(data) => {
                    load_data(data, terv, old_terv);
                },
                Err(err) => log!("rust: error while parsing message.content", err.to_string()),
            };
            link.send_message(TervMsg::SetState("Elmentve".to_string()));
            ReDrawType::All
        },
        jcom::DATA_SAVED => {
            link.send_message(TervMsg::SetState("Elmentve".to_string()));
            ReDrawType::Main
        },
        _ => {
            todo!("Not implemented command from parent to yew");
        },
    }
}

// pub fn register_message_listener(link: Scope<impl Component>/*callback: impl Fn(String, JsValue) + 'static*/) {
//     let closure = Closure::wrap(Box::new(move |event: MessageEvent| {
//         let data = event.data();
//         if let Ok(val) = data.into_serde::<serde_json::Value>() {
//             if let Some(message_id) = val.get("id").and_then(|id| id.as_str()) {
//                 //callback(message_id.to_string(), data);
//                
//                 handle_message(link, message_id.to_string(), data);
//             }
//         }
//     }) as Box<dyn FnMut(MessageEvent)>);
//
//     window().unwrap()
//         .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
//         .unwrap();
//
//     closure.forget(); // don't drop it
// }

pub fn request_data() {
    if let Some(window) = web_sys::window().unwrap().parent().unwrap() {
        send_message(window, Message {
            id: String::new(),
            command: rcom::REQUEST_DATA.to_string(),
            content: dcom::ALL.to_string(),
        });
    }
}

pub fn save_data(terv: &mut Terv, old_terv: &mut Terv) {
    let mut data = Data::default();
    terv.pure();
    *old_terv = terv.clone();
    terv.make_beszerek();
    data.convert_string(terv, dcom::ALL);
    if let Some(window) = web_sys::window().unwrap().parent().unwrap() {
        send_message(window, Message {
            id: String::new(),
            command: rcom::SAVE_DATA.to_string(),
            content: serde_json::to_string(&data).unwrap(),
        });
    }
}

pub fn send_message(window: Window, message: Message) {
    let js_msg = JsValue::from_serde(&message).unwrap();
    window.post_message(&js_msg, "*").unwrap();
}
