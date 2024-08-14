use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum TemplateError {
    QuestionFieldError(String),
    FrontFieldError(String),
    TemplateFieldNotInFields(String),
    FieldNotInTemplate(String),
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemplateError::QuestionFieldError(ref field) => {
                write!(f, "Question field '{}' is not in fields", field)
            }
            TemplateError::FrontFieldError(ref field) => {
                write!(f, "Front field '{}' is not in fields", field)
            }
            TemplateError::TemplateFieldNotInFields(ref field) => {
                write!(
                    f,
                    "Field '{}' in template is not found in the config fields",
                    field
                )
            }
            TemplateError::FieldNotInTemplate(ref field) => {
                write!(
                    f,
                    "Field '{}' in config is not found in the template",
                    field
                )
            }
        }
    }
}

impl Error for TemplateError {}

#[cfg(test)]
mod tests {
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
}
