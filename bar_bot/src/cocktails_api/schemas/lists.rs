use serde::Deserialize;
use std::fmt::{Display, Formatter};
#[derive(Deserialize)]
pub struct List {
    #[serde(alias = "strCategory", alias = "strGlass", alias = "strIngredient1")]
    pub name: String,
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " - Name: {}", self.name)
    }
}
