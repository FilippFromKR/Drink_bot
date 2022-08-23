use std::fmt::Display;

use teloxide::Bot;
use teloxide::prelude::{AutoSend, Message, Requester};
use teloxide::types::InputFile;
use url::Url;
use crate::coctails_api::services::coctail_service::DrinksService;
use crate::telegramm::commands::func::CommandsHandler;
use crate::telegramm::{LocalDialogue, ReturnTy};
use crate::telegramm::buttons::callback_handler::CallBackHandler;
use crate::telegramm::state::State;
use crate::utils::str_builder;
use crate::utils::str_builder::vec_to_string;
use crate::utils::unicod::Emojis;

pub const TELEGRAMM_CHAR_LIMIT: usize = 4096;

pub struct MessageHandler;

impl MessageHandler {
    /// todo: Add settings to pass images or save install limit.
    /// todo:add list fun answers on this
    pub async fn unexpected_message(message: Message, bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        bot.send_message(dialogue.chat_id(), format!("{} Unexpected message: {} ", Emojis::ShitHappens.get_randoms(1).expect("Unexpected behavior.").get(0).unwrap_or(&&' '), message.text().unwrap_or(""))).await?;
        CommandsHandler::start_commands(&bot,&dialogue).await?;
        Ok(())
    }
    pub async fn find_by_name(message: Message, bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        if let Some(message) = message.text() {
            if let Some(result) = DrinksService::get_drink_by_name(message).await? {
                for result in result {
                    CallBackHandler::send_message(&result.to_string(), &bot, &dialogue).await?;
                    if let Some(ref image) = result.image {
                        bot.send_photo(dialogue.chat_id(), InputFile::url(Url::parse(image)?)).await?;
                    }
                }
                CommandsHandler::start_commands(&bot,&dialogue).await?;
            } else {
                bot.send_message(dialogue.chat_id(), format!("{} Seems like we don't find anything, try again please.",
                                                             Emojis::ShitHappens.random()?)).await?;
            }
        }
        Ok(())
    }
    pub async fn with_category(message: Message, bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        if let Some(message) = message.text() {
            if let Some(result) = DrinksService::find_by_category(message).await? {
                for result in result {
                    CallBackHandler::send_message(&result.name, &bot, &dialogue).await?;
                    bot.send_photo(dialogue.chat_id(),InputFile::url(Url::parse(&result.image_url)?)).await?;
                }
                CommandsHandler::start_commands(&bot,&dialogue).await?;
            } else {
                bot.send_message(dialogue.chat_id(),
                                 format!("{} Seems like it was wrong category, try again please.",
                                         Emojis::ShitHappens.random()?)).await?;
            }
        }
        Ok(())
    }
    pub async fn with_ingredient(message: Message, bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        if let Some(message) = message.text() {
            if let Some(result) = DrinksService::find_by_ingredient(message).await? {
                for result in result {
                    bot.send_message(dialogue.chat_id(),result.to_string()).await?;
                    bot.send_photo(dialogue.chat_id(), InputFile::url(Url::parse(&result.image_url)?)).await?;
                }
                CommandsHandler::start_commands(&bot,&dialogue).await?;
            } else {
                bot.send_message(dialogue.chat_id(),
                                 format!("{} Seems like it was wrong ingredient, try again please.",
                                         Emojis::ShitHappens.random()?)).await?;
            }
        }
        Ok(())
    }

    pub async fn find_ingredient_by_name(message: Message, bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        if let Some(message) = message.text() {
            if let Some(result) = DrinksService::get_ingredient_by_name(message).await? {
                for result in result {
                    CallBackHandler::send_message(&result.to_string(), &bot, &dialogue).await?;
                }
                CommandsHandler::start_commands(&bot,&dialogue).await?;
            }
            else {
                bot.send_message(dialogue.chat_id(),format!("{} Seems like it was wrong ingredient, try again please.",
                                                            Emojis::ShitHappens.random()?)).await?;
            }
        }
        Ok(())
    }
}