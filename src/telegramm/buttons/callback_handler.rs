use itertools::Itertools;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{AutoSend, CallbackQuery, Requester};
use teloxide::types::InputFile;
use teloxide::Bot;
use url::Url;

use crate::coctails_api::schemas::drink::Drink;
use crate::coctails_api::services::coctail_service::DrinksService;
use crate::telegramm::buttons::keyboard::{make_keyboard, Keyboard};
use crate::telegramm::commands::func::CommandsHandler;
use crate::telegramm::messages::message_handler::TELEGRAMM_CHAR_LIMIT;
use crate::telegramm::settings::settings::{SettingsKeyboard, UserSettings};
use crate::telegramm::state::State;
use crate::telegramm::{LocalDialogue, ReturnTy};
use crate::utils::helpers;
use crate::utils::helpers::random_num_in_range;
use crate::utils::unicod::Emojis;

pub struct CallBackHandler;

impl CallBackHandler {
    pub async fn main_commands(
        bot: AutoSend<Bot>,
        callback: CallbackQuery,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        if let Some(response) = callback.data {
            match Keyboard::try_from(response)? {
                Keyboard::FindCocktail => Self::find_by_name(bot, dialogue).await?,
                Keyboard::FindIngredient => Self::find_ingredient(bot, dialogue).await?,
                Keyboard::Ingredients => Self::all_ingredients(bot, dialogue).await?,
                Keyboard::Categories => Self::all_category(bot, dialogue).await?,
                Keyboard::WithThisIngredient => Self::with_this_ingredient(bot, dialogue).await?,
                Keyboard::WithThisCategory => Self::with_this_category(bot, dialogue).await?,
                Keyboard::DrinkForYou => Self::cocktail_for_you(bot, dialogue).await?,
                Keyboard::Settings => Self::settings(bot, dialogue).await?,
            }
        }
        Ok(())
    }
    pub async fn callback_settings(
        bot: AutoSend<Bot>,
        dialogue: LocalDialogue,
        callback: CallbackQuery,
    ) -> ReturnTy {
        if let Some(callback) = callback.data {
            match SettingsKeyboard::try_from(callback.as_str())? {
                SettingsKeyboard::Back => {
                    return CommandsHandler::start_commands(&bot, &dialogue).await
                }
                SettingsKeyboard::Images => {
                    let mut user_settings = CommandsHandler::get_settings(&dialogue).await?;
                    user_settings.send_image = match &user_settings.send_image {
                        true => false,
                        false => true,
                    };
                    dialogue.update(State::Settings(user_settings)).await?;
                    Self::send_setting_message(&bot, &dialogue).await?;
                }
                SettingsKeyboard::MessageLimit => {
                    bot.send_message(
                        dialogue.chat_id(),
                        "Please enter the message limit (min - 3, max - 80 ).",
                    )
                    .await?;
                    dialogue
                        .update(State::SettingsUpdate(
                            CommandsHandler::get_settings(&dialogue).await?,
                            SettingsKeyboard::MessageLimit,
                        ))
                        .await?;
                }
                SettingsKeyboard::Name => {
                    bot.send_message(dialogue.chat_id(), "Please enter your name.")
                        .await?;
                    dialogue
                        .update(State::SettingsUpdate(
                            CommandsHandler::get_settings(&dialogue).await?,
                            SettingsKeyboard::Name,
                        ))
                        .await?;
                }
            };
        }
        Ok(())
    }
    async fn settings(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        Self::send_setting_message(&bot, &dialogue).await?;
        dialogue
            .update(State::Settings(
                CommandsHandler::get_settings(&dialogue).await?,
            ))
            .await?;
        Ok(())
    }
    async fn send_setting_message(bot: &AutoSend<Bot>, dialogue: &LocalDialogue) -> ReturnTy {
        let settings = CommandsHandler::get_settings(dialogue).await?;
        let keyboard = SettingsKeyboard::as_array()
            .iter()
            .map(|key| key.as_str())
            .collect::<Vec<&str>>();
        let keyboard = make_keyboard(&keyboard);
        bot.send_message(dialogue.chat_id(), settings.to_string())
            .reply_markup(keyboard)
            .await?;
        Ok(())
    }
    async fn cocktail_for_you(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let random_char_of_eng_alphabet = helpers::random_english_character();
        if let Some(result) =
            DrinksService::search_by_first_letter(&random_char_of_eng_alphabet).await?
        {
            let alcohol = Self::ingredients_as_str_vec(&result);
            let keyboard = make_keyboard(&alcohol[0..2].to_vec());
            bot.send_message(dialogue.chat_id(), "Choose one. ")
                .reply_markup(keyboard)
                .await?;
            let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
            dialogue
                .update(State::CocktailForYou {
                    game: (alcohol[0].to_string(), alcohol[1].to_string()),
                    all: result,
                    settings: user_settings,
                })
                .await?;
        }
        Ok(())
    }
    fn ingredients_as_str_vec(raw_drink: &[Drink]) -> Vec<&str> {
        raw_drink
            .iter()
            .filter(|drink| drink.ingredients.get(0).is_some())
            .map(|drink| {
                let random_ingredient = random_num_in_range(0, drink.ingredients.len());
                drink
                    .ingredients
                    .get(random_ingredient)
                    .expect("Unreachable code.")
                    .0
                    .as_str()
            })
            .unique()
            .collect::<Vec<&str>>()
    }

    /// todo: use not first, but random ingredient every time
    pub async fn game(
        bot: AutoSend<Bot>,
        callback: CallbackQuery,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        if let Some(callback) = callback.data {
            if let State::CocktailForYou {
                mut all,
                game: (first, second),
                settings,
            } = dialogue.get().await?.expect("Unreachable code.")
            {
                let callback = if callback == first { second } else { first };
                Self::filter(&callback, &mut all);
                let drink_str = Self::ingredients_as_str_vec(&all);

                if drink_str.len() >= 2 {
                    let keyboard = make_keyboard(&drink_str[0..2].to_vec());
                    bot.send_message(dialogue.chat_id(), "Choose one. ")
                        .reply_markup(keyboard)
                        .await?;
                    dialogue
                        .update(State::CocktailForYou {
                            game: (drink_str[0].to_string(), drink_str[1].to_string()),
                            all,
                            settings,
                        })
                        .await?;
                } else {
                    let drink = all.get(0).expect("Unreachable code.");
                    bot.send_message(dialogue.chat_id(), &drink.to_string())
                        .await?;
                    if settings.send_image {
                        if let Some(url) = &drink.image {
                            bot.send_photo(dialogue.chat_id(), InputFile::url(Url::parse(url)?))
                                .await?;
                            CommandsHandler::start_commands(&bot, &dialogue).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
    fn filter(filter: &str, vec: &mut Vec<Drink>) {
        vec.retain(|drink| !drink.ingredients.iter().any(|(ingr, _)| ingr == filter));
    }

    async fn with_this_category(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
        let message = format!(
            "Write one of existing category, {}{}",
            &user_settings.name.as_ref().unwrap_or(&"".to_string()),
            Emojis::Smile.random()?
        );
        dialogue.update(State::WithCategory(user_settings)).await?;
        bot.send_message(dialogue.chat_id(), message).await?;
        Ok(())
    }
    async fn with_this_ingredient(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
        let message = format!(
            "Write one of existing ingredient, {}{}",
            &user_settings.name.as_ref().unwrap_or(&"".to_string()),
            Emojis::Smile.random()?
        );
        dialogue
            .update(State::WithIngredient(user_settings))
            .await?;
        bot.send_message(dialogue.chat_id(), message).await?;
        Ok(())
    }
    async fn find_by_name(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
        let message = format!(
            "What kind of cocktail do you want to find, {}{}",
            &user_settings.name.as_ref().unwrap_or(&"".to_string()),
            Emojis::Smile.random()?
        );
        dialogue.update(State::FindByName(user_settings)).await?;
        bot.send_message(dialogue.chat_id(), message).await?;
        Ok(())
    }
    async fn find_ingredient(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
        let message = format!(
            "What kind of Ingredient do you want to find, {}{}",
            &user_settings.name.as_ref().unwrap_or(&"".to_string()),
            Emojis::Smile.random()?
        );
        dialogue
            .update(State::FindIngrByName(user_settings))
            .await?;
        bot.send_message(dialogue.chat_id(), message).await?;
        Ok(())
    }
    async fn all_ingredients(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let result = helpers::vec_to_string(
            &DrinksService::get_all_ingredients().await?,
            "\n_______________________\n",
        );
        Self::send_message(&result, &bot, &dialogue).await?;
        CommandsHandler::start_commands(&bot, &dialogue).await?;
        Ok(())
    }
    async fn all_category(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let result = helpers::vec_to_string(
            &DrinksService::get_all_category().await?,
            "\n_______________________\n",
        );

        Self::send_message(&result, &bot, &dialogue).await?;
        CommandsHandler::start_commands(&bot, &dialogue).await?;
        Ok(())
    }

    pub async fn send_message(
        message: &str,
        bot: &AutoSend<Bot>,
        dialogue: &LocalDialogue,
    ) -> ReturnTy {
        if message.len() >= TELEGRAMM_CHAR_LIMIT {
            for message in helpers::split(message) {
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
