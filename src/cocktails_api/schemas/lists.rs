use std::fmt::{Display, Formatter};
use std::sync::Arc;

use serde::Deserialize;

use crate::cocktails_api::schemas::ToLangDrink;
use crate::ErrorHandler;
use crate::localization::lang::Lang;

#[derive(Deserialize)]
pub struct List {
    #[serde(alias = "strCategory", alias = "strGlass", alias = "strIngredient1")]
    pub name: String,
}

pub struct LangList {
    pub list: List,
    pub lang: Arc<Lang>,
}

impl ToLangDrink<List> for LangList {
    type Output = List;
    fn new(drink: List, lang: Arc<Lang>) -> Result<Self, ErrorHandler> {
        Ok(Self {
            list: drink,
            lang,
        })
    }
    fn get_drink(&self) -> &Self::Output {
        &self.list
    }
}



impl Display for LangList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " - {} {}", self.lang.service_responses.beverage_name, self.list.name)
    }
}
