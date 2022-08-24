use serde::{Deserialize,Serialize};
use crate::coctails_api::schemas::drink::Drink;

/// Todo:create struct with setting and stick it to every State, to handle user likens
#[derive(Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
pub enum State {
    #[default]
    Start,
    Finish,
    CallBack,
    FindByName,
    FindIngrByName,
    AllIngredients,
    WithIngredient,
    WithCategory,
    CocktailForYou2{game:(String,String),all:Vec<Drink>},

}