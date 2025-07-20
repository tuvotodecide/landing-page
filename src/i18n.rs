use std::{collections::HashMap, fs};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Translations(pub Arc<HashMap<String, Value>>);

impl Translations {
    pub fn get(&self, lang: &str) -> Option<&Value> {
        self.0.get(lang)
    }
    pub fn codes(&self) -> Vec<&str> {
        self.0.keys().map(String::as_str).collect()
    }
}

pub fn load(dir: &str) -> std::io::Result<Translations> {
    let mut map = HashMap::new();
    for entry in fs::read_dir(dir)? {
        let p = entry?.path();
        if p.extension().and_then(|s| s.to_str()) == Some("json") {
            let code = p.file_stem().unwrap().to_string_lossy().to_string();
            let json: Value = serde_json::from_reader(fs::File::open(&p)?)?;
            map.insert(code, json);
        }
    }
    Ok(Translations(Arc::new(map)))
}
