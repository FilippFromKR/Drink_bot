use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::prelude::Dialogue;

use crate::error::error_handler::ErrorHandler;
use crate::telegramm::state::State;

pub mod runner;
pub mod state;
pub mod messages;
pub mod buttons;
pub mod commands;

type LocalDialogue = Dialogue<State, ErasedStorage<State>>;
type ReturnTy = Result<(), ErrorHandler>;