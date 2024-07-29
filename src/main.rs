use genanki_rs::{Deck, Error, Field, Model, Note, Template};

fn main() -> Result<(), Error> {
    let my_model = Model::new(
        1607392319,
        "Simple Model",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    );
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
