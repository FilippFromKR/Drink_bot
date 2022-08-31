use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::localization::schemas::LangConfig;
use crate::localization::{ENG_CONFIG, UKR_CONFIG};

/// todo: use this in lazy static to show path to localization
#[derive(Debug, Serialize, Deserialize, Clone)]
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
