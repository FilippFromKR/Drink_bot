use std::ops::Deref;
use serde::{Deserialize, Serialize};

use crate::localization::{ENG_CONFIG, UKR_CONFIG};
use crate::localization::schemas::LangConfig;

/// todo: use this in lazy static to show path to localization
#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum Lang {
    Ukr,
    Eng,
}


impl Deref for Lang {
    type Target = LangConfig;

    fn deref(&self) -> &'static Self::Target {
        match self {
            Lang::Eng => &ENG_CONFIG,
            Lang::Ukr => &UKR_CONFIG,
        }
    }
}
