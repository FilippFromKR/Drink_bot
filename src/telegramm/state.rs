use crate::cocktails_api::schemas::drink::LangDrink;
use crate::telegramm::settings::settings::{SettingsKeyboard, UserSettings};

/// Todo:create struct with setting and stick it to every State, to handle user likens
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Default)]
pub enum State {
    #[default]
    Start,
    Settings(UserSettings),
    SettingsUpdate(UserSettings, SettingsKeyboard),
    CallBack(UserSettings),
    FindByName(UserSettings),
    FindIngrByName(UserSettings),
    AllIngredients(UserSettings),
    WithIngredient(UserSettings),
    WithCategory(UserSettings),
    CocktailForYou {
        settings: UserSettings,
        game: (String, String),
        all: Vec<LangDrink>,
    },
}

impl State {
    pub fn get_settings(&self) -> Option<UserSettings> {
        match self {
            State::CallBack(setting) => Some(setting.clone()),
            State::FindByName(setting) => Some(setting.clone()),
            State::FindIngrByName(setting) => Some(setting.clone()),
            State::AllIngredients(setting) => Some(setting.clone()),
            State::WithIngredient(setting) => Some(setting.clone()),
            State::WithCategory(setting) => Some(setting.clone()),
            State::CocktailForYou { settings, .. } => Some(settings.clone()),
            State::Settings(setting) => Some(setting.clone()),
            State::SettingsUpdate(settings, ..) => Some(settings.clone()),
            State::Start => None,
        }
    }
}
