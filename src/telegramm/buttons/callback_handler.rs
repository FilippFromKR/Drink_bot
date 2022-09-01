use itertools::Itertools;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{AutoSend, CallbackQuery, Requester};
use teloxide::types::InputFile;
use teloxide::Bot;
use url::Url;

use crate::cocktails_api::schemas::drink::LangDrink;
use crate::cocktails_api::schemas::ToLangDrink;
use crate::cocktails_api::services::coctail_service::DrinksService;
use crate::localization::lang::Lang;
use crate::telegramm::buttons::keyboard::{make_keyboard, Keyboard};
use crate::telegramm::commands::func::CommandsHandler;
use crate::telegramm::messages::message_handler::TELEGRAMM_CHAR_LIMIT;
use crate::telegramm::settings::settings::{SettingsKeyboard, UserSettings};
use crate::telegramm::state::State;
use crate::telegramm::{LocalDialogue, ReturnTy};
use crate::utils::helpers;
use crate::utils::helpers::random_num_in_range;
use crate::utils::unicod::Emojis;
use crate::{ErrorHandler, ErrorType};

pub struct CallBackHandler;

impl CallBackHandler {
    pub async fn main_commands(
        bot: AutoSend<Bot>,
        callback: CallbackQuery,
        dialogue: LocalDialogue,
    ) -> ReturnTy {
        if let Some(response) = callback.data {
            let UserSettings { lang, .. } = CommandsHandler::get_settings(&dialogue).await?;
            match Keyboard::try_from(Self::to_button(&response, &lang))? {
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
    fn to_button(response: &str, lang: &Lang) -> String {
        let (key, _) = lang
            .buttons
            .main
            .iter()
            .find(|(_, value)| *value == response)
            .expect("Unreachable code.");
        key.clone()
    }
    pub async fn callback_settings(
        bot: AutoSend<Bot>,
        dialogue: LocalDialogue,
        callback: CallbackQuery,
    ) -> ReturnTy {
        if let Some(callback) = callback.data {
            let mut user_settings = CommandsHandler::get_settings(&dialogue).await?;
            let button_key = Self::to_setting_button(&callback, &user_settings.lang);
            match SettingsKeyboard::try_from(button_key.as_str())? {
                SettingsKeyboard::Back => {
                    return CommandsHandler::start_commands(&bot, &dialogue).await;
                }
                SettingsKeyboard::Images => {
                    user_settings.send_image = match &user_settings.send_image {
                        true => false,
                        false => true,
                    };
                    dialogue.update(State::Settings(user_settings.clone())).await?;
                    Self::send_setting_message(&bot, &dialogue, &user_settings.lang).await?;
                }
                SettingsKeyboard::MessageLimit => {
                    bot.send_message(
                        dialogue.chat_id(),
                        format!(" -{} \n", &user_settings.lang.todo.settings_set_limit),
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
                    bot.send_message(
                        dialogue.chat_id(),
                        &format!(" -{} \n", &user_settings.lang.todo.settings_set_name),
                    )
                    .await?;
                    dialogue
                        .update(State::SettingsUpdate(
                            CommandsHandler::get_settings(&dialogue).await?,
                            SettingsKeyboard::Name,
                        ))
                        .await?;
                }
                SettingsKeyboard::Lang => {
                    user_settings.lang = match &user_settings.lang {
                        Lang::Eng => Lang::Ukr,
                        Lang::Ukr => Lang::Eng,
                    };
                    dialogue
                        .update(State::Settings(user_settings.clone()))
                        .await?;
                    Self::send_setting_message(&bot, &dialogue, &user_settings.lang).await?;
                }
            };
        }
        Ok(())
    }
    fn to_setting_button(response: &str, lang: &Lang) -> String {
        let (key, _) = lang
            .buttons
            .settings
            .iter()
            .find(|(_, value)| *value == response)
            .expect("Unreachable code.");
        key.clone()
    }

    async fn settings(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let UserSettings { lang, .. } = CommandsHandler::get_settings(&dialogue).await?;
        Self::send_setting_message(&bot, &dialogue, &lang).await?;
        dialogue
            .update(State::Settings(
                CommandsHandler::get_settings(&dialogue).await?,
            ))
            .await?;
        Ok(())
    }

    async fn send_setting_message(
        bot: &AutoSend<Bot>,
        dialogue: &LocalDialogue,
        lang: &Lang,
    ) -> ReturnTy {
        let settings = CommandsHandler::get_settings(dialogue).await?;
        let keyboard = SettingsKeyboard::as_array()
            .iter()
            .map(|key| {
                lang.buttons
                    .settings
                    .get(key.as_str())
                    .expect("Unexpected code.")
                    .as_str()
            })
            .collect::<Vec<&str>>();
        let keyboard = make_keyboard(&keyboard);
        bot.send_message(dialogue.chat_id(), settings.to_string())
            .reply_markup(keyboard)
            .await?;
        Ok(())
    }

    async fn cocktail_for_you(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let random_char_of_eng_alphabet = helpers::random_english_character()?;
        let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
        if let Some(result) = DrinksService::search_by_first_letter(
            &random_char_of_eng_alphabet,
            user_settings.lang.clone(),
        )
        .await?

        { let result =  Self::make_less(result);
            let alcohol = Self::ingredients_as_str_vec(&result);
            let keyboard = make_keyboard(&alcohol[0..2]);
            bot.send_message(dialogue.chat_id(), &user_settings.lang.todo.game_choose)
                .reply_markup(keyboard)
                .await?;

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
    fn make_less(vec:Vec<LangDrink>)->Vec<LangDrink>{
        vec.into_iter()
            .enumerate()
            .filter_map(|(num,drink)|  match num%2 == 0 {
                true => Some(drink),
                _ => None,
            } )
            .collect::<Vec<LangDrink>>()

    }

    fn ingredients_as_str_vec(raw_drink: &[LangDrink]) -> Vec<&str> {
        raw_drink
            .iter()
            .filter_map(|drink| match &drink.get_drink().ingredients.get(0) {
                None => None,
                Some(_) => {
                    let random_ingredient =
                        random_num_in_range(0, drink.get_drink().ingredients.len());
                    Some(
                        drink
                            .get_drink()
                            .ingredients
                            .get(random_ingredient)
                            .expect("Unreachable code.")
                            .0
                            .as_ref(),
                    )
                }
            })
            .unique()
            .collect::<Vec<&str>>()
    }

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
            } = dialogue.get().await?.ok_or(ErrorHandler {
                msg: "Absents of State in dialogue.".to_string(),
                ty: ErrorType::Unexpected,
            })? {
                let callback = if callback == first { second } else { first };
                Self::filter(&callback, &mut all);
                let drink_str = Self::ingredients_as_str_vec(&all);

                if drink_str.len() >= 2 {
                    let keyboard = make_keyboard(&drink_str[0..2]);
                    bot.send_message(dialogue.chat_id(), &settings.lang.todo.game_choose)
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
                    let drink = all.get(0).ok_or(ErrorHandler {
                        msg: "Exception in Game algorithm.".to_string(),
                        ty: ErrorType::Unexpected,
                    })?;
                    bot.send_message(dialogue.chat_id(), &drink.to_string())
                        .await?;
                    if settings.send_image {
                        if let Some(url) = &drink.get_drink().image {
                            bot.send_photo(dialogue.chat_id(), InputFile::url(Url::parse(url)?))
                                .await?;
                        }
                    }
                    CommandsHandler::start_commands(&bot, &dialogue).await?;
                }
            }
        }

        Ok(())
    }

    fn filter(filter: &str, vec: &mut Vec<LangDrink>) {
        vec.retain(|drink| {
            !drink
                .get_drink()
                .ingredients
                .iter()
                .any(|(ingr, _)| ingr == filter)
        });
    }

    async fn with_this_category(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
        let message = format!(
            "{}{}, \n{} ",
            &user_settings.name.as_ref().unwrap_or(&"".to_string()),
            Emojis::Smile.random()?,
            &user_settings.lang.todo.category_write,
        );
        dialogue.update(State::WithCategory(user_settings)).await?;
        bot.send_message(dialogue.chat_id(), message).await?;
        Ok(())
    }

    async fn with_this_ingredient(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
        let message = format!(
            "{}{}. \n{}",
            &user_settings.name.as_ref().unwrap_or(&"".to_string()),
            Emojis::Smile.random()?,
            &user_settings.lang.todo.ingredient_write,
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
            " {}{} \n{}",
            &user_settings.name.as_ref().unwrap_or(&"".to_string()),
            Emojis::Smile.random()?,
            &user_settings.lang.todo.find_cocktails_write
        );
        dialogue.update(State::FindByName(user_settings)).await?;
        bot.send_message(dialogue.chat_id(), message).await?;
        Ok(())
    }

    async fn find_ingredient(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let user_settings: UserSettings = CommandsHandler::get_settings(&dialogue).await?;
        let message = format!(
            "{}{} \n{} ",
            &user_settings.name.as_ref().unwrap_or(&"".to_string()),
            Emojis::Smile.random()?,
            &user_settings.lang.todo.find_ingredient_write,
        );
        dialogue
            .update(State::FindIngrByName(user_settings))
            .await?;
        bot.send_message(dialogue.chat_id(), message).await?;
        Ok(())
    }

    async fn all_ingredients(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let UserSettings { lang, .. } = CommandsHandler::get_settings(&dialogue).await?;
        let result = helpers::vec_to_string(
            DrinksService::get_all_ingredients(lang).await?.as_slice(),
            "\n_______________________\n",
        );
        Self::send_message(&result, &bot, &dialogue).await?;
        CommandsHandler::start_commands(&bot, &dialogue).await?;
        Ok(())
    }

    async fn all_category(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let UserSettings { lang, .. } = CommandsHandler::get_settings(&dialogue).await?;
        let result = helpers::vec_to_string(
            &DrinksService::get_all_category(lang).await?,
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
