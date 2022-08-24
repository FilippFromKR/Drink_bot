use std::fmt::{Debug, Display};

use itertools::Itertools;
use log::log;
use teloxide::Bot;
use teloxide::payloads::SendDocumentSetters;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{AutoSend, CallbackQuery, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use url::Url;

use crate::coctails_api::schemas::drink::Drink;
use crate::coctails_api::services::coctail_service::DrinksService;
use crate::telegramm::{LocalDialogue, ReturnTy};
use crate::telegramm::buttons::keyboard::{Keyboard, make_keyboard};
use crate::telegramm::commands::func::CommandsHandler;
use crate::telegramm::messages::message_handler::TELEGRAMM_CHAR_LIMIT;
use crate::telegramm::state::State;
use crate::utils::str_builder;
use crate::utils::unicod::Emojis;

pub struct CallBackHandler;

/// todo:add to Keyboard instanse of fn(bot, dialogue)-> returnTy to invoke it in every keyboard.
impl CallBackHandler {
    pub async fn main_commands(bot: AutoSend<Bot>, callback: CallbackQuery, dialogue: LocalDialogue) -> ReturnTy {
        if let Some(response) = callback.data {
            match Keyboard::try_from(response)? {
                Keyboard::FindCocktail => Self::find_by_name(bot, dialogue).await?,
                Keyboard::FindIngredient => Self::find_ingredient(bot, dialogue).await?,
                Keyboard::Ingredients => Self::all_ingredients(bot, dialogue).await?,
                Keyboard::Categories => Self::all_category(bot, dialogue).await?,
                Keyboard::WithThisIngredient => Self::with_this_ingredient(bot, dialogue).await?,
                Keyboard::WithThisCategory => Self::with_this_category(bot, dialogue).await?,
                Keyboard::CocktailForYou => Self::cocktail_for_you(bot, dialogue).await?,
            }
        }
        Ok(())
    }
    async fn cocktail_for_you(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let random_char_of_eng_alphabet = char::from_u32(Emojis::random_num_in_range(65, 90) as u32).expect("Unreachable code.");
        if let Some(result) = DrinksService::search_by_first_letter(&random_char_of_eng_alphabet).await? {
            let alcohol = Self::ingredients_as_str_vec(&result);
            let keyboard = make_keyboard(alcohol[0..2].to_vec());
            bot.send_message(dialogue.chat_id(), "Choose one. ").reply_markup(keyboard).await?;
            dialogue.update(State::CocktailForYou2 { game: (alcohol[0].to_string(), alcohol[1].to_string()), all: result }).await?;
        }
        Ok(())
    }
    fn ingredients_as_str_vec(raw_drink: &Vec<Drink>) -> Vec<&str> {
        raw_drink
            .iter()
            .filter(|drink| drink.ingredients.get(0).is_some())
            .map(|drink| drink.ingredients.get(0).expect("Unreachable code.").0.as_str())
            .unique()
            .collect::<Vec<&str>>()
    }
    pub async fn game(bot: AutoSend<Bot>, callback: CallbackQuery, dialogue: LocalDialogue) -> ReturnTy {
        if let Some(callback) = callback.data {
            if let State::CocktailForYou2 { mut all, game: (first, second) } = dialogue.get().await?.expect("Unreachable code.") {
                let callback = if callback == first { second } else { first };
                Self::filter(&callback, &mut all);
                let drink_str = Self::ingredients_as_str_vec(&all);

                if drink_str.len() >= 2 {
                    let keyboard = make_keyboard(drink_str[0..2].to_vec());
                    bot.send_message(dialogue.chat_id(), "Choose one. ").reply_markup(keyboard).await?;
                    dialogue.update(State::CocktailForYou2 { game: (drink_str[0].to_string(), drink_str[1].to_string()), all }).await?;
                }
                else {
                    let drink = all.iter().next().expect("Unreachable code.");
                    bot.send_message(dialogue.chat_id(), &drink.to_string()).await?;
                    if let Some(url) = &drink.image {
                        bot.send_photo(dialogue.chat_id(), InputFile::url(Url::parse(url)?)).await?;
                        CommandsHandler::start_commands(&bot, &dialogue).await?;
                    }
                }
            }
        }

        Ok(())
    }
    fn filter(filter: &str, vec: &mut Vec<Drink>) {
        vec.retain(|drink| drink.ingredients[0].0 != filter);
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
        CommandsHandler::start_commands(&bot, &dialogue).await?;
        Ok(())
    }
    async fn all_category(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let result = str_builder::vec_to_string(DrinksService::get_all_category().await?,
                                                "\n_______________________\n");

        Self::send_message(&result, &bot, &dialogue).await?;
        CommandsHandler::start_commands(&bot, &dialogue).await?;
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

#[cfg(test)]
mod test {
    #[test]
    fn test_char() {
        let char = char::from_u32(90 - 25).unwrap();
        println!("output {}", char);
    }
}
