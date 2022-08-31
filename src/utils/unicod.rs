use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::error::error_handler::ErrorHandler;
use crate::utils::helpers::random_num_in_range;
use crate::ErrorType;

type EmojiMap = HashMap<Emojis, Vec<char>>;

lazy_static! {
    static ref EMOJI_LIST: EmojiMap = {
        let mut map: EmojiMap = HashMap::new();
        let e_drink = vec![
            '\u{1F37C}',
            '\u{1F95B}',
            '\u{2615}',
            '\u{1FAD6}',
            '\u{1F375}',
            '\u{1F376}',
            '\u{1F37E}',
            '\u{1F377}',
            '\u{1F378}',
            '\u{1F379}',
            '\u{1F37A}',
            '\u{1F37B}',
            '\u{1F942}',
            '\u{1F943}',
            '\u{1FAD7}',
            '\u{1F964}',
            '\u{1F9CB}',
            '\u{1F9C3}',
            '\u{1F9C9}',
            '\u{1F9CA}',
        ];
        let e_smile = vec![
            '\u{1F600}',
            '\u{1F603}',
            '\u{1F604}',
            '\u{1F601}',
            '\u{1F606}',
            '\u{1F605}',
            '\u{1F923}',
            '\u{1F602}',
            '\u{1F642}',
            '\u{1F643}',
            '\u{1FAE0}',
            '\u{1F609}',
            '\u{1F60A}',
            '\u{1F607}',
        ];
        let e_hello = vec![
            '\u{1F44}', '\u{1F91}', '\u{1F59}', '\u{270B}', '\u{1F59}', '\u{270C}', '\u{1F91}',
            '\u{1F91}', '\u{1F91}', '\u{1F91}', '\u{270A}', '\u{1F44}', '\u{1F91}', '\u{1F91}',
            '\u{1F91}', '\u{1F44}',
        ];
        let e_shit = vec![
            '\u{1F610}',
            '\u{1F910}',
            '\u{1F928}',
            '\u{1F611}',
            '\u{1F636}',
            '\u{1F60F}',
            '\u{1F612}',
            '\u{1F644}',
            '\u{1F976}',
            '\u{1F974}',
            '\u{1F47F}',
            '\u{1F480}',
            '\u{2620}',
            '\u{1F624}',
            '\u{1F621}',
            '\u{1F620}',
            '\u{1F92C}',
            '\u{1F63F}',
            '\u{1F63E}',
        ];
        map.insert(Emojis::Drink, e_drink);
        map.insert(Emojis::Hello, e_hello);
        map.insert(Emojis::Smile, e_smile);
        map.insert(Emojis::ShitHappens, e_shit);
        map
    };
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Emojis {
    Drink,
    Hello,
    ShitHappens,
    Smile,
}

impl Emojis {
    pub fn random(&self) -> Result<&'static char, ErrorHandler> {
        let emojis = EMOJI_LIST.get(self).ok_or(ErrorHandler {
            msg: "Absent one of emojis list.".to_string(),
            ty: ErrorType::Unexpected,
        })?;
        emojis
            .get(random_num_in_range(1, emojis.len()))
            .ok_or(ErrorHandler {
                msg: "Exception in random .".to_string(),
                ty: ErrorType::Unexpected,
            })
    }
}
