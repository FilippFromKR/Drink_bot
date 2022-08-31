use std::fmt::{Display, Formatter};
use std::sync::Arc;

use serde::{de, Deserialize};

use crate::cocktails_api::schemas::ToLangDrink;
use crate::ErrorHandler;
use crate::localization::lang::Lang;
use crate::utils::str_builder::StringBuilder;

#[derive(Deserialize, Debug)]
pub struct Ingredient {
    #[serde(rename = "strIngredient")]
    pub name: String,
    #[serde(rename = "strDescription")]
    pub description: Option<String>,
    #[serde(rename = "strType")]
    pub ty: Option<String>,
    #[serde(rename = "strAlcohol", deserialize_with = "deserialize_bool")]
    pub alco: bool,
}

pub struct LangIngredient {
    pub ingredient: Ingredient,
    pub lang: Arc<Lang>,
}

impl ToLangDrink<Ingredient> for LangIngredient {
    type Output = Ingredient;
    fn new(drink: Ingredient, lang: Arc<Lang>) -> Result<Self, ErrorHandler> {
        Ok(Self {
            ingredient: drink,
            lang,
        })
    }
    fn get_drink(&self) -> &Ingredient {
        &self.ingredient
    }
}


impl Display for LangIngredient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = StringBuilder::new()
            .add(&format!("{}: ", self.lang.service_responses.ingredient_name), Some(self.ingredient.name.clone()))
            .add(&format!("{}: ", self.lang.service_responses.description), self.ingredient.description.clone())
            .add(&format!("{}: ", self.lang.service_responses.ty), self.ingredient.ty.clone())
            .add(
                &format!("{}: ", self.lang.service_responses.alco),
                Some(
                    match self.ingredient.alco {
                        true => &self.lang.settings_descriptions.yes,
                        _ => &self.lang.settings_descriptions.no,
                    }
                        .to_string(),
                ),
            )
            .get_str();
        write!(f, "{}", result)
    }
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "Yes" => Ok(true),
        "No" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["Yes", "No"])),
    }
}
