use std::fmt::Display;

use crate::telegramm::messages::message_handler::TELEGRAMM_CHAR_LIMIT;

pub struct StringBuilder {
    str: Vec<String>,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self {
            str: vec![]
        }
    }
    pub fn add(mut self, prefix: &str, str: Option<String>) -> Self {
        match str {
            None => self,
            Some(str) => {
                let new_str = format!(" - {} {} ", prefix, str);
                self.str.push(new_str);
                self
            }
        }
    }
    pub fn add_many(mut self, strs: &Vec<(String, Option<String>)>) -> Self {
        for (prefix, str) in strs {
            let new_str = format!(" - {} {} ", prefix, str.clone().unwrap_or("".to_owned()));
            self.str.push(new_str);
        }
        self
    }
    pub fn get_str(self) -> String {
        self.str.join("\n ___________________________ \n ")
    }
}

pub fn vec_to_string<T: Display>(vec: Vec<T>, join: &str) -> String {
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