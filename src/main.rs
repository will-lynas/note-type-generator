use genanki_rs::{Deck, Field, Model, Note, Template};
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::io;

const CSS_FILE_PATH: &str = "input/style.css";
const CONFIG_PATH: &str = "input/config.toml";
const TEMPLATE_PATH: &str = "input/template.html";
const OUTPUT_PATH: &str = "output.apkg";

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_note_type_name")]
    note_type_name: String,
    #[serde(default = "default_deck_name")]
    deck_name: String,
    #[serde(default)]
    deck_description: String,
    #[serde(default)]
    fields: Vec<String>,
    #[serde(default)]
    templates: Vec<TemplateConfig>,
}

#[derive(Deserialize, Debug)]
struct TemplateConfig {
    front_fields: Vec<String>,
    question_field: String,
}

fn default_note_type_name() -> String {
    "Imported Note Type".to_string()
}

fn default_deck_name() -> String {
    "Imported Deck".to_string()
}

fn hash_string_to_i64(s: &str) -> i64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    hash as i64
}

fn main() -> Result<(), Box<dyn Error>> {
    let css = match read_to_string(CSS_FILE_PATH) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => String::new(),
            _ => panic!("Error reading file {}", CSS_FILE_PATH),
        },
    };

    let template = match read_to_string(TEMPLATE_PATH) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => String::new(),
            _ => panic!("Error reading file {}", TEMPLATE_PATH),
        },
    };

    let config_content = match read_to_string(CONFIG_PATH) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => String::new(),
            _ => panic!("Error reading file {}", CONFIG_PATH),
        },
    };

    let config: Config = toml::from_str(&config_content).expect("Error parsing config toml");

    let my_model = Model::new(
        hash_string_to_i64(&config.note_type_name),
        &config.note_type_name,
        config
            .fields
            .iter()
            .map(|s| Field::new(&s.clone()).font("Arial"))
            .collect(),
        config
            .templates
            .into_iter()
            .enumerate()
            .map(|(index, template_config)| {
                let qfmt = template_config.front_fields.join(" ")
                    + " -> "
                    + &template_config.question_field;
                Template::new(&format!("Card {}", index + 1))
                    .qfmt(&qfmt)
                    .afmt(&template)
            })
            .collect(),
    )
    .css(css);

    // Use the field names as values on the placeholder note
    let my_note = Note::new(my_model, config.fields.iter().map(|s| s.as_str()).collect())?;

    let mut my_deck = Deck::new(
        hash_string_to_i64(&config.deck_name),
        &config.deck_name,
        &config.deck_description,
    );

    my_deck.add_note(my_note);
    my_deck.write_to_file(OUTPUT_PATH)?;
    Ok(())
}
