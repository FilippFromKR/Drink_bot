use serde::{Deserialize,Serialize};
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

}