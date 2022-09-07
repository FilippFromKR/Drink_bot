use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::{ErrorHandler, ErrorType};
use crate::telegramm::messages::message_handler::TELEGRAMM_CHAR_LIMIT;

pub fn random_num_in_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(min..max);
    uniform.sample(&mut rng)
}

pub fn random_english_character() -> Result<char, ErrorHandler> {
    char::from_u32(random_num_in_range(65, 90) as u32).ok_or(ErrorHandler {
        msg: "Fail to get random character..".to_string(),
        ty: ErrorType::Unexpected,
    })
}

pub fn vec_to_string<T: Display>(vec: &[T], join: &str) -> String {
    vec.iter()
        .map(|category| category.to_string())
        .collect::<Vec<String>>()
        .join(join)
}

/// TODO:fix bag with string separating
/// separates bytes on wrong way and panic
/// probable, need to change this 'message.split_at(message.len()  / 2)' for something else
/// '+10' make it works, however it's shoudn't work like this.
/// to find bag, delete '+10' and call:  find_cocktail->coffee->panic
pub fn split(message: &str) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let (first, second) = message.split_at((message.len() / 2) + 10);
    if first.len() >= TELEGRAMM_CHAR_LIMIT {
        let mut first = split(first);
        let mut second = split(second);

        vec.append(&mut first);
        vec.append(&mut second);
    } else {
        vec.push(first.to_owned());
        vec.push(second.to_owned());
    }
    vec
}

pub fn write_to_file(text: &str) -> Result<(), ErrorHandler> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./suggestion_bags/suggestion.txt")
        .map_err(|err| ErrorHandler
        { msg: err.to_string(), ty: ErrorType::File })?;

    writeln!(file, "{}\n_____________________________\n" ,text)
        .map_err(|err| ErrorHandler
        { msg: err.to_string(), ty: ErrorType::File })?;
    Ok(())
}
