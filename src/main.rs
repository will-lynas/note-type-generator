use genanki_rs::{Deck, Field, Model, Note, Template};
use serde::Deserialize;
use std::error::Error;
use std::fs::read_to_string;
use std::io;

const CSS_FILE_PATH: &str = "input/style.css";
const CONFIG_PATH: &str = "input/config.toml";

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_note_type_name")]
    note_type_name: String,
    #[serde(default = "default_deck_name")]
    deck_name: String,
    #[serde(default)]
    deck_description: String,
}

fn default_note_type_name() -> String {
    "Imported Note Type".to_string()
}

fn default_deck_name() -> String {
    "Imported Deck".to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let css = match read_to_string(CSS_FILE_PATH) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => String::new(),
            _ => panic!("Error reading file {}", CSS_FILE_PATH),
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
        1607392319,
        &config.note_type_name,
        vec![
            Field::new("Question").font("Arial"),
            Field::new("Answer").font("Arial"),
        ],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    )
    .css(css);

    let my_note = Note::new(my_model, vec!["Capital of Argentina", "Buenos Aires"])?;

    let mut my_deck = Deck::new(2059400110, &config.deck_name, &config.deck_description);

    my_deck.add_note(my_note);
    my_deck.write_to_file("output.apkg")?;
    Ok(())
}
