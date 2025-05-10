use web_sys::js_sys::JSON;
use yew::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{MessageEvent, Window};
use web_sys::HtmlInputElement;
use gloo::console::log;

use crate::data::Data;
use crate::meal::{self, Meal};
use crate::data::com;
use crate::terv::TervContext;


pub struct Socket {
    listener: Option<EventListener>,
    received: String,
    send: String,
}

// #[derive(Properties, PartialEq)]
// pub struct SocketProps {
//     pub on_data_received: Callback<()>,
// }

pub enum SocketMsg {
    ReceivedData(Data),
    ReceivedMessage(String),
    SendData,
    SendMessage,
    RequestData,
    TriggerRedraw,
    
    UpdateSend(String),

}

impl Component for Socket {
    type Message = SocketMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window = web_sys::window().unwrap();
        let link = ctx.link().clone();

        let listener = EventListener::new(&window, "message", move |event| {
            let event: MessageEvent = event.dyn_ref::<MessageEvent>().unwrap().clone();
            let message = event.data();

            log!("sent data from rust: ", message.as_string().unwrap());

            let parsed = JSON::parse(&message.as_string().unwrap()).unwrap();

            if let Ok(data) = serde_wasm_bindgen::from_value::<Data>(parsed) {
                log!("data to load: ", serde_json::to_string(&data).unwrap());
                link.send_message(SocketMsg::ReceivedData(data));
                return;
            }

            if let Some(msg) = message.as_string() {
                link.send_message(SocketMsg::ReceivedMessage(msg));
            }
        });

        Self {
            listener: Some(listener),
            received: String::from("Waiting for message..."),
            send: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let terv = ctx.link().context::<TervContext>(Callback::noop()).unwrap().0;
        let mut terv = terv.borrow_mut();
        match msg {
            SocketMsg::ReceivedMessage(message) => {
                self.received = format!("Received: {}", message);
                true
            },
            SocketMsg::ReceivedData(data) => {
                data.convert_data(&mut terv);
                //ctx.props().on_data_received.emit(());
                ctx.link().send_message(SocketMsg::TriggerRedraw);
                true
            },
            SocketMsg::SendMessage => {
                if let Some(window) = web_sys::window() {
                    let _ = window.parent().unwrap().unwrap()
                        .post_message(&JsValue::from(self.send.clone()), "*");
                }
                self.received = String::from("Waiting for message...");
                true
            },
            SocketMsg::SendData => {
                let mut data = Data::new();
                data.convert_string(&terv, com::ALL);
                if let Some(window) = web_sys::window() {
                    let json_data = serde_json::to_string(&data).unwrap();
                    log!("data from rust:", data.command, "json_data", &json_data);
                    let _ = window.parent().unwrap().unwrap()
                        .post_message(&JsValue::from_str(&json_data), "*");
                }
                false
            },
            SocketMsg::RequestData => {
                let mut data = Data::new();
                data.command = com::SEND | com::ALL;
                if let Some(window) = web_sys::window() {
                    let json_data = serde_json::to_string(&data).unwrap();
                    let _ = window.parent().unwrap().unwrap()
                        .post_message(&JsValue::from_str(&json_data), "*");
                }
                false
            },
            SocketMsg::TriggerRedraw => {
                terv.version += 1;
                true
            },
            SocketMsg::UpdateSend(data) => {
                self.send = data;
                false
            }
            _ => false 
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onchange = link.callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            SocketMsg::UpdateSend(input.value())
        });

        html! {
            <div>
                <p>{ self.received.clone() }</p>
                //<input type="text" {onchange}/>
                <button onclick={link.callback(|_| SocketMsg::SendData)}>{ "Save data" }</button>
                <button onclick={link.callback(|_| SocketMsg::RequestData)}>{ "Load data" }</button>
            </div>
        }
    }
}