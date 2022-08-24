use macroses::as_array;
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::error::error_handler::{ErrorHandler, ErrorType};

#[derive(as_array, Debug, Deserialize, Serialize, Clone)]
pub enum Keyboard {
    FindCocktail,
    FindIngredient,
    Ingredients,
    Categories,
    WithThisIngredient,
    WithThisCategory,
    CocktailForYou,

}

impl TryFrom<String> for Keyboard {
    type Error = ErrorHandler;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        Ok(Keyboard::as_array()
            .iter()
            .find(|key| key.as_str() == str)
            .ok_or(ErrorHandler {
                msg: "Wrong string argument".to_string(),
                ty: ErrorType::PARSE,
            })?.clone())
    }
}

pub fn make_keyboard(vec:Vec<&str>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for keys in vec.chunks(2) {
        let key = keys
            .into_iter()
            .map(|&key| InlineKeyboardButton::callback(key, key))
            .collect();
        keyboard.push(key);
    }
    InlineKeyboardMarkup::new(keyboard)
}

pub fn as_str_vec() -> Vec<&'static str> {
    Keyboard::as_array()
        .iter()
        .map(|key| key.as_str())
        .collect::<Vec<&str>>()
}