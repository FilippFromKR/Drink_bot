use macroses::as_array;
use teloxide::utils::command::BotCommands;

#[derive(as_array, Clone, BotCommands, Eq, PartialEq, Debug)]
#[command(
rename = "lowercase",
)]
pub enum StartCommands {
    Back,
}
