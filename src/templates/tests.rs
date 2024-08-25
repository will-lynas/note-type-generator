use indoc::indoc;

use super::*;

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

    let template = "{{Field1}} | {{Field2}} | {{Field3}}";

    let templates = pre_create(template_configs, &fields, template).unwrap();

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
    let template = "";

    assert_eq!(
        pre_create(template_configs, &fields, template)
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
    let template = "";

    assert_eq!(
        pre_create(template_configs, &fields, template)
            .err()
            .unwrap(),
        TemplateError::FrontFieldError("FieldY".to_string())
    );
}

#[test]
fn config_field_not_in_template() {
    let template_configs = vec![];
    let fields = vec![String::from("FieldX")];
    let template = "";

    assert_eq!(
        pre_create(template_configs, &fields, template)
            .err()
            .unwrap(),
        TemplateError::FieldNotInTemplate("FieldX".to_string())
    );
}

#[test]
fn template_field_not_in_config() {
    let template_configs = vec![];
    let fields = vec![];
    let template = "{{FieldX}}";

    assert_eq!(
        pre_create(template_configs, &fields, template)
            .err()
            .unwrap(),
        TemplateError::TemplateFieldNotInFields("FieldX".to_string())
    );
}
