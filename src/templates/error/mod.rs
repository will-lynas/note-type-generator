use std::{error::Error, fmt};

// TODO: Change this name
#[allow(clippy::module_name_repetitions)]
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
                write!(f, "Question field '{field}' is not in fields")
            }
            TemplateError::FrontFieldError(ref field) => {
                write!(f, "Front field '{field}' is not in fields")
            }
            TemplateError::TemplateFieldNotInFields(ref field) => {
                write!(
                    f,
                    "Field '{field}' in template is not found in the config fields"
                )
            }
            TemplateError::FieldNotInTemplate(ref field) => {
                write!(
                    f,
                    "Field '{field}' in config is not found in the template"
                )
            }
        }
    }
}

impl Error for TemplateError {}

#[cfg(test)]
mod tests;
