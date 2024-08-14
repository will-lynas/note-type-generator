use genanki_rs::Template;
use regex::Regex;

use crate::config::TemplateConfig;

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
) -> Vec<PreTemplate> {
    for template_config in &template_configs {
        if !fields.contains(&template_config.question_field) {
            panic!(
                "Question field '{}' is not in fields",
                template_config.question_field
            );
        }

        for front_field in &template_config.front_fields {
            if !fields.contains(front_field) {
                panic!("Front field '{}' is not in fields", front_field);
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
            panic!("Field '{}' in config is not found in the template", field);
        }
    }

    for field in &all_fields_in_template {
        if !fields.contains(field) {
            panic!(
                "Field '{}' in template is not found in the config fields",
                field
            );
        }
    }

    template_configs
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
        .collect()
}

pub fn create(
    template_configs: Vec<TemplateConfig>,
    fields: Vec<String>,
    template: String,
) -> Vec<Template> {
    pre_create(template_configs, fields, template)
        .iter()
        .map(|template| {
            Template::new(&template.name)
                .qfmt(&template.qfmt)
                .afmt(&template.afmt)
        })
        .collect()
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

        let templates = pre_create(template_configs, fields, template);

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
    #[should_panic(expected = "Question field 'FieldX' is not in fields")]
    fn question_field_not_in_fields() {
        let template_configs = vec![TemplateConfig {
            question_field: String::from("FieldX"),
            front_fields: vec![],
        }];
        let fields = vec![];
        let template = String::from("");

        pre_create(template_configs, fields, template);
    }

    #[test]
    #[should_panic(expected = "Front field 'FieldY' is not in fields")]
    fn front_field_not_in_fields() {
        let template_configs = vec![TemplateConfig {
            question_field: String::from("FieldX"),
            front_fields: vec![String::from("FieldY")],
        }];
        let fields = vec![String::from("FieldX")];
        let template = String::from("");

        pre_create(template_configs, fields, template);
    }
}
