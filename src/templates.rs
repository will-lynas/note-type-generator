use genanki_rs::Template;
use regex::Regex;

use crate::{config::TemplateConfig, template_error::TemplateError};

// TODO: Only derive these during tests
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
mod tests {
    use super::*;
    use indoc::indoc;

    impl PreTemplate {
        fn new_from_str(name: &str, qfmt: &str, afmt: &str) -> Self {
            Self {
                name: name.to_string(),
                qfmt: qfmt.to_string(),
                afmt: afmt.to_string(),
            }
        }
    }

    #[test]
    fn create_template_success() {
        let template_configs = vec![
            TemplateConfig {
                question_field: String::from("Field1"),
                front_fields: vec![String::from("Field2")],
            },
            TemplateConfig {
                question_field: String::from("Field3"),
                front_fields: vec![String::from("Field1"), String::from("Field2")],
            },
        ];

        let fields = vec![
            String::from("Field1"),
            String::from("Field2"),
            String::from("Field3"),
        ];

        let template = String::from("{{Field1}} | {{Field2}} | {{Field3}}");

        let templates = pre_create(template_configs, fields, template).unwrap();

        assert_eq!(templates.len(), 2);

        let expected_templates: Vec<PreTemplate> = vec![
            PreTemplate::new_from_str(
                "Field1",
                indoc! {r#"
                {{#Field1}}

                <span class="cloze">?</span> | {{Field2}} | 

                {{/Field1}}
                "#},
                indoc! {r#"
                <span class="cloze">{{Field1}}</span> | {{Field2}} | {{Field3}}"#},
            ),
            PreTemplate::new_from_str(
                "Field3",
                indoc! {r#"
                {{#Field3}}

                {{Field1}} | {{Field2}} | <span class="cloze">?</span>

                {{/Field3}}
                "#},
                indoc! {r#"
                {{Field1}} | {{Field2}} | <span class="cloze">{{Field3}}</span>"#},
            ),
        ];

        assert_eq!(templates, expected_templates);
    }

    #[test]
    fn question_field_not_in_fields() {
        let template_configs = vec![TemplateConfig {
            question_field: String::from("FieldX"),
            front_fields: vec![],
        }];
        let fields = vec![];
        let template = String::from("");

        assert_eq!(
            pre_create(template_configs, fields, template)
                .err()
                .unwrap(),
            TemplateError::QuestionFieldError("FieldX".to_string())
        );
    }

    #[test]
    fn front_field_not_in_fields() {
        let template_configs = vec![TemplateConfig {
            question_field: String::from("FieldX"),
            front_fields: vec![String::from("FieldY")],
        }];
        let fields = vec![String::from("FieldX")];
        let template = String::from("");

        assert_eq!(
            pre_create(template_configs, fields, template)
                .err()
                .unwrap(),
            TemplateError::FrontFieldError("FieldY".to_string())
        );
    }

    #[test]
    fn config_field_not_in_template() {
        let template_configs = vec![];
        let fields = vec![String::from("FieldX")];
        let template = String::from("");

        assert_eq!(
            pre_create(template_configs, fields, template)
                .err()
                .unwrap(),
            TemplateError::FieldNotInTemplate("FieldX".to_string())
        );
    }

    #[test]
    fn template_field_not_in_config() {
        let template_configs = vec![];
        let fields = vec![];
        let template = String::from("{{FieldX}}");

        assert_eq!(
            pre_create(template_configs, fields, template)
                .err()
                .unwrap(),
            TemplateError::TemplateFieldNotInFields("FieldX".to_string())
        );
    }
}
