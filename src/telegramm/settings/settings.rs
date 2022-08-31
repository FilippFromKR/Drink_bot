use std::fmt::{Display, Formatter};

use crate::error::error_handler::{ErrorHandler, ErrorType};
use crate::localization::lang::Lang;
use crate::utils::str_builder::StringBuilder;
use macroses::as_array;
use serde::{Deserialize, Serialize};

#[derive(as_array, Deserialize, Serialize, Clone, Debug)]
pub enum SettingsKeyboard {
    Name,
    Images,
    MessageLimit,
    Lang,
    Back,
}

impl TryFrom<&str> for SettingsKeyboard {
    type Error = ErrorHandler;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = SettingsKeyboard::as_array()
            .iter()
            .find(|key| key.as_str() == value);
        Ok(result
            .ok_or(ErrorHandler {
                msg: "Wrong argument to parse enum Settings Keyboard".to_string(),
                ty: ErrorType::Parse,
            })?
            .clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSettings {
    pub name: Option<String>,
    pub send_image: bool,
    pub limit_of_messages: u32,
    pub lang: Lang,
}

impl Display for UserSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = StringBuilder::new()
            .add(
                &self.lang.settings_descriptions.name,
                Some(self.name.clone().unwrap_or_else(|| "Bro".to_string())),
            )
            .add(
                &self.lang.settings_descriptions.image,
                Some(match self.send_image {
                    true => self.lang.settings_descriptions.yes.clone(),
                    false => self.lang.settings_descriptions.no.clone(),
                }),
            )
            .add(
                &self.lang.settings_descriptions.lang,
                Some(match self.lang {
                    Lang::Ukr => self.lang.settings_descriptions.lang_ukr.clone(),
                    Lang::Eng => self.lang.settings_descriptions.lang_eng.clone(),
                }),
            )
            .add(
                &self.lang.settings_descriptions.limit,
                Some(self.limit_of_messages.to_string()),
            )
            .get_str();
        write!(f, "{}", result)
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            name: Some("Dear".to_string()),
            send_image: true,
            limit_of_messages: 10,
            lang: Lang::Ukr,
        }
    }
}
