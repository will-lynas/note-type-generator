use super::*;

#[test]
fn question_field_error() {
    assert_eq!(
        format!(
            "{}",
            TemplateError::QuestionFieldError("FieldX".to_string())
        ),
        "Question field 'FieldX' is not in fields"
    );
}

#[test]
fn front_field_error() {
    assert_eq!(
        format!("{}", TemplateError::FrontFieldError("FieldX".to_string())),
        "Front field 'FieldX' is not in fields"
    );
}

#[test]
fn template_field_not_in_fields() {
    assert_eq!(
        format!(
            "{}",
            TemplateError::TemplateFieldNotInFields("FieldX".to_string())
        ),
        "Field 'FieldX' in template is not found in the config fields"
    );
}

#[test]
fn field_not_in_template() {
    assert_eq!(
        format!(
            "{}",
            TemplateError::FieldNotInTemplate("FieldX".to_string())
        ),
        "Field 'FieldX' in config is not found in the template"
    );
}
