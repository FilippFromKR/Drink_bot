use std::fs::File;
use std::io::Read;

use teloxide::Bot;
use teloxide::prelude::*;

use crate::telegramm::commands::command::StartCommands;
use crate::telegramm::{LocalDialogue, ReturnTy};
use crate::telegramm::buttons::keyboard::{as_str_vec, Keyboard, make_keyboard};
use crate::telegramm::state::State;
use crate::utils::read_file::read_file_as_str;


pub struct CommandsHandler;

impl CommandsHandler {
    pub async fn start(cmd: StartCommands, bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        match cmd {
            StartCommands::Start => Self::start_commands(&bot, &dialogue).await?,
            StartCommands::Help => Self::help(bot, dialogue).await?,
            StartCommands::Finish => Self::finish(bot, dialogue).await?,
        }
        Ok(())
    }
    pub async fn start_commands(bot: &AutoSend<Bot>, dialogue: &LocalDialogue) -> ReturnTy {
        let keyboard = make_keyboard(as_str_vec());
        bot.send_message(dialogue.chat_id(), "Here we go:").reply_markup(keyboard).await?;
        dialogue.update(State::CallBack).await?;
        Ok(())
    }
    async fn help(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        let text = read_file_as_str(".messages/help.txt")?;
        let keyboard = make_keyboard(as_str_vec());
        bot.send_message(dialogue.chat_id(), text).reply_markup(keyboard).await?;
        dialogue.update(State::CallBack).await?;
        Ok(())
    }
    async fn finish(bot: AutoSend<Bot>, dialogue: LocalDialogue) -> ReturnTy {
        bot.send_message(dialogue.chat_id(), "See u soon, Bro. :).").await?;
        dialogue.exit().await?;
        Ok(())
    }
}