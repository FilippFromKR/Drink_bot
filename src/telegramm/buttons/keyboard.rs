use macroses::as_array;
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::error::error_handler::{ErrorHandler, ErrorType};
use crate::localization::schemas::LangConfig;

#[derive(as_array, Debug, Deserialize, Serialize, Clone)]
pub enum Keyboard {
    FindCocktail,
    FindIngredient,
    Ingredients,
    Categories,
    WithThisIngredient,
    WithThisCategory,
    DrinkForYou,
    Settings,
}

impl TryFrom<String> for Keyboard {
    type Error = ErrorHandler;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        Ok(Keyboard::as_array()
            .iter()
            .find(|key| key.as_str() == str)
            .ok_or(ErrorHandler {
                msg: format!(
                    "Fail to create keyboard for string, wrong argument:  {}",
                    &str
                ),
                ty: ErrorType::Parse,
            })?
            .clone())
    }
}

pub fn make_keyboard(vec: &[&str]) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for keys in vec.chunks(2) {
        let key = keys
            .iter()
            .map(|&key| InlineKeyboardButton::callback(key, key))
            .collect();
        keyboard.push(key);
    }
    InlineKeyboardMarkup::new(keyboard)
}

pub fn standard_keyboard_as_str_vec(lang: &LangConfig) -> Vec<&str> {
    Keyboard::as_array()
        .iter()
        .map( |key| lang.buttons.main.get(key.as_str()).expect("Unexpected code.").as_str())
        .collect::<Vec<&str>>()
}
