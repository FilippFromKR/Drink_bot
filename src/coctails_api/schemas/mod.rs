use serde::de::DeserializeOwned;
use serde::Deserialize;

pub mod drink;
pub mod ingredient;
pub mod lists;


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