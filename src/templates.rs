use genanki_rs::Template;
use regex::Regex;

use crate::config::TemplateConfig;

pub fn create(
    template_configs: Vec<TemplateConfig>,
    fields: Vec<String>,
    template: String,
) -> Vec<Template> {
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
                "{{{{#{}}}}}\n\n{}\n\n{{{{/{}}}}}",
                template_config.question_field, qfmt, template_config.question_field
            );

            Template::new(&template_config.question_field)
                .qfmt(&qfmt)
                .afmt(&afmt)
        })
        .collect()
}
