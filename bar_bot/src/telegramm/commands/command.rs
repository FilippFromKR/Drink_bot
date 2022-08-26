use macroses::as_array;
use teloxide::utils::command::BotCommands;

#[derive(as_array, Clone, BotCommands, Eq, PartialEq, Debug)]
#[command(description = "Hello, my friend! For now you can use next commands: ")]
pub enum StartCommands {
    Start,
    Help,
}
