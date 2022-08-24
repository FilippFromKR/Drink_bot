use std::sync::Arc;

use teloxide::{Bot, dptree};
use teloxide::dispatching::{dialogue, Dispatcher, HandlerExt, UpdateFilterExt, UpdateHandler};
use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::SqliteStorage;
use teloxide::dispatching::dialogue::Storage;
use teloxide::dptree::case;
use teloxide::prelude::{RequesterExt, Update};
use teloxide::types::Message;
use crate::config::Env;
use crate::error::error_handler::{ErrorHandler, ErrorType};
use crate::telegramm::buttons::callback_handler::CallBackHandler;
use crate::telegramm::commands::command::StartCommands;
use crate::telegramm::commands::func::CommandsHandler;
use crate::telegramm::messages::message_handler::MessageHandler;
use crate::telegramm::state::State;

pub mod config;
mod coctails_api;
mod error;
mod telegramm;
mod utils;

pub struct TelegrammBuilder;

impl TelegrammBuilder {
    pub async fn run(env: Env) -> Result<(), ErrorHandler> {
        pretty_env_logger::init();
        log::info!("Waking up with variables {:?}...", &env);

        let storage: std::sync::Arc<ErasedStorage<State>> = SqliteStorage::open("db.sqlite", Json)
            .await
            .map_err(|error| ErrorHandler {
                msg: error.to_string(),
                ty: ErrorType::DATABASE,
            })?
            .erase();

        let handler = Self::create_handler();


        let bot = Bot::new(&env.bot_id).auto_send();

        Dispatcher::builder(bot, handler)
            .dependencies(dptree::deps![storage])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;

        Ok(())
    }
    fn create_handler() -> UpdateHandler<ErrorHandler> {
        let commands_handler = teloxide::filter_command::<StartCommands, _>()
            .branch(case![State::Start].endpoint(CommandsHandler::start));

        let message_handler = Update::filter_message()
            .branch(commands_handler)
            .branch(case![State::FindByName].endpoint(MessageHandler::find_by_name))
            .branch(case![State::FindIngrByName].endpoint(MessageHandler::find_ingredient_by_name))
            .branch(case![State::WithIngredient].endpoint(MessageHandler::with_ingredient))
            .branch(case![State::WithCategory].endpoint(MessageHandler::with_category))
            .branch(dptree::entry().endpoint(MessageHandler::unexpected_message));

        let callback_handler = Update::filter_callback_query()
            .branch(case![State::CallBack].endpoint(CallBackHandler::main_commands))
            .branch(case![State::CocktailForYou2 {all, game}].endpoint(CallBackHandler::game));

        dialogue::enter::<Update, ErasedStorage<State>, State, _>()
            .branch(message_handler)
            .branch(callback_handler)
    }
}