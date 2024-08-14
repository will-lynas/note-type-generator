use genanki_rs::Template;
use regex::Regex;

use crate::{config::TemplateConfig, template_error::TemplateError};

#[derive(Debug, PartialEq)]
struct PreTemplate {
    name: String,
    qfmt: String,
    afmt: String,
}

impl PreTemplate {
    fn new(name: String, qfmt: String, afmt: String) -> Self {
        Self { name, qfmt, afmt }
    }
}

fn pre_create(
    template_configs: Vec<TemplateConfig>,
    fields: Vec<String>,
    template: String,
) -> Result<Vec<PreTemplate>, TemplateError> {
    for template_config in &template_configs {
        if !fields.contains(&template_config.question_field) {
            return Err(TemplateError::QuestionFieldError(
                template_config.question_field.clone(),
            ));
        }

        for front_field in &template_config.front_fields {
            if !fields.contains(front_field) {
                return Err(TemplateError::FrontFieldError(front_field.to_string()));
            }
        }
    }

    let field_pattern = Regex::new(r"\{\{([^\}]+)\}\}").unwrap();

    let all_fields_in_template: Vec<String> = field_pattern
        .captures_iter(&template)
        .map(|cap| cap[1].to_string())
        .collect();

    // TODO: don't panic here
    // Instead, aggregate errors from both loops and report them

    for field in &fields {
        if !all_fields_in_template.contains(field) {
            return Err(TemplateError::FieldNotInTemplate(field.to_string()));
        }
    }

    for field in &all_fields_in_template {
        if !fields.contains(field) {
            return Err(TemplateError::TemplateFieldNotInFields(field.to_string()));
        }
    }

    Ok(template_configs
        .into_iter()
        .map(|template_config| {
            let mut qfmt = template.clone();
            qfmt = qfmt.replace(
                &format!("{{{{{}}}}}", template_config.question_field),
                r#"<span class="cloze">?</span>"#,
            );

            let mut afmt = template.clone();
            afmt = afmt.replace(
                &format!("{{{{{}}}}}", template_config.question_field),
                &format!(
                    r#"<span class="cloze">{{{{{}}}}}</span>"#,
                    template_config.question_field
                ),
            );

            for field in &fields {
                if !template_config.front_fields.contains(field) {
                    qfmt = qfmt.replace(&format!("{{{{{}}}}}", field), "");
                }
            }

            let qfmt = format!(
                "{{{{#{}}}}}\n\n{}\n\n{{{{/{}}}}}\n",
                template_config.question_field, qfmt, template_config.question_field
            );

            PreTemplate::new(template_config.question_field, qfmt, afmt)
        })
        .collect())
}

pub fn create(
    template_configs: Vec<TemplateConfig>,
    fields: Vec<String>,
    template: String,
) -> Result<Vec<Template>, TemplateError> {
    Ok(pre_create(template_configs, fields, template)?
        .iter()
        .map(|template| {
            Template::new(&template.name)
                .qfmt(&template.qfmt)
                .afmt(&template.afmt)
        })
        .collect())
}

#[cfg(test)]
mod tests;
