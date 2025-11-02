use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::rc::Rc;

use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::HtmlInputElement;
use gloo::console::log;
use yew::html::Scope;

use crate::beszer::display::round;
use crate::recipe::subrecipe::SubRecipe;
use crate::terv::{AppContext, AppData, Terv};
use crate::backend::keyboard::TableFocusNavigator;
use crate::terv::display::TervProps;
use crate::convert::Convert;
use crate::input::{GetDataVec, Input, FNP, create_gdv};

use super::*;


#[derive(Debug)]
pub struct RecipePage {
    pub current_recipe: Option<usize>,
    pub focus_nav_ing: Rc<RefCell<TableFocusNavigator>>,
    pub focus_nav_sub: Rc<RefCell<TableFocusNavigator>>,
}

impl RecipePage {
    pub fn search_recipe<'a, 'b, 'c>(&'a mut self, name: &'b str, terv: &'c Terv) -> Result<&'c Recipe, String> {
        for (index, recipe) in terv.recipes.iter().enumerate() {
            if recipe.name.contains(name) {
                self.current_recipe = Some(index);
                return Ok(recipe);
            }
        }
        Result::Err(format!("Nem található ilyen recept: {name}"))
    }

    fn get_curr_recipe_mut<'a, 'b>(&'a self, terv: &'b mut Terv) -> &'b mut Recipe {
        terv.recipes.get_mut(self.current_recipe.expect("Nem lehet rossz! 004")).expect("Nem lehet rossz! 005")
    }

    fn get_curr_recipe<'a, 'b>(&'a self, terv: &'b Terv) -> &'b Recipe {
        terv.recipes.get(self.current_recipe.expect("Nem lehet rossz! 004")).expect("Nem lehet rossz! 005")
    }
}

// Display

pub enum RecipeFocusNav {
    SubRec,
    Ingred,
}

pub enum RecipeMsg {
    AddRecipe,
    SearchRecipe(String),
    UpdateName(String),
    UpdateNumber(String),
    SetSensMode(String),
    UpdateSensitivities(String),
    UpdateSubRecipeName(usize, String),
    UpdateScale(usize, String),
    UpdateIngredientName(usize, String),
    UpdateQuantity(usize, String),
    UpdateUnit(usize, String),
    RemoveIngredient(usize),
    RemoveSubRecipe(usize),
    AddIngredient,
    AddSubRecipe,
    RemoveRecipe,
    //KeyPressed(usize, usize, KeyboardEvent, RecipeFocusNav),
    MouseClick,
}

impl Component for RecipePage {
    type Message = RecipeMsg;
    type Properties = TervProps;

    fn create(ctx: &Context<Self>) -> Self {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        let mut focus_nav_ing = TableFocusNavigator::new(0, 3);
        let mut focus_nav_sub = TableFocusNavigator::new(0, 2);
        if let Some(recipe) = terv.recipes.get(0) {
            focus_nav_ing.build(recipe.ingredients.len(), 3);
            focus_nav_sub.build(recipe.sub_recipes.len(), 2);
        }
        RecipePage {
            current_recipe: if terv.recipes.len() > 0 { Some(0) } else { None },
            focus_nav_ing: Rc::new(RefCell::new(focus_nav_ing)),
            focus_nav_sub: Rc::new(RefCell::new(focus_nav_sub)),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        if let Some(recipe) = terv.recipes.get(0) {
            self.focus_nav_ing.borrow_mut().build(recipe.ingredients.len(), 3);
            self.focus_nav_sub.borrow_mut().build(recipe.sub_recipes.len(), 2);
        }
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let mut terv = app_data.terv.borrow_mut();

        match msg {
            RecipeMsg::AddRecipe => {
                terv.recipes.push(Recipe::default());
                self.current_recipe = Some(terv.recipes.len() - 1);
                self.rebuild_focus_navs(&terv);
                true
            },
            RecipeMsg::SearchRecipe(name) => {
                if let Ok(recipe) = self.search_recipe(name.as_str(), &terv) {
                    self.rebuild_focus_navs(&terv);
                }
                true
            },
            RecipeMsg::UpdateName(name) => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.name = name;
                println!("{:?}", recipe);
                true
            },
            RecipeMsg::UpdateNumber(number) => {
                if let Ok(number) = number.parse() {
                    let recipe = self.get_curr_recipe_mut(&mut terv);
                    recipe.number = number;
                }
                true
            },
            RecipeMsg::SetSensMode(sens_mode) => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.sens_mode = sens_mode.parse().unwrap_or(SensMode::default());
                true
            },
            RecipeMsg::UpdateSensitivities(sens) => {
                log!("sens:", &sens);
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.sens = Sensitivities::from(&sens);
                true
            },
            RecipeMsg::UpdateSubRecipeName(index, name) => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.sub_recipes[index].name = name;
                true
            },
            RecipeMsg::UpdateScale(index, scale) => {
                if let Ok(scale) = scale.parse() {
                    let recipe = self.get_curr_recipe_mut(&mut terv);
                    recipe.sub_recipes[index].scale = scale;
                } 
                true
            },
            RecipeMsg::UpdateIngredientName(index, name) => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 006").name = name;
                true
            },
            RecipeMsg::UpdateQuantity(index, quantity) => {
                if let Ok(quantity) = quantity.parse() {
                    let recipe = self.get_curr_recipe_mut(&mut terv);
                    recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 003").quantity = quantity;
                }
                true
            },
            RecipeMsg::UpdateUnit(index, unit) => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.ingredients.get_mut(index).expect("Nem lehet rossz! 002").unit = unit;
                true
            },
            RecipeMsg::AddIngredient => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.ingredients.push(Ingredient::new());
                self.rebuild_ing_focus_nav(&terv);
                true
            },
            RecipeMsg::AddSubRecipe => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.sub_recipes.push(SubRecipe::default());
                self.rebuild_sub_focus_nav(&terv);
                true
            },
            RecipeMsg::RemoveIngredient(index) => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.ingredients.remove(index);
                self.rebuild_ing_focus_nav(&terv);
                true
            },
            RecipeMsg::RemoveSubRecipe(index) => {
                let recipe = self.get_curr_recipe_mut(&mut terv);
                recipe.sub_recipes.remove(index);
                self.rebuild_sub_focus_nav(&terv);
                true
            },
            RecipeMsg::RemoveRecipe => {
                terv.recipes.remove(self.current_recipe.expect("Nem lehet rossz! 001"));
                if terv.recipes.len() > 0 {
                    self.current_recipe = Some(0);
                    self.rebuild_focus_navs(&terv);
                } else {
                    self.current_recipe = None;
                }
                true
            },
            // RecipeMsg::KeyPressed(row, col, e, rfn) => {
            //     match rfn {
            //         RecipeFocusNav::Ingred => self.focus_nav_ing.handle_key(row, col, e),
            //         RecipeFocusNav::SubRec => self.focus_nav_sub.handle_key(row, col, e)
            //     };
            //     false
            // },
            // RecipeMsg::MouseClick => {
            //     self.focus_nav_ing.set_edit();
            //     self.focus_nav_sub.set_edit();
            //     false
            // },
            _ => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let app_data = ctx.link().context::<AppContext>(Callback::noop()).unwrap().0;
        let terv = app_data.terv.borrow();

        let mut osszetevo_name_list: Vec<VNode> = Vec::new();
        self.make_osszetevo_list(&terv, &mut osszetevo_name_list);

        let recipe_list = self.make_recipe_list(&terv);
        
        html! {
            <div class="recipes">

                { self.toolbar_display(&terv, link, recipe_list, osszetevo_name_list) }

                <div class="current_recipe">
                    if let Some(recipe_index) = self.current_recipe {
                        if let Some(recipe) = terv.recipes.get(recipe_index) {
                            <button onclick={link.callback(move |_| RecipeMsg::RemoveRecipe)}>{ "Remove Recipe" }</button>
                            <table>
                                { self.general_display(&terv, link, recipe) }

                                { self.sub_recipe_display(&terv, link, recipe, app_data.clone()) }

                                { self.ingredient_display(&terv, link, recipe, app_data.clone()) }
                            </table>
                    }}
                </div>
            </div>
        }
    }
}

impl RecipePage {
    pub fn rebuild_focus_navs(&mut self, terv: &Terv) {
        self.rebuild_ing_focus_nav(terv);
        self.rebuild_sub_focus_nav(terv);
    }
    pub fn rebuild_ing_focus_nav(&mut self, terv: &Terv) {
        if let Some(_) = self.current_recipe {
            let recipe = self.get_curr_recipe(terv);
            self.focus_nav_ing.borrow_mut().build(recipe.ingredients.len(), 3);
        } else {
            self.focus_nav_ing.borrow_mut().build(0, 3);
        }
    }
    pub fn rebuild_sub_focus_nav(&mut self, terv: &Terv) {
        if let Some(_) = self.current_recipe {
            let recipe = self.get_curr_recipe(terv);
            self.focus_nav_sub.borrow_mut().build(recipe.sub_recipes.len(), 2);
        } else {
            self.focus_nav_sub.borrow_mut().build(0, 2);
        }
    }

    fn make_osszetevo_list(&self, terv: &Terv, list: &mut Vec<VNode>) {
        if let Some(recipe_index) = self.current_recipe {
            if let Some(recipe) = terv.recipes.get(recipe_index) {
                let mut all_osszetevo_name_list: Vec<&String> = terv.osszetevok.iter().map(|ossz| &ossz.name).collect();
                terv.recipes.iter().for_each(|recipe| {
                    recipe.ingredients.iter().for_each(|ingredient| all_osszetevo_name_list.push(&ingredient.name))
                });
                all_osszetevo_name_list.sort();
                all_osszetevo_name_list.dedup();
                *list = all_osszetevo_name_list.iter().map(|&rec_ossz| {
                    if !recipe.ingredients.iter().map(|x| &x.name).collect::<Vec<&String>>().contains(&rec_ossz) {
                        html! {<option value={rec_ossz.clone()} />}
                    } else {
                        html! {}
                    }
                }).collect();
            }
        }
    }

    fn make_recipe_list(&self, terv: &Terv) -> Vec<VNode> {
        terv.recipes.iter().map(|value| {
            let ec = value.get_errors(&terv).count();
            let wc = value.get_warnings(&terv).count();
            html! {<option value={value.name.clone()} >
                <p>{ value.name.clone() }</p>
                if ec > 0 {<p class="err">{ format!(" {}e", ec)}</p>}
                if wc > 0 {<p class="warn">{format!(" {}w", wc)}</p>}
                </option>}
        }).collect()
    }

    fn toolbar_display(&self, terv: &Terv, link: &Scope<RecipePage>, 
        recipe_list: Vec<VNode>, o_n_l: Vec<VNode>) -> VNode 
    {
        html!{
            <div class="toolbar">
                <button onclick={link.callback(|_| RecipeMsg::AddRecipe)}>{ "Add recipe" }</button>
                //<label for="recipe_search">{ "Kerressen rá egy receptre" }</label>
                <input type="text" list="recipe_list" id="recipe_search" name="recipe_search" oninput={
                    link.callback(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::SearchRecipe(input.value())})} />
                <datalist id="recipe_list">
                    { recipe_list.clone() }
                </datalist>
                <select list="recipe_list" onchange={
                        link.callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        RecipeMsg::SearchRecipe(input.value())})} >
                        { recipe_list.clone() }
                </select>
                <datalist id="osszetevo_name_list">
                    { o_n_l }
                </datalist>
                <p>{ format!("Kiválasztott recept: {}", match self.current_recipe {
                    Some(index) => {terv.recipes.get(index).unwrap().name.clone()},
                    None => {String::from("Nem található ilyen recept!")}
                }) }</p>
            </div>
        }
    }

    fn general_display(&self, terv: &Terv, link: &Scope<RecipePage>, 
        recipe: &Recipe) -> VNode 
    {
        html!{<>
            <tr>
                <th>{ "Név:" }</th>
                <th><input type="text" value={recipe.name.clone()} onchange={link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::UpdateName(input.value())})} /></th>
            </tr>
            <tr>
                <th>{ "Létszám:" }</th>
                <th><input type="number" min="0" step="1" value={if recipe.number != 0 {recipe.number.to_string()} else {"".to_string()}} 
                    onchange={link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::UpdateNumber(input.value())})} /></th>
                if recipe.number == 0 {
                    <td class="warn">{ "A létszám nulla" }</td>
                }
            </tr>
            <tr>
                <th>{ "Érzékenység mód" }</th>
                <td><select onchange={ link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::SetSensMode(input.value())})} >
                    <option value={ "FO" }>{ "Összetevőkből" }</option>
                    <option value={ "FR" }>{ "Receptből" }</option>
                    <option value={ "B" }>{ "Mindkettőből" }</option>
                    <option value={ "N" }>{ "Nincs" }</option>
                </select></td>
                <td><input type="text" value={recipe.sens.to_string()} 
                    disabled={if recipe.sens_mode == SensMode::FromOssz || recipe.sens_mode == SensMode::None {true} else {false}}
                    onchange={link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::UpdateSensitivities(input.value())})} /></td>
            </tr>
        </>}
    }

    fn sub_recipe_display(&self, terv: &Terv, link: &Scope<RecipePage>, 
        recipe: &Recipe, app_data: Rc<AppData>) -> VNode 
    {
        html!{<>
            <tr>
                <th>{ "Alrecept" }</th><th>{ "Szorzó" }</th>
            </tr>
            { for recipe.sub_recipes.iter().enumerate().map(|(index, sub_recipe)| {
                let update_name = link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::UpdateSubRecipeName(index, input.value())
                });

                let update_scale = link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::UpdateScale(index, input.value())
                });

                // let onkeydown = |col| link.callback(move |e: KeyboardEvent| {
                //     RecipeMsg::KeyPressed(index, col, e, RecipeFocusNav::SubRec)
                // });

                // let onclick = link.callback(move |_| {
                //     RecipeMsg::MouseClick
                // });
                let cr = self.current_recipe.unwrap();
                let fnp = FNP::new(index, 0, 
                    self.focus_nav_sub.clone(), 
                    create_gdv(move |t| t.recipes[cr].sub_recipes.as_mut()),
                    app_data.clone()
                );

                html! {
                    <tr>
                        <td>
                            <Input<SubRecipe> r#type="text" list="recipe_list" value={sub_recipe.name.clone()} onchange={update_name}
                            fnp = {fnp.clone_set_colomn(0)}
                            // fnp={ FNP {row: index, colomn: 0, app_data: app_data.clone(),
                            //     focus_nav: self.focus_nav_sub.clone(),
                            //     get_data_vec: create_gdv(move |t| t.recipes[cr].ingredients.as_mut()),
                            // }} 
                            />
                        </td>
                        <td>
                            <Input<SubRecipe> r#type="number" value={sub_recipe.scale.to_string()} onchange={update_scale}
                            fnp={fnp.clone_set_colomn(1)} />
                        </td>
                        
                        // <td><input type="text" list="recipe_list" value={sub_recipe.name.clone()} onchange={update_name}
                        //     onkeydown={onkeydown(0)} ref={self.focus_nav_sub.refs[index][0].clone()} onclick={onclick.clone()} /></td>
                        // <td><input type="number" step="any" value={sub_recipe.scale.to_string()} onchange={update_scale}
                        //     onkeydown={onkeydown(1)} ref={self.focus_nav_sub.refs[index][1].clone()} onclick={onclick.clone()} /></td>
                        <td><button onclick={link.callback(move |_| RecipeMsg::RemoveSubRecipe(index))}>{ "Remove" }</button></td>
                        {for sub_recipe.get_errors(&terv).iter().map(|error| html!{
                            <td class="err">{ match error {
                                EW::Owned(text) => text.clone(),
                                _ => "hiba a kódban".to_string(),
                            } }</td>
                        })}
                        {for sub_recipe.get_warnings(&terv).iter().map(|warning| html!{
                            <td class="warn">{ match warning {
                                EW::Owned(text) => text.clone(),
                                _ => "hiba a kódban".to_string(),
                            } }</td>
                        })}
                    </tr>
                }
            })}
            <tr>
                <td><button onclick={link.callback(move |_| RecipeMsg::AddSubRecipe)}>{ "Add SubRecipe" }</button></td>
            </tr>
        </>}
    }

    fn ingredient_display(&self, terv: &Terv, link: &Scope<RecipePage>, 
        recipe: &Recipe, app_data: Rc<AppData>) -> VNode 
    {
        html!{<>
            <tr>
                <th>{ "Összetevő" }</th><th>{ "Mennyiség" }</th><th>{ "Mértékegység" }</th><th>{ "/fő" }</th>
            </tr>
            { for recipe.ingredients.iter().enumerate().map(|(index, value)| {
                let update_name = link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::UpdateIngredientName(index, input.value())
                });

                let update_quantity = link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::UpdateQuantity(index, input.value())
                });

                let update_unit = link.callback(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    RecipeMsg::UpdateUnit(index, input.value())
                });

                // let onkeydown = |col| link.callback(move |e: KeyboardEvent| {
                //     RecipeMsg::KeyPressed(index, col, e, RecipeFocusNav::Ingred)
                // });

                // let onclick = link.callback(move |_| {
                //     RecipeMsg::MouseClick
                // });

                let osszetevo = terv.osszetevok.by_name(&value.name);

                let cr = self.current_recipe.unwrap();
                let fnp = FNP::new(index, 0, 
                    self.focus_nav_ing.clone(), 
                    create_gdv(move |t| t.recipes[cr].ingredients.as_mut()),
                    app_data.clone()
                );

                html! {
                    <tr>
                        <td><Input<Ingredient> r#type="text" list="osszetevo_name_list" value={value.name.clone()} onchange={update_name}
                            fnp={fnp.clone_set_colomn(0)} /></td>
                        <td><Input<Ingredient> r#type="number" onchange={update_quantity}
                            value={if value.quantity != 0.0 {value.quantity.to_string()} else {"".to_string()}} 
                            fnp={fnp.clone_set_colomn(0)} /></td>
                        <td><Input<Ingredient> r#type="text" value={value.unit.clone()} onchange={update_unit} 
                            fnp={fnp.clone_set_colomn(0)} /></td>

                        // <td><input type="text" list="osszetevo_name_list" value={value.name.clone()} onchange={update_name}
                        //     onkeydown={onkeydown(0)} ref={self.focus_nav_ing.refs[index][0].clone()} onclick={onclick.clone()} /></td>
                        // <td><input type="number" step="any" value={if value.quantity != 0.0 {value.quantity.to_string()} else {"".to_string()}} onchange={update_quantity}
                        //     onkeydown={onkeydown(1)} ref={self.focus_nav_ing.refs[index][1].clone()} onclick={onclick.clone()} /></td>
                        // <td><input type="text" value={value.unit.clone()} onchange={update_unit}
                        //     onkeydown={onkeydown(2)} ref={self.focus_nav_ing.refs[index][2].clone()} onclick={onclick.clone()} /></td>

                        <td>{ format!("{} {}", round(value.quantity / recipe.number as f64, 3), value.unit) }</td>
                        <td><button onclick={link.callback(move |_| RecipeMsg::RemoveIngredient(index))}>{ "Remove" }</button></td>
                        {for value.get_errors(&terv).iter().map(|error| html!{
                            <td class="err">{ match error {
                                EW::Owned(text) => text.clone(),
                                _ => "hiba a kódban".to_string(),
                            } }</td>
                        })}
                        {for value.get_warnings(&terv).iter().map(|warning| html!{
                            <td class="warn">{ match warning {
                                EW::Owned(text) => text.clone(),
                                _ => "hiba a kódban".to_string(),
                            } }</td>
                        })}
                        
                        // {match osszetevo {
                        //     None => html!{<td class="err">{ "Az összetevő nem található" }</td>},
                        //     Some(osszetevo) => {
                        //         if let None = value.convert(&osszetevo.unit, &terv.convs) {
                        //             html!{<td class="err">{ format!("'{}' nem váltható át '{}'-ra/re", value.unit, osszetevo.unit) }</td>}
                        //         } else {html!{}}
                        //     },
                        // } }

                        // if value.quantity == 0.0 {
                        //     <td class="warn">{ "A mennyiség nulla" }</td>
                        // }
                        // if value.unit == "" {
                        //     <td class="warn">{ "Nincs mértékegység" }</td>
                        // }
                    </tr>
                }
            })}
            <tr>
                <td><button onclick={link.callback(move |_| RecipeMsg::AddIngredient)}>{ "Add Ingredient" }</button></td>
            </tr>
        </>}
    }
}
