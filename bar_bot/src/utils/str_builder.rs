pub struct StringBuilder {
    str: Vec<String>,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self { str: vec![] }
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
            let new_str = format!(
                " - {} {} ",
                prefix,
                str.clone().unwrap_or_else(|| "".to_owned())
            );
            self.str.push(new_str);
        }
        self
    }
    pub fn get_str(self) -> String {
        self.str.join("\n ___________________________ \n ")
    }
}
