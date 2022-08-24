use core::fmt;
use std::fmt::{Display, format, Formatter, write};
use std::thread;
use std::time::Duration;

use image::DynamicImage;
use log::log;
use serde::{de, Deserialize,Serialize};
use serde_json::Value;
use tokio::sync::oneshot::channel;
use tokio::task;

use crate::error::error_handler::{ErrorHandler, ErrorType};
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



#[derive(Deserialize, Debug)]
pub struct LazyDrink {
    #[serde(rename = "strDrink")]
    pub name: String,
    #[serde(rename = "strDrinkThumb")]
    pub image_url: String,
}




impl Display for LazyDrink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = StringBuilder::new()
            .add("Beverage name: ", Some(self.name.clone()))
            // .add(&format!("{} is looks like: ", self.name), Some(self.image_url.clone()))
            .get_str();
        write!(f, "{}", result)
    }
}

#[derive(Deserialize,Serialize, Debug,Clone)]
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

impl Display for Drink {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = Emojis::Drink.get_randoms(1);
        match result {
            Err(err) => log::error!("{:?}",err),
            Ok(result) => {
                let str_builder = StringBuilder::new()
                    .add(&format!("Here is your cocktail {} :", format!("{}", result.get(0).unwrap_or(&&' '))), Some(self.name.clone()))
                    .add("Type of your drink: ", self.ty.clone())
                    .add("Category is: ", self.category.clone())
                    .add("It is with alcohol:  ", Some(match self.alco {
                        true => "Yes".to_string(),
                        _ => "No".to_string(),
                    }))
                    .add("Use this Glass for it: ", self.glass.clone())
                    .add(&format!("How to cook the {}: ", self.name), self.instructions.clone())
                    .add_many(&self.ingredients);


                write!(f, "{}", str_builder.get_str());
            }
        };
        Ok(())
    }
}


impl TryFrom<Value> for Drink {
    type Error = ErrorHandler;

    fn try_from(input: Value) -> Result<Drink, Self::Error> {
        Ok(Drink {
            name: {
                match input.get(NAME.to_owned()) {
                    Some(value) => serde_json::from_value::<String>(value.clone())?,
                    None => Err(ErrorHandler { msg: "Service doesn't have a drink name.".to_string(), ty: ErrorType::SERVICE })?,
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
                loop {
                    match serde_json::from_value::<Option<String>>(input.get(format!("{}{}", INGREDIENT, counter)).unwrap().clone())? {
                        Some(ingr) => {
                            let measure = match input.get(format!("{}{}", MEASURE, counter)) {
                                Some(value) => serde_json::from_value::<Option<String>>(value.clone())?,
                                None => None,
                            };
                            vec.push((ingr, measure));
                            counter += 1;
                        }
                        None => break,
                    }
                }
                vec
            },
        })
    }
}