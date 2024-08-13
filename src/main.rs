mod args;
mod config;
mod files;
mod templates;

use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};

use genanki_rs::{Deck, Field, Model, Note};

use files::Files;

const OUTPUT_PATH: &str = "output.apkg";

fn hash_string_to_i64(s: &str) -> i64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    hash as i64
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse();
    let files = Files::load(args.template, args.css, args.config);
    let config = config::get(files.config);

    let my_model = Model::new(
        hash_string_to_i64(&config.note_type_name),
        &config.note_type_name,
        config
            .fields
            .iter()
            .map(|s| Field::new(&s.clone()).font("Arial"))
            .collect(),
        templates::create(config.templates, config.fields.clone(), files.template),
    )
    .css(files.css);

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
