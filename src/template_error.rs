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
