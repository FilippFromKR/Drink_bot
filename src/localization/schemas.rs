use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LangConfig {
    pub send_commands: String,
    pub fail_messages: FailMessages,
    pub todo: Todo,
    pub service_responses: ServiceResponses,
    pub settings_descriptions: SettingsDescriptions,
    pub buttons: MainButtons,
}

#[derive(Debug, Deserialize)]
pub struct MainButtons {
    pub main: HashMap<String, String>,
    pub settings: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct FailMessages {
    pub unexpected: String,
    pub game_limit: String,
    pub need_number: String,
    pub non_results: String,
    pub wrong_category: String,
    pub wrong_ingredient: String,
    pub try_again: String,
    pub suggestion: String,
}

#[derive(Debug, Deserialize)]
pub struct Todo {
    pub settings_set_limit: String,
    pub settings_set_name: String,
    pub game_choose: String,
    pub category_write: String,
    pub ingredient_write: String,
    pub find_cocktails_write: String,
    pub find_ingredient_write: String,
    pub suggestion: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceResponses {
    pub beverage_name: String,
    pub ty: String,
    pub category: String,
    pub alco: String,
    pub cook: String,
    pub glass: String,
    pub ingredient_name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsDescriptions {
    pub name: String,
    pub lang: String,
    pub lang_ukr: String,
    pub lang_eng: String,
    pub yes: String,
    pub no: String,
    pub image: String,
    pub limit: String,
    pub limit_name: String,
}
