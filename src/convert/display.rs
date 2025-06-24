use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::backend::keyboard::TableFocusNavigator;
use crate::terv::AppContext;
use crate::terv::display::TervProps;

use super::*;



pub struct ConvertPage {
    pub focus_nav: TableFocusNavigator,
}

pub enum ConvertMsg {
    AddConvert,
    UpdateFrom(usize, String),
    UpdateTo(usize, String),
    UpdateFactor(usize, String),
    RemoveConvert(usize),

    KeyPressed(usize, usize, KeyboardEvent),
    MouseClick,
}

impl Component for ConvertPage {
    type Message = ConvertMsg;
    type Properties = TervProps;

    fn create(ctx: &Context<Self>) -> Self {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        ConvertPage {
            focus_nav: TableFocusNavigator::new(terv.convs.len(), 3),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();
        self.focus_nav.build(terv.convs.len(), 3);
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();
        match msg {
            ConvertMsg::AddConvert => {
                terv.convs.push(Conversation::new());
                self.focus_nav.build(terv.convs.len(), 3);
                true
            },
            ConvertMsg::UpdateFrom(index, from) => {
                terv.convs.get_mut(index).unwrap().from = from;
                true
            },
            ConvertMsg::UpdateTo(index, to) => {
                terv.convs.get_mut(index).unwrap().to = to;
                true
            },
            ConvertMsg::UpdateFactor(index, factor) => {
                if let Ok(factor) = factor.parse() {
                    terv.convs.get_mut(index).unwrap().factor = factor;
                }
                true
            },
            ConvertMsg::RemoveConvert(index) => {
                terv.convs.remove(index);
                self.focus_nav.build(terv.convs.len(), 3);
                true
            },
            ConvertMsg::KeyPressed(row, col, e) => {
                self.focus_nav.handle_key(row, col, e);
                false
            },
            ConvertMsg::MouseClick => {
                self.focus_nav.set_edit();
                false
            },
            _ => {true}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let app_data = link.context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        html! {
            <div class="meals">
                <button onclick={link.callback(move |_| ConvertMsg::AddConvert)}>{ "Add Conversation" }</button>
                <table>
                    <tr>
                        <th>{ "From" }</th><th>{ "To" }</th><th>{ "Factor" }</th>
                    </tr>
                    { for terv.convs.iter().enumerate().map(|(index, value)| {
                        let update_from = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            ConvertMsg::UpdateFrom(index, input.value())
                        });

                        let update_to = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            ConvertMsg::UpdateTo(index, input.value())
                        });

                        let update_factor = link.callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            ConvertMsg::UpdateFactor(index, input.value())
                        });

                        let onkeydown = |col| link.callback(move |e: KeyboardEvent| {
                            ConvertMsg::KeyPressed(index, col, e)
                        });

                        let onclick = link.callback(move |_| {
                            ConvertMsg::MouseClick
                        });

                        html! {
                            <tr>
                                <td><input type="text" value={value.from.clone()} onchange={update_from}
                                    onkeydown={onkeydown(0)} ref={self.focus_nav.refs[index][0].clone()} onclick={onclick.clone()} /></td>
                                <td><input type="text" value={value.to.clone()} onchange={update_to}
                                    onkeydown={onkeydown(1)} ref={self.focus_nav.refs[index][1].clone()} onclick={onclick.clone()} /></td>
                                <td><input type="number" min="0" value={value.factor.to_string()} onchange={update_factor}
                                    onkeydown={onkeydown(2)} ref={self.focus_nav.refs[index][2].clone()} onclick={onclick.clone()} /></td>
                                <td><button onclick={link.callback(move |_| ConvertMsg::RemoveConvert(index))}>{ "Remove" }</button></td>
                            </tr>
                        }
                    })}
                </table>
            </div>
        }
    }
}