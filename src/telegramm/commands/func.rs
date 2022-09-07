use teloxide::Bot;
use teloxide::prelude::*;

use crate::{MessageHandler, StartCommands};
use crate::cocktails_api::schemas::drink::LangDrink;
use crate::cocktails_api::services::coctail_service::DrinksService;
use crate::error::error_handler::ErrorHandler;
use crate::telegramm::{LocalDialogue, ReturnTy};
use crate::telegramm::buttons::keyboard::{make_keyboard, standard_keyboard_as_str_vec};
use crate::telegramm::settings::settings::UserSettings;
use crate::telegramm::state::State;
use crate::utils::helpers::{random_english_character, random_num_in_range, write_to_file};

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
    pub async fn handle_commands(bot: AutoSend<Bot>, dialogue: LocalDialogue, command: StartCommands) -> ReturnTy {
        match command {
            StartCommands::Back => Self::start_commands(&bot,&dialogue).await?,
            StartCommands::Random => Self::random(&bot,&dialogue).await?,
            StartCommands::SuggestionAndBags => Self::suggestion_bugs(&bot,&dialogue).await?,
        };
        Ok(())
    }
    async fn suggestion_bugs(bot: &AutoSend<Bot>, dialogue: &LocalDialogue) -> ReturnTy {
        let settings = CommandsHandler::get_settings(&dialogue).await?;
        bot.send_message(dialogue.chat_id(), &settings.lang.todo.suggestion).await;
        dialogue.update(State::Suggestion(settings)).await?;
        Ok(())
    }

    async fn random(bot: &AutoSend<Bot>, dialogue: &LocalDialogue) -> ReturnTy {
        let mut drinks = Self::till_get(&dialogue).await?;
        let random_num = random_num_in_range(0, drinks.len());
        MessageHandler::send_vec_with_photo(&vec![drinks.remove(random_num)], &bot, &dialogue).await?;
        Ok(())
    }
    async fn till_get(dialogue: &LocalDialogue) -> Result<Vec<LangDrink>, ErrorHandler> {
        let UserSettings { lang, .. } = CommandsHandler::get_settings(&dialogue).await?;
        loop {
            match DrinksService::search_by_first_letter(
                &random_english_character()?,
                lang.clone(),
            )
                .await? {
                Some(drinks) => { return Ok(drinks); }
                None => {}
            };
        }
    }

    pub async fn get_settings(dialogue: &LocalDialogue) -> Result<UserSettings, ErrorHandler> {
        let state = dialogue.get().await?.expect("Untraceable code.");
        match state.get_settings() {
            Some(settings) => Ok(settings),
            None => Ok(UserSettings::default()),
        }
    }
}
