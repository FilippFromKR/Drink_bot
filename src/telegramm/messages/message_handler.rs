use std::fmt::Display;

use teloxide::Bot;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{AutoSend, Message, Requester};
use teloxide::types::InputFile;
use url::Url;

use crate::cocktails_api::schemas::drink::WithPhoto;
use crate::cocktails_api::services::coctail_service::DrinksService;
use crate::telegramm::{LocalDialogue, ReturnTy};
use crate::telegramm::buttons::callback_handler::CallBackHandler;
use crate::telegramm::buttons::keyboard::{make_keyboard, standard_keyboard_as_str_vec};
use crate::telegramm::commands::func::CommandsHandler;
use crate::telegramm::settings::settings::{SettingsKeyboard, UserSettings};
use crate::telegramm::state::State;
use crate::utils::helpers::{random_num_in_range, vec_to_string};
use crate::utils::unicod::Emojis;

pub const TELEGRAMM_CHAR_LIMIT: usize = 4096;

pub struct MessageHandler;

impl MessageHandler {
    pub async fn unexpected_message(
        message: Message,
        bot: AutoSend<Bot>,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        let settings = CommandsHandler::get_settings(&dialogue).await?;
        bot.send_message(
            dialogue.chat_id(),
            format!(
                "{} {}, {} {} ",
                settings.name.unwrap_or_else(|| "".to_string()),
                Emojis::ShitHappens.random()?,
                &settings.lang.fail_messages.unexpected,
                message.text().unwrap_or("")
            ),
        )
            .await?;
        CommandsHandler::start_commands(&bot, &dialogue).await?;
        Ok(())
    }
    pub async fn settings(
        message: Message,
        bot: AutoSend<Bot>,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        if let Some(message) = message.text() {
            let settings = if let State::SettingsUpdate(mut settings, SettingsKeyboard::Name) =
            dialogue.get().await?.expect("Untraceable code.")
            {
                settings.name = Some(message.to_owned());
                settings
            } else if let State::SettingsUpdate(mut settings, SettingsKeyboard::MessageLimit) =
            dialogue.get().await?.expect("Untraceable code.")
            {
                match message.parse::<u32>() {
                    Ok(limit) => {
                        if limit > 2 && limit < 81 {
                            settings.limit_of_messages = limit;
                        } else {
                            bot.send_message(
                                dialogue.chat_id(),
                                format!(
                                    "{}, {}",
                                    &settings.name.unwrap_or_else(|| "".to_string()),
                                    &settings.lang.settings_descriptions.limit
                                ),
                            )
                                .await?;
                            return Ok(());
                        }
                        settings
                    }
                    Err(_) => {
                        bot.send_message(
                            dialogue.chat_id(),
                            format!(
                                "{} {}, {} ",
                                settings.name.unwrap_or_else(|| "".to_string()),
                                Emojis::ShitHappens.random()?,
                                &settings.lang.fail_messages.need_number
                            ),
                        )
                            .await?;
                        return Ok(());
                    }
                }
            } else {
                unreachable!()
            };
            let keyboard = make_keyboard(&standard_keyboard_as_str_vec(&settings.lang));
            bot.send_message(dialogue.chat_id(), &settings.lang.send_commands)
                .reply_markup(keyboard)
                .await?;
            dialogue.update(State::CallBack(settings)).await?;

        }

        Ok(())
    }

    pub async fn find_by_name(
        message: Message,
        bot: AutoSend<Bot>,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        if let Some(message) = message.text() {
            let UserSettings { lang, .. } = CommandsHandler::get_settings(&dialogue).await?;
            if let Some(result) = DrinksService::get_drink_by_name(message, lang).await? {
                Self::send_vec_with_photo(&result, &bot, &dialogue).await?;
            } else {
                Self::send_wrong_message("Seems like we don't find anything", &bot, &dialogue)
                    .await?;
            }
        }
        Ok(())
    }
    /// todo: will be goood to send some tips
    pub async fn with_category(
        message: Message,
        bot: AutoSend<Bot>,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        if let Some(message) = message.text() {
            let user_setting = CommandsHandler::get_settings(&dialogue).await?;
            if let Some(result) = DrinksService::find_by_category(message, user_setting.lang.clone()).await? {
                Self::send_vec_with_photo(&result, &bot, &dialogue).await?;
            } else {
                Self::send_wrong_message("Seems like it was wrong category", &bot, &dialogue)
                    .await?;
            }
        }
        Ok(())
    }
    pub async fn with_ingredient(
        message: Message,
        bot: AutoSend<Bot>,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        if let Some(message) = message.text() {
            let user_settings = CommandsHandler::get_settings(&dialogue).await?;
            if let Some(result) = DrinksService::find_by_ingredient(message, user_settings.lang).await? {
                Self::send_vec_with_photo(&result, &bot, &dialogue).await?;
            } else {
                Self::send_wrong_message("Seems like it was wrong ingredient", &bot, &dialogue)
                    .await?;
            }
        }
        Ok(())
    }
    async fn send_wrong_message(
        message: &str,
        bot: &AutoSend<Bot>,
        dialogue: &LocalDialogue,
    ) -> ReturnTy {
        let settings = CommandsHandler::get_settings(dialogue).await?;
        bot.send_message(
            dialogue.chat_id(),
            format!(
                "{} {},{}, try again please.",
                settings.name.unwrap_or_else(|| "".to_string()),
                message,
                Emojis::ShitHappens.random()?
            ),
        )
            .await?;
        Ok(())
    }
    async fn send_vec_with_photo<T>(
        to_send: &Vec<T>,
        bot: &AutoSend<Bot>,
        dialogue: &LocalDialogue,
    ) -> ReturnTy
        where
            T: Display + WithPhoto,
    {
        let settings = CommandsHandler::get_settings(dialogue).await?;
        if settings.send_image {
            let (start_el, final_el) =
                Self::get_range(to_send.len(), settings.limit_of_messages as usize);
            for result in to_send[start_el..final_el].iter() {
                CallBackHandler::send_message(&result.to_string(), bot, dialogue).await?;
                if let Some(image) = result.get_url() {
                    bot.send_photo(dialogue.chat_id(), InputFile::url(Url::parse(&image)?))
                        .await?;
                }
            }
        } else {
            let result = vec_to_string(to_send, "\n_________________________\n");
            CallBackHandler::send_message(&result, bot, dialogue).await?;
        }
        CommandsHandler::start_commands(bot, dialogue).await?;
        Ok(())
    }
    fn get_range(vec_len: usize, settings_params: usize) -> (usize, usize) {
        match vec_len > settings_params {
            false => (0_usize, vec_len - 1),
            _ => {
                let random_start = random_num_in_range(0, vec_len - settings_params as usize);
                (random_start, random_start + settings_params)
            }
        }
    }

    pub async fn find_ingredient_by_name(
        message: Message,
        bot: AutoSend<Bot>,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        if let Some(message) = message.text() {
            let user_settings = CommandsHandler::get_settings(&dialogue).await?;
            if let Some(result) = DrinksService::get_ingredient_by_name(message, user_settings.lang.clone()).await? {
                for result in result {
                    CallBackHandler::send_message(&result.to_string(), &bot, &dialogue).await?;
                }

                CommandsHandler::start_commands(&bot, &dialogue).await?;
            } else {
                Self::send_wrong_message("Seems like it was wrong ingredient", &bot, &dialogue)
                    .await?;
            }
        }
        Ok(())
    }
}
