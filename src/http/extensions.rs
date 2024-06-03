use std::collections::HashMap;

pub trait HashMapExt {
    fn to_string(&self) -> String;
}

impl HashMapExt for HashMap<String, String> {
    fn to_string(&self) -> String {
        self.iter().fold(String::new(), |acc, (k, v)| format!("{acc}{k}: {v}\r\n"))
    }
}
