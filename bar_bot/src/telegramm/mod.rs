use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::prelude::Dialogue;

use crate::error::error_handler::ErrorHandler;
use crate::telegramm::state::State;

pub mod buttons;
pub mod commands;
pub mod messages;
pub mod settings;
pub mod state;

type LocalDialogue = Dialogue<State, ErasedStorage<State>>;
type ReturnTy = Result<(), ErrorHandler>;
