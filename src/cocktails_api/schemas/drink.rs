use std::fmt::{Display, Formatter};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cocktails_api::schemas::ToLangDrink;
use crate::error::error_handler::ErrorHandler;
use crate::error::error_handler::ErrorType;
use crate::localization::lang::Lang;
use crate::utils::str_builder::StringBuilder;
use crate::utils::unicod::Emojis;

pub const MEASURE: &str = "strMeasure";
pub const INSTRUCTIONS: &str = "strInstructions";
pub const NAME: &str = "strDrink";
pub const CATEGORY: &str = "strCategory";
pub const GLASS: &str = "strGlass";
pub const ALCO: &str = "strAlcoholic";
pub const INGREDIENT: &str = "strIngredient";
pub const TY: &str = "strTags";
pub const IMAGE: &str = "strDrinkThumb";

pub trait WithPhoto {
    fn get_url(&self) -> Option<String>;
}

#[derive(Deserialize, Debug)]
pub struct LazyDrink {
    #[serde(rename = "strDrink")]
    pub name: String,
    #[serde(rename = "strDrinkThumb")]
    pub image_url: String,
}

pub struct LangLazyDrink {
    pub lazy: LazyDrink,
    pub lang: Arc<Lang>,

}

impl ToLangDrink<LazyDrink> for LangLazyDrink {
    type Output = LazyDrink;
    fn new(drink: LazyDrink, lang: Arc<Lang>) -> Result<Self, ErrorHandler> {
        Ok(Self {
            lazy: drink,
            lang,
        })
    }
    fn get_drink(&self) -> &LazyDrink {
        &self.lazy
    }
}


impl WithPhoto for LangLazyDrink {
    fn get_url(&self) -> Option<String> {
        Some(self.lazy.image_url.clone())
    }
}

impl Display for LangLazyDrink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = StringBuilder::new()
            .add(&self.lang.service_responses.beverage_name, Some(self.lazy.name.clone()))
            .get_str();
        write!(f, "{}", result)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Drink {
    pub name: String,
    pub ty: Option<String>,
    pub category: Option<String>,
    pub alco: bool,
    pub glass: Option<String>,
    pub instructions: Option<String>,
    pub image: Option<String>,
    pub ingredients: Vec<(String, Option<String>)>,

}

impl ToLangDrink<Value> for LangDrink {
    type Output = Drink;

    fn new(drink: Value, lang: Arc<Lang>) -> Result<Self, ErrorHandler> {
        Ok(Self {
            drink: Self::drink_from_value(&drink)?,
            lang,
        })
    }
    fn get_drink(&self) -> &Drink {
        &self.drink
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LangDrink {
    pub drink: Drink,
    pub lang: Arc<Lang>,
}

impl LangDrink {
    pub fn new(value: Value, lang: Arc<Lang>) -> Result<Self, ErrorHandler> {
        Ok(Self {
            drink: Self::drink_from_value(&value)?,
            lang,
        })
    }


    fn drink_from_value(input: &Value) -> Result<Drink, ErrorHandler> {
        Ok(Drink {
            name: {
                match input.get(NAME.to_owned()) {
                    Some(value) => serde_json::from_value::<String>(value.clone())?,
                    None => Err(ErrorHandler {
                        msg: "Service doesn't have a drink name.".to_string(),
                        ty: ErrorType::Service,
                    })?,
                }
            },
            ty: {
                match input.get(TY.to_owned()) {
                    Some(value) => serde_json::from_value::<Option<String>>(value.clone())?,
                    None => None,
                }
            },
            category: {
                match input.get(CATEGORY.to_owned()) {
                    Some(value) => serde_json::from_value::<Option<String>>(value.clone())?,
                    None => None,
                }
            },
            alco: {
                match input.get(ALCO.to_owned()) {
                    Some(value) => value.to_string().contains("Alcoholic"),
                    None => false,
                }
            },
            glass: {
                match input.get(GLASS.to_owned()) {
                    Some(value) => serde_json::from_value::<Option<String>>(value.clone())?,
                    None => None,
                }
            },
            instructions: {
                match input.get(INSTRUCTIONS.to_owned()) {
                    Some(value) => serde_json::from_value::<Option<String>>(value.clone())?,
                    None => None,
                }
            },
            image: {
                match input.get(IMAGE.to_owned()) {
                    Some(value) => serde_json::from_value::<Option<String>>(value.clone())?,
                    None => None,
                }
            },
            ingredients: {
                let mut vec: Vec<(String, Option<String>)> = vec![];
                let mut counter = 1;

                while let Some(ingr) = serde_json::from_value::<Option<String>>(
                    input
                        .get(format!("{}{}", INGREDIENT, counter))
                        .unwrap()
                        .clone(),
                )? {
                    let measure = match input.get(format!("{}{}", MEASURE, counter)) {
                        Some(value) => serde_json::from_value::<Option<String>>(value.clone())?,
                        None => None,
                    };
                    vec.push((ingr, measure));
                    counter += 1;
                }

                vec
            },
        })
    }
}

impl WithPhoto for LangDrink {
    fn get_url(&self) -> Option<String> {
        self.get_drink().image.as_ref().cloned()
    }
}


impl Display for LangDrink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = Emojis::Drink.random().unwrap_or(&' ');
        let drink = self.get_drink();
        let str_builder = StringBuilder::new()
            .add(
                &format!("{} {} :", &self.lang.service_responses.beverage_name, result),
                Some(drink.name.clone()),
            )
            .add(&format!("{}: ", &self.lang.service_responses.ty), drink.ty.clone())
            .add(&format!("{}: ", &self.lang.service_responses.category), drink.category.clone())
            .add(
                &format!("{}: ",&self.lang.service_responses.alco),
                Some(match drink.alco {
                    true => self.lang.settings_descriptions.yes.clone(),
                    _ => self.lang.settings_descriptions.no.clone(),
                }),
            )
            .add(&format!("{}: ", self.lang.service_responses.glass), drink.glass.clone())
            .add(
                &format!("{}: ", self.lang.service_responses.cook), drink.instructions.clone(),
            )
            .add_many(&drink.ingredients);

        write!(f, "{}", str_builder.get_str())
    }
}

