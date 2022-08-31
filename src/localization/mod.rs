use lazy_static::lazy_static;

use crate::localization::schemas::LangConfig;
use std::fs::File;
use std::io::{BufReader, Read};

pub mod lang;
pub mod schemas;

lazy_static! {
    pub static ref ENG_CONFIG: LangConfig = {
        let file = File::open("./localization/en.json").expect("could not open file");
        let buffered_reader = BufReader::new(file);
        let bytes = buffered_reader
            .bytes()
            .filter_map(|b| b.ok())
            .collect::<Vec<u8>>();
        serde_json::from_slice::<LangConfig>(bytes.as_slice())
            .expect("Fail to parse the localization file.")
    };
    pub static ref UKR_CONFIG: LangConfig = {
        let file = File::open("./localization/ukr.json").expect("could not open file");
        let buffered_reader = BufReader::new(file);
        let bytes = buffered_reader
            .bytes()
            .filter_map(|b| b.ok())
            .collect::<Vec<u8>>();
        serde_json::from_slice::<LangConfig>(bytes.as_slice())
            .expect("Fail to parse the localization file.")
    };
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufReader, Read};

    use crate::localization::lang::Lang;
    use crate::localization::{ENG_CONFIG, UKR_CONFIG};

    use crate::localization::schemas::LangConfig;

    #[test]
    pub fn test() {
        assert_eq!(&*Lang::Ukr.send_commands, UKR_CONFIG.send_commands)
    }

    #[test]
    fn test2() {
        let file = File::open("./localization/ukr.json").expect("could not open file");
        let mut buffered_reader = BufReader::new(file);
        let bytes = buffered_reader
            .bytes()
            .filter(|b| b.is_ok())
            .map(|b| b.unwrap())
            .collect::<Vec<u8>>();
        let result = serde_json::from_slice::<LangConfig>(bytes.as_slice());
        assert!(result.is_ok());
        println!("{:?}", result.unwrap());
    }

    #[test]
    fn test3() {
        assert_eq!(ENG_CONFIG.send_commands, "Here we go: ");
    }
    #[test]
    fn test4() {
        assert_eq!(UKR_CONFIG.send_commands, "Поїхали!: ");
    }
}
