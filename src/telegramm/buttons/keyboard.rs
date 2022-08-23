
use macroses::as_array;
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::error::error_handler::{ErrorHandler, ErrorType};

#[derive(as_array, Debug, Deserialize, Serialize,Clone)]
pub enum Keyboard {
    FindCocktail,
    FindIngredient,
    Ingredients,
    Categories,
    WithThisIngredient,
    WithThisCategory,

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

pub fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for keys in Keyboard::as_array().chunks(2) {
        let key = keys
            .iter()
            .map(|key| InlineKeyboardButton::callback(key.as_str(), key.as_str()))
            .collect();
        keyboard.push(key);
    }
    InlineKeyboardMarkup::new(keyboard)
}