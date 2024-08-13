use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_note_type_name")]
    pub note_type_name: String,
    #[serde(default = "default_deck_name")]
    pub deck_name: String,
    #[serde(default)]
    pub deck_description: String,
    #[serde(default)]
    pub fields: Vec<String>,
    #[serde(default)]
    pub templates: Vec<TemplateConfig>,
}

#[derive(Deserialize, Debug)]
pub struct TemplateConfig {
    pub front_fields: Vec<String>,
    pub question_field: String,
}

fn default_note_type_name() -> String {
    "Imported Note Type".to_string()
}

fn default_deck_name() -> String {
    "Imported Deck".to_string()
}

pub fn get(config_content: String) -> Config {
    // TODO: proper error handling here
    toml::from_str(&config_content).expect("Error parsing config toml")
}
