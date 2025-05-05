use yew::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{MessageEvent, Window};
use web_sys::HtmlInputElement;

pub struct Socket {
    listener: Option<EventListener>,
    received: String,
    send: String,
}

pub enum SocketMsg {
    ReceivedMessage(String),
    UpdateSend(String),
    SendMessage,
}

impl Component for Socket {
    type Message = SocketMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window = web_sys::window().unwrap();
        let link = ctx.link().clone();

        let listener = EventListener::new(&window, "message", move |event| {
            let message_event: &MessageEvent = event.dyn_ref().unwrap();
            if let Some(data) = message_event.data().as_string() {
                link.send_message(SocketMsg::ReceivedMessage(data));
            }
        });

        Self {
            listener: Some(listener),
            received: String::from("Waiting for message..."),
            send: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SocketMsg::ReceivedMessage(data) => {
                self.received = format!("Received: {}", data);
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
                <input type="text" {onchange}/>
                <button onclick={link.callback(|_| SocketMsg::SendMessage)}>{ "Send data" }</button>
            </div>
        }
    }
}