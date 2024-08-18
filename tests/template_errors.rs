use std::io::Write;

use assert_cmd::Command;
use indoc::indoc;
use tempfile::NamedTempFile;

#[test]
fn template_not_in_fields() {
    let css_file = NamedTempFile::new().unwrap();
    let mut template_file = NamedTempFile::new().unwrap();
    let config_file = NamedTempFile::new().unwrap();

    template_file.write_all(b"{{does_not_exist}}").unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("--css")
        .arg(css_file.path())
        .arg("--template")
        .arg(template_file.path())
        .arg("--config")
        .arg(config_file.path());

    let expected_stderr = indoc! {r#"
        Error generating templates: Field 'does_not_exist' in template is not found in the config fields
        "#};

    cmd.assert()
        .failure()
        .code(1)
        .stdout("")
        .stderr(expected_stderr);
}

#[test]
fn field_not_in_template() {
    let css_file = NamedTempFile::new().unwrap();
    let template_file = NamedTempFile::new().unwrap();
    let mut config_file = NamedTempFile::new().unwrap();

    config_file
        .write_all(indoc! {br#"
            fields = [
            "does_not_exist",
            ]
        "#})
        .unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("--css")
        .arg(css_file.path())
        .arg("--template")
        .arg(template_file.path())
        .arg("--config")
        .arg(config_file.path());

    let expected_stderr = indoc! {r#"
        Error generating templates: Field 'does_not_exist' in config is not found in the template
        "#};

    cmd.assert()
        .failure()
        .code(1)
        .stdout("")
        .stderr(expected_stderr);
}
