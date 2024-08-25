use indoc::indoc;

use super::*;

#[test]
fn config_empty() {
    let config = get("").unwrap();
    assert_eq!(config.note_type_name, "Imported Note Type");
    assert_eq!(config.deck_name, "Imported Deck");
    assert_eq!(config.deck_description, "");
    assert_eq!(config.fields, Vec::<String>::new());
    assert_eq!(config.templates, Vec::<TemplateConfig>::new());
    assert_eq!(config.field_font, "Arial");
    assert_eq!(config.output_path, "output.apkg");
}

#[test]
fn unknown_key() {
    // Unknown keys should be ignored
    get("unknown_key = 'asdf'").unwrap();
}

#[test]
fn config_full() {
    let config = get(indoc! {r#"
        note_type_name = "Test Note Type"
        deck_name = "Test Deck"
        deck_description = "Test Description"
        field_font = "Test Font"
        output_path = "test_output.apkg"

        fields = ["FieldX", "FieldY", "FieldZ"]

        [[templates]]
        front_fields = ["FieldX", "FieldY"]
        question_field = "FieldZ"

        [[templates]]
        front_fields = ["FieldZ"]
        question_field = "FieldY"
    "#})
    .unwrap();

    assert_eq!(config.note_type_name, "Test Note Type");
    assert_eq!(config.deck_name, "Test Deck");
    assert_eq!(config.deck_description, "Test Description");
    assert_eq!(config.fields, vec!["FieldX", "FieldY", "FieldZ"]);
    assert_eq!(
        config.templates,
        vec![
            TemplateConfig {
                front_fields: vec!["FieldX".to_string(), "FieldY".to_string()],
                question_field: "FieldZ".to_string()
            },
            TemplateConfig {
                front_fields: vec!["FieldZ".to_string()],
                question_field: "FieldY".to_string()
            }
        ]
    );
    assert_eq!(config.field_font, "Test Font");
    assert_eq!(config.output_path, "test_output.apkg");
}
