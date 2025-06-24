
use gloo::events::EventListener;
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::beszer::BeszerListak;
use crate::convert::{Conversation, Conversations};
use crate::meal::{Meal, Meals};
use crate::osszetevok::{Osszetevo, Osszetevok};
use crate::recipe::{Recipe, Recipes};
use crate::shop::{Shopping, Shoppings};
use crate::backend::matrix::Matrix;
use crate::socket::display_socket::send_data;

use crate::terv::{Terv, AppContext};
use crate::osszetevok::display::OsszetevoPage;
use crate::recipe::display::RecipePage;
use crate::meal::display::MealPage;
use crate::shop::display::ShopPage;
use crate::beszer::display::BeszerPage;
use crate::socket::{display_socket::Socket, display_close::ClosePage};
use crate::convert::display::ConvertPage;


use super::AppData;

// #[derive(Clone, PartialEq)]
// pub struct TervState{
//     pub terv: Rc<UseStateHandle<Terv>>,
// }

// impl TervState {
//     pub fn new(state: UseStateHandle<Terv>) -> Self {
//         TervState {
//             terv: Rc::new(state),
//         }
//     }
// }

pub enum Pages {
    Osszetevok,
    Recipes,
    Meals,
    ShoppingDays,
    Beszer,
    Conversations,
    //Close,
}

impl ToString for Pages {
    fn to_string(&self) -> String {
        match self {
            Pages::Osszetevok => "Összetevők",
            Pages::Recipes => "Receptek",
            Pages::Meals => "Étkezések",
            Pages::ShoppingDays => "Vásárnapok",
            Pages::Beszer => "Beszerlisták",
            Pages::Conversations => "Átváltások",
            //Pages::Close => "Bezárás",
        }.to_string()
    }
}

pub struct TervPage {
    pub current_page: Pages,
    pub app_data: AppContext,
    pub close: bool,
    pub _mouse_listener: Option<EventListener>,
}

#[derive(Properties, PartialEq)]
pub struct TervProps {
    pub version: u64,
}

// Display

pub enum TervMsg {
    Osszetevok,
    Recipes,
    Meals,
    ShoppingDays,
    Beszer,
    Conversations,
    Close,
    SafeIf,
    ReDraw,
    EndClose,
}

impl Component for TervPage {
    type Message = TervMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let app_data = Rc::new(AppData::new());

        // let listener = EventListener::new(&web_sys::window().unwrap(), "mousemove", move |event: &Event| {
        //     let mouse_event = event.dyn_ref::<web_sys::MouseEvent>().unwrap();
        //     let dialog = web_sys::window()
        //         .unwrap()
        //         .document()
        //         .unwrap()
        //         .get_element_by_id("dialog-box") // Replace with your dialog box ID
        //         .unwrap();

        //     let dialog = dialog.dyn_ref::<HtmlElement>().unwrap();
        //     //let dialog_rect = dialog.dyn_ref::<HtmlElement>().unwrap().get_bounding_client_rect();
        //     let mouse_x = mouse_event.client_x() as f64;
        //     let mouse_y = mouse_event.client_y() as f64;
        //     log!("mousex: ", mouse_x, ", clientlef: ", dialog.client_left(), ", offsetleft: ", dialog.offset_left());

        //     // if mouse_x < dialog
        //     //     || mouse_x > dialog_rect.x() + dialog_rect.width()
        //     //     || mouse_y < dialog_rect.y()
        //     //     || mouse_y > dialog_rect.y() + dialog_rect.height()
        //     // {
        //     //     link.send_message(TervMsg::SafeIf);
        //     // }
        // });

        let listener = EventListener::new(&web_sys::window().unwrap(), "mouseleave", move |event: &Event| {
            let mouse_event = event.dyn_ref::<web_sys::MouseEvent>().unwrap();
            let dialog = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("dialog-box") // Replace with your dialog box ID
                .unwrap();

            let dialog = dialog.dyn_ref::<HtmlElement>().unwrap();
            //let dialog_rect = dialog.dyn_ref::<HtmlElement>().unwrap().get_bounding_client_rect();
            let mouse_x = mouse_event.client_x() as f64;
            let mouse_y = mouse_event.client_y() as f64;
            log!("mousex: ", mouse_x, ", clientlef: ", dialog.client_left(), ", offsetleft: ", dialog.offset_left());

            // if mouse_x < dialog
            //     || mouse_x > dialog_rect.x() + dialog_rect.width()
            //     || mouse_y < dialog_rect.y()
            //     || mouse_y > dialog_rect.y() + dialog_rect.height()
            // {
            //     link.send_message(TervMsg::SafeIf);
            // }
        });

        Self {
            current_page: Pages::Osszetevok,
            app_data,
            close: false,
            _mouse_listener: Some(listener),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut terv = self.app_data.terv.borrow_mut();
        let mut old_terv = self.app_data.old_terv.borrow_mut();
        match msg {
            TervMsg::Osszetevok => {
                self.current_page = Pages::Osszetevok;
                true
            },
            TervMsg::Recipes => {
                self.current_page = Pages::Recipes;
                true
            },
            TervMsg::Meals => {
                self.current_page = Pages::Meals;
                true
            },
            TervMsg::ShoppingDays => {
                self.current_page = Pages::ShoppingDays;
                true
            },
            TervMsg::Beszer => {
                self.current_page = Pages::Beszer;
                true
            },
            TervMsg::Conversations => {
                self.current_page = Pages::Conversations;
                true
            },
            TervMsg::Close => {
                self.close = true;
                true
            },
            TervMsg::SafeIf => {
                //log!("mentés");
                save_if(&mut terv, &mut old_terv);
                true
            },
            TervMsg::ReDraw => {
                true
            },
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        //let state = use_state(|| Terv::new());
        let app_context = Rc::clone(&self.app_data);
        let terv = self.app_data.terv.borrow();
        
        let tervmsg_callback = link.callback(|msg: TervMsg| msg); 

        //let onmouseleave = link.callback(|_| TervMsg::SafeIf);

        html! {
            <div id="dialog-box" class="root">
                <p>{ "localhost test" }</p>
                <div class="menu">
                    <button onclick={link.callback(|_| TervMsg::Osszetevok)}>{ "Összetevők" }</button>
                    <button onclick={link.callback(|_| TervMsg::Recipes)}>{ "Receptek" }</button>
                    <button onclick={link.callback(|_| TervMsg::Meals)}>{ "Étkezések" }</button>
                    <button onclick={link.callback(|_| TervMsg::ShoppingDays)}>{ "Vásárnapok" }</button>
                    <button onclick={link.callback(|_| TervMsg::Beszer)}>{ "Beszerlisták" }</button>
                    <button onclick={link.callback(|_| TervMsg::Conversations)}>{ "Átváltások" }</button>
                    <button onclick={link.callback(|_| TervMsg::Close)}>{ "Bezárás" }</button>
                </div>
                <p>{ self.current_page.to_string() }</p>
                <div class="container">
                    <ContextProvider<AppContext> context={app_context}>
                    <div class="socket_on_terv">
                        <Socket redraw={tervmsg_callback.clone()} />
                    </div>
                    <div class="close_on_terv">
                        <ClosePage version={terv.version} visible={self.close} end={tervmsg_callback} />
                    </div>
                    {match self.current_page {
                        Pages::Osszetevok => {
                            html! {<OsszetevoPage version={terv.version} />}
                        },
                        Pages::Recipes => {
                            html! {<RecipePage version={terv.version} />}
                        },
                        Pages::Meals => {
                            html! {<MealPage version={terv.version} />}
                        },
                        Pages::ShoppingDays => {
                            html! {<ShopPage version={terv.version} />}
                        },
                        Pages::Beszer => {
                            html! {<BeszerPage version={terv.version} />}
                        },
                        Pages::Conversations => {
                            html! {<ConvertPage version={terv.version} />}
                        },
                        _ => {html! {<p>{ "Ismeretlen 3" }</p>}}
                    }}
                    </ContextProvider<AppContext>>
                </div>
            </div>
            
        }
    }
}

pub fn save_if(terv: &mut Terv, old_terv: &mut Terv) -> bool {
    log!("mentés");
    if !terv_eq(terv, old_terv) {
        send_data(terv, old_terv);
        true
    } else {
        false
    }
}

pub fn terv_eq(terv1: &Terv, terv2: &Terv) -> bool {
    terv1.clone_pure() == terv2.clone_pure()
    // let ossz1 = terv1.osszetevok.iter().filter(|&v| v != &Osszetevo::new()).collect::<Vec<_>>();
    // let ossz2 = terv2.osszetevok.iter().filter(|&v| v != &Osszetevo::new()).collect::<Vec<_>>();
    // if ossz1 != ossz2 { return false; }

    // let rec1 = terv1.recipes.iter().filter(|&v| v != &Recipe::new()).collect::<Vec<_>>();
    // let rec2 = terv2.recipes.iter().filter(|&v| v != &Recipe::new()).collect::<Vec<_>>();
    // if rec1 != rec2 { return false; }

    // let meal1 = terv1.meals.iter().filter(|&v| v != &Meal::new()).collect::<Vec<_>>();
    // let meal2 = terv2.meals.iter().filter(|&v| v != &Meal::new()).collect::<Vec<_>>();
    // if meal1 != meal2 { return false; }

    // let shop1 = terv1.shoppingdays.iter().filter(|&v| v != &Shopping::new()).collect::<Vec<_>>();
    // let shop2 = terv2.shoppingdays.iter().filter(|&v| v != &Shopping::new()).collect::<Vec<_>>();
    // if shop1 != shop2 { return false; }

    // let conv1 = terv1.convs.iter().filter(|&v| v != &Conversation::new()).collect::<Vec<_>>();
    // let conv2 = terv2.convs.iter().filter(|&v| v != &Conversation::new()).collect::<Vec<_>>();
    // if conv1 != conv2 { return false; }
    
    // true
}