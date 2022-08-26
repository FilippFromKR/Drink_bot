use std::fmt::{Display, Formatter};

use crate::error::error_handler::{ErrorHandler, ErrorType};
use crate::utils::str_builder::StringBuilder;
use macroses::as_array;
use serde::{Deserialize, Serialize};

#[derive(as_array, Deserialize, Serialize, Clone, Debug)]
pub enum SettingsKeyboard {
    Name,
    Images,
    MessageLimit,
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
}

impl Display for UserSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = StringBuilder::new()
            .add("As far as i know, your name is: ", Some(self.name.clone().unwrap_or_else(||"Bro".to_string())))
            .add("This parameter defines if the images will be shown or not. Current configuration is: ", Some(match self.send_image {
                true => "Yes".to_owned(),
                false => "No".to_owned()
            }))
            .add("Limit defines the max value of received messages at once. \
            Some responses could include a dozens of it, \
            we predict that it may be annoying. \
            The algorithm chooses a random slice of all results, so you will not lose anything, so feel free to configure it. Current amount of messages is:  ", Some(self.limit_of_messages.to_string()))

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
        }
    }
}
