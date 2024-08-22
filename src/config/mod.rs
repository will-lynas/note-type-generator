use serde::Deserialize;
use toml::de::Error;

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
    #[serde(default = "default_field_font")]
    pub field_font: String,
    #[serde(default = "default_output_path")]
    pub output_path: String,
}

// TODO: Change this name
#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize, Debug, PartialEq)]
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

fn default_field_font() -> String {
    "Arial".to_string()
}

fn default_output_path() -> String {
    "output.apkg".to_string()
}

pub fn get(config_content: &str) -> Result<Config, Error> {
    toml::from_str(config_content)
}

#[cfg(test)]
mod tests;
