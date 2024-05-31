use std::collections::HashMap;

pub trait HashMapExt {
    fn to_string(&self) -> String;
}

impl HashMapExt for HashMap<String, String> {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for (key, value) in self {
            result.push_str(&format!("{key}: {value}\r\n"));
        }

        result
    }
}
