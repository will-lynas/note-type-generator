use super::*;

#[test]
fn template_errors() {
    assert_eq!(
        format!(
            "{}",
            TemplateError::QuestionFieldError("FieldX".to_string())
        ),
        "Question field 'FieldX' is not in fields"
    );
    assert_eq!(
        format!("{}", TemplateError::FrontFieldError("FieldX".to_string())),
        "Front field 'FieldX' is not in fields"
    );
    assert_eq!(
        format!(
            "{}",
            TemplateError::TemplateFieldNotInFields("FieldX".to_string())
        ),
        "Field 'FieldX' in template is not found in the config fields"
    );
    assert_eq!(
        format!(
            "{}",
            TemplateError::FieldNotInTemplate("FieldX".to_string())
        ),
        "Field 'FieldX' in config is not found in the template"
    );
}
