use genanki_rs::{Deck, Field, Model, Note, Template};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n}\n";

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
