use std::fmt::{Debug, Display};

use teloxide::Bot;
use teloxide::prelude::{AutoSend, CallbackQuery, Requester};
use crate::telegramm::commands::func::CommandsHandler;
use crate::coctails_api::services::coctail_service::DrinksService;
use crate::telegramm::{LocalDialogue, ReturnTy};
use crate::telegramm::buttons::keyboard::Keyboard;
use crate::telegramm::messages::message_handler::TELEGRAMM_CHAR_LIMIT;
use crate::telegramm::state::State;
use crate::utils::str_builder;
use crate::utils::unicod::Emojis;

pub struct CallBackHandler;

impl CallBackHandler {
    pub async fn main_commands(bot: AutoSend<Bot>, callback: CallbackQuery, dialogue: LocalDialogue) -> ReturnTy {
        if let Some(response) = callback.data {
            match Keyboard::try_from(response)? {
                Keyboard::FindCocktail => Self::find_by_name(bot, dialogue).await?,
                Keyboard::FindIngredient => Self::find_ingredient(bot, dialogue).await?,
                Keyboard::Ingredients => Self::all_ingredients(bot, dialogue).await?,
                Keyboard::Categories => Self::all_category(bot, dialogue).await?,
                Keyboard::WithThisIngredient => Self::with_this_ingredient(bot, dialogue).await?,
                Keyboard::WithThisCategory => Self::with_this_category(bot,dialogue).await?,

            }
        }
        Ok(())
    }

    async fn with_this_category(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        dialogue.update(State::WithCategory).await?;
        bot.send_message(dialogue.chat_id(), format!("Write one of existing category, Bro{}", Emojis::Smile.get_randoms(1)?.get(0).unwrap_or(&&' '))).await?;
        Ok(())


    }
    async fn with_this_ingredient(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        dialogue.update(State::WithIngredient).await?;
        bot.send_message(dialogue.chat_id(), format!("Write one of existing ingredient, Bro{}", Emojis::Smile.get_randoms(1)?.get(0).unwrap_or(&&' '))).await?;
        Ok(())


    }
    async fn find_by_name(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        dialogue.update(State::FindByName).await?;
        bot.send_message(dialogue.chat_id(), format!("What kind of cocktail do you want to find, bro? {}", Emojis::Drink.get_randoms(1)?.get(0).unwrap_or(&&' '))).await?;
        Ok(())
    }
    async fn find_ingredient(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        dialogue.update(State::FindIngrByName).await?;
        bot.send_message(dialogue.chat_id(), format!("What kind of Ingredient do you want to find, bro? {}", Emojis::Smile.get_randoms(1)?.get(0).unwrap_or(&&' '))).await?;
        Ok(())
    }
    async fn all_ingredients(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let result = str_builder::vec_to_string(DrinksService::get_all_ingredients().await?,
                                         "\n_______________________\n");
        Self::send_message(&result, &bot, &dialogue).await?;
        CommandsHandler::start_commands(&bot,&dialogue).await?;
        Ok(())
    }
    async fn all_category(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let result = str_builder::vec_to_string(DrinksService::get_all_category().await?,
                                         "\n_______________________\n");

        Self::send_message(&result, &bot, &dialogue).await?;
        CommandsHandler::start_commands(&bot,&dialogue).await?;
        Ok(())
    }

   pub async fn send_message(message: &str, bot: &AutoSend<Bot>, dialogue: &LocalDialogue) -> ReturnTy {
        if message.len() >= TELEGRAMM_CHAR_LIMIT {
            for message in str_builder::split(message) {
                bot.send_message(dialogue.chat_id(), message).await?;
            }
        } else {
            bot.send_message(dialogue.chat_id(), message).await?;
        }
        Ok(())
    }
}