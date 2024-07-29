use genanki_rs::{Deck, Field, Model, Note, Template};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const CSS_FILE_PATH: &str = "input/style.css";

fn main() -> Result<(), Box<dyn Error>> {
    let mut custom_css = String::new();

    if let Ok(css_file) = File::open(CSS_FILE_PATH) {
        let mut buf_reader = BufReader::new(css_file);

        if let Err(e) = buf_reader.read_to_string(&mut custom_css) {
            eprintln!("Error reading file {}: {}", CSS_FILE_PATH, e);
            return Err(Box::new(e));
        }
    }

    let my_model = Model::new(
        1607392319,
        "Simple Model",
        vec![
            Field::new("Question").font("Arial"),
            Field::new("Answer").font("Arial"),
        ],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    )
    .css(custom_css);

    let my_note = Note::new(my_model, vec!["Capital of Argentina", "Buenos Aires"])?;

    let mut my_deck = Deck::new(
        2059400110,
        "Country Capitals",
        "Deck for studying country capitals",
    );

    my_deck.add_note(my_note);
    my_deck.write_to_file("output.apkg")?;
    Ok(())
}
