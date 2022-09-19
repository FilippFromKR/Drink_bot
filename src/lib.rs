use crate::config::Env;
use crate::error::error_handler::{ErrorHandler, ErrorType};
use crate::telegramm::buttons::callback_handler::CallBackHandler;
use crate::telegramm::commands::command::StartCommands;
use crate::telegramm::commands::func::CommandsHandler;
use crate::telegramm::messages::message_handler::MessageHandler;
use crate::telegramm::state::State;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::dispatching::dialogue::SqliteStorage;
use teloxide::dispatching::dialogue::Storage;
use teloxide::dispatching::{dialogue, Dispatcher, UpdateFilterExt, UpdateHandler};
use teloxide::dptree::case;
use teloxide::prelude::{RequesterExt, Update};
use teloxide::{dptree, Bot};

mod cocktails_api;
pub mod config;
mod error;
mod localization;
mod telegramm;
mod utils;


pub struct TelegrammBuilder;

impl TelegrammBuilder {
    pub fn run(env: Env) {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(env.workers_number)
            .max_blocking_threads(env.blocking_treads)
            .enable_all()
            .build()
            .unwrap();

        let handle = rt.handle().clone();
        let (sx, rx) = tokio::sync::oneshot::channel();

        rt.block_on(async move {
            if let Err(error) = Self::build(env).await {
                if error.is_critical() {
                    log::error!("Service critical error! Error msg: {:?}", error);
                    sx.send(0).expect("Sender failed...");
                } else {
                    log::error!("Error msg: {:?}", error);
                }
            }
        });

        handle.spawn(async move {
            // error listener

            let result = rx.await.unwrap();

            if result == 0 {
                log::info!("Service shutdown...");
                rt.shutdown_background();
            }
        });
    }

    async fn build(env: Env) -> Result<(), ErrorHandler> {
        pretty_env_logger::init();
        log::info!("Waking up with variables {:?}...", &env);

        let storage: std::sync::Arc<ErasedStorage<State>> = SqliteStorage::open(&env.db_path, Json)
            .await
            .map_err(|error| ErrorHandler {
                msg: error.to_string(),
                ty: ErrorType::Database,
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
            .branch(dptree::entry().endpoint(CommandsHandler::handle_commands));

        let message_handler = Update::filter_message()
            .branch(commands_handler)
            .branch(case![State::FindByName(setting)].endpoint(MessageHandler::find_by_name))
            .branch(
                case![State::FindIngrByName(setting)]
                    .endpoint(MessageHandler::find_ingredient_by_name),
            )
            .branch(case![State::WithIngredient(setting)].endpoint(MessageHandler::with_ingredient))
            .branch(case![State::WithCategory(setting)].endpoint(MessageHandler::with_category))
            .branch(
                case![State::SettingsUpdate(settings, params)].endpoint(MessageHandler::settings),
            )
            .branch(case![State::Suggestion(settings)].endpoint(MessageHandler::suggestion))
            .branch(dptree::entry().endpoint(MessageHandler::unexpected_message));

        let callback_handler = Update::filter_callback_query()
            .branch(case![State::CallBack(settings)].endpoint(CallBackHandler::main_commands))
            .branch(
                case![State::CocktailForYou {
                    all,
                    game,
                    settings
                }]
                .endpoint(CallBackHandler::game),
            )
            .branch(case![State::Settings(settings)].endpoint(CallBackHandler::callback_settings));

        dialogue::enter::<Update, ErasedStorage<State>, State, _>()
            .branch(message_handler)
            .branch(callback_handler)
    }
}
