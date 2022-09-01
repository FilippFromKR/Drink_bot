use teloxide::prelude::*;
use teloxide::Bot;

use crate::error::error_handler::ErrorHandler;
use crate::StartCommands;
use crate::telegramm::buttons::keyboard::{make_keyboard, standard_keyboard_as_str_vec};
use crate::telegramm::settings::settings::UserSettings;
use crate::telegramm::state::State;
use crate::telegramm::{LocalDialogue, ReturnTy};

pub struct CommandsHandler;

impl CommandsHandler {
    pub async fn start_commands(bot: &AutoSend<Bot>, dialogue: &LocalDialogue) -> ReturnTy {
        let UserSettings { lang, .. } = CommandsHandler::get_settings(dialogue).await?;
        let keyboard = make_keyboard(&standard_keyboard_as_str_vec(&lang));
        dialogue
            .update(State::CallBack(
                CommandsHandler::get_settings(dialogue).await?,
            ))
            .await?;
        bot.send_message(dialogue.chat_id(), &lang.send_commands)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }
    #[allow(irrefutable_let_patterns)]
    pub async fn back(bot: AutoSend<Bot>, dialogue: LocalDialogue,command: StartCommands)->ReturnTy {
       if let StartCommands::Back = command {
           CommandsHandler::start_commands(&bot, &dialogue).await?;
       }

        Ok(())
    }

    pub async fn get_settings(dialogue: &LocalDialogue) -> Result<UserSettings, ErrorHandler> {
        let state = dialogue.get().await?.expect("Untraceable code.");
        match state.get_settings() {
            Some(settings) => Ok(settings),
            None => Ok(UserSettings::default()),
        }
    }
}
