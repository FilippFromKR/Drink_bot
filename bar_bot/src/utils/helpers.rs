use crate::telegramm::messages::message_handler::TELEGRAMM_CHAR_LIMIT;
use crate::{ErrorHandler, ErrorType};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use std::fmt::Display;

pub fn random_num_in_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(min..max);
    uniform.sample(&mut rng)
}
pub fn random_english_character() -> Result<char, ErrorHandler> {
    char::from_u32(random_num_in_range(65, 90) as u32).ok_or(ErrorHandler {
        msg: "Exception in Game algorithm.".to_string(),
        ty: ErrorType::Unexpected,
    })
}

pub fn vec_to_string<T: Display>(vec: &[T], join: &str) -> String {
    vec.iter()
        .map(|category| category.to_string())
        .collect::<Vec<String>>()
        .join(join)
}

pub fn split(message: &str) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let (first, second) = message.split_at(message.len() / 2);
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
