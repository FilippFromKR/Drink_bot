use std::sync::Arc;

use serde::Deserialize;

use crate::localization::lang::Lang;
use crate::ErrorHandler;

pub mod drink;
pub mod ingredient;
pub mod lists;

pub trait ToLangDrink<T> {
    type Output;
    fn new(drink: T, lang: Arc<Lang>) -> Result<Self, ErrorHandler>
    where
        Self: Sized;
    fn get_drink(&self) -> &Self::Output;
}

#[derive(Deserialize, Debug)]
pub struct RawDrinkListSchema<T> {
    #[serde(bound(deserialize = "Vec<T>:Deserialize<'de>"), alias = "ingredients")]
    pub drinks: Option<Vec<T>>,
}

impl<T> RawDrinkListSchema<T> {
    pub fn is_empty(&self) -> bool {
        self.drinks.is_none()
    }
}
