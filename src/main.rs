mod args;
mod config;
mod files;

use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};

use genanki_rs::{Deck, Field, Model, Note, Template};
use regex::Regex;

use files::Files;

const OUTPUT_PATH: &str = "output.apkg";

fn hash_string_to_i64(s: &str) -> i64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    hash as i64
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse();
    let files = Files::load(args.template, args.css, args.config);
    let config = config::get(files.config);

    for template_config in &config.templates {
        if !config.fields.contains(&template_config.question_field) {
            panic!(
                "Question field '{}' is not in fields",
                template_config.question_field
            );
        }

        for front_field in &template_config.front_fields {
            if !config.fields.contains(front_field) {
                panic!("Front field '{}' is not in fields", front_field);
            }
        }
    }

    let field_pattern = Regex::new(r"\{\{([^\}]+)\}\}").unwrap();

    let all_fields_in_template: Vec<String> = field_pattern
        .captures_iter(&files.template)
        .map(|cap| cap[1].to_string())
        .collect();

    for field in &config.fields {
        if !all_fields_in_template.contains(field) {
            panic!("Field '{}' in config is not found in the template", field);
        }
    }

    for field in &all_fields_in_template {
        if !config.fields.contains(field) {
            panic!(
                "Field '{}' in template is not found in the config fields",
                field
            );
        }
    }

    let my_model = Model::new(
        hash_string_to_i64(&config.note_type_name),
        &config.note_type_name,
        config
            .fields
            .iter()
            .map(|s| Field::new(&s.clone()).font("Arial"))
            .collect(),
        config
            .templates
            .into_iter()
            .map(|template_config| {
                let mut qfmt = files.template.clone();
                qfmt = qfmt.replace(
                    &format!("{{{{{}}}}}", template_config.question_field),
                    r#"<span class="cloze">?</span>"#,
                );

                let mut afmt = files.template.clone();
                afmt = afmt.replace(
                    &format!("{{{{{}}}}}", template_config.question_field),
                    &format!(
                        r#"<span class="cloze">{{{{{}}}}}</span>"#,
                        template_config.question_field
                    ),
                );

                for field in &config.fields {
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
            .collect(),
    )
    .css(files.css);

    // Use the field names as values on the placeholder note
    let my_note = Note::new(my_model, config.fields.iter().map(|s| s.as_str()).collect())?;

    let mut my_deck = Deck::new(
        hash_string_to_i64(&config.deck_name),
        &config.deck_name,
        &config.deck_description,
    );

    my_deck.add_note(my_note);
    my_deck.write_to_file(OUTPUT_PATH)?;
    Ok(())
}
