use clap::{Parser, ValueHint};
use genanki_rs::{Deck, Field, Model, Note, Template};
use regex::Regex;
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::PathBuf;

const OUTPUT_PATH: &str = "output.apkg";

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(
        long,
        value_hint = ValueHint::FilePath,
        value_parser = clap::value_parser!(PathBuf)
    )]
    css: PathBuf,

    #[arg(
        long,
        value_hint = ValueHint::FilePath,
        value_parser = clap::value_parser!(PathBuf)
    )]
    template: PathBuf,

    #[arg(
        long,
        value_hint = ValueHint::FilePath,
        value_parser = clap::value_parser!(PathBuf)
    )]
    config: PathBuf,
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_note_type_name")]
    note_type_name: String,
    #[serde(default = "default_deck_name")]
    deck_name: String,
    #[serde(default)]
    deck_description: String,
    #[serde(default)]
    fields: Vec<String>,
    #[serde(default)]
    templates: Vec<TemplateConfig>,
}

#[derive(Deserialize, Debug)]
struct TemplateConfig {
    front_fields: Vec<String>,
    question_field: String,
}

fn default_note_type_name() -> String {
    "Imported Note Type".to_string()
}

fn default_deck_name() -> String {
    "Imported Deck".to_string()
}

fn hash_string_to_i64(s: &str) -> i64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    hash as i64
}

fn get_file_contents(path: PathBuf) -> String {
    // TODO: Return Error instead of panicking
    match read_to_string(path.clone()) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                panic!("File does not exist: {}", path.clone().display());
            }
            _ => panic!("Error reading file {}", path.clone().display()),
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let template = get_file_contents(args.template);
    let css = get_file_contents(args.css);
    let config_content = get_file_contents(args.config);

    let config: Config = toml::from_str(&config_content).expect("Error parsing config toml");

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
        .captures_iter(&template)
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
    .css(css);

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
