use std::fmt::{Display, Formatter};

use serde::{de, Deserialize};

use crate::utils::str_builder::StringBuilder;

#[derive(Deserialize)]
pub struct LazyIngredient {
    #[serde(rename = "strIngredient")]
    pub name: String,
    #[serde(rename = "strDescription")]
    pub description: String,
}

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
///todo:research ho to get rid clone
impl Display for Ingredient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = StringBuilder::new()
            .add("Name of ingredient: ", Some(self.name.clone()))
            .add("Description: ", self.description.clone())
            .add("Type: ", self.ty.clone())
            .add(
                "This is alcohol: ",
                Some(
                    match self.alco {
                        true => "Yes",
                        _ => "No",
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
