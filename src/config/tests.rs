use super::*;

#[test]
fn config_empty() {
    let config = get("".to_string());
    assert_eq!(config.note_type_name, "Imported Note Type");
    assert_eq!(config.deck_name, "Imported Deck");
    assert_eq!(config.deck_description, "");
    assert_eq!(config.fields, Vec::<String>::new());
    assert_eq!(config.templates, Vec::<TemplateConfig>::new());
    assert_eq!(config.field_font, "Arial")
}
