use std::{collections::HashMap, fs::File, sync::Arc};
use serde_json::Value;
use walkdir::WalkDir;        

pub type PageMap = HashMap<String, HashMap<String, Value>>;

#[derive(Clone)]
pub struct Translations(pub Arc<PageMap>);

impl Translations {
    pub fn get(&self, page: &str, lang: &str) -> Option<&Value> {
        self.0.get(page)?.get(lang)
    }
    pub fn langs_for(&self, page: &str) -> Vec<&str> {
        self.0.get(page)
              .map(|m| m.keys().map(String::as_str).collect())
              .unwrap_or_default()
    }
    pub fn get_pair(&self, page: &str, lang: &str) -> (&Value, &Value) {
        let common_map = self.0
            .get("common")
            .expect("[i18n] Falta carpeta common");

        let common = common_map
            .get(lang)
            .or_else(|| common_map.get("es"))
            .expect("[i18n] Falta common/es.json");

        let page_json = self
            .get(page, lang)
            .unwrap_or_else(|| self.get(page, "es").expect("[i18n] Falta página default"));

        (page_json, common)
    }
}

pub fn load(dir: &str) -> std::io::Result<Translations> {
    let mut pages: PageMap = HashMap::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
            let lang = entry.path().file_stem().unwrap().to_string_lossy().to_string(); // es, en…
            let page = entry.path().parent().unwrap().file_name().unwrap().to_string_lossy().to_string(); // index, about…
            let json: Value = serde_json::from_reader(File::open(entry.path())?)?;
            pages.entry(page).or_default().insert(lang, json);
        }
    }
    Ok(Translations(Arc::new(pages)))
}
