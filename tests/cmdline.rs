use assert_cmd::Command;
use indoc::indoc;
use tempfile::NamedTempFile;

#[test]
fn empty() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let expected_stderr = indoc! {r#"
        error: the following required arguments were not provided:
          --css <FILE>
          --template <FILE>
          --config <FILE>

        Usage: note-type-generator --css <FILE> --template <FILE> --config <FILE>

        For more information, try '--help'.
    "#};
    let cmdline_args_error_code = 2;

    cmd.assert()
        .failure()
        .code(cmdline_args_error_code)
        .stderr(expected_stderr)
        .stdout("");
}

#[test]
fn good_empty_files() {
    let css_file = NamedTempFile::new().unwrap();
    let template_file = NamedTempFile::new().unwrap();
    let config_file = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("--css")
        .arg(css_file.path())
        .arg("--template")
        .arg(template_file.path())
        .arg("--config")
        .arg(config_file.path());
    println!("{}", css_file.path().to_str().unwrap());

    cmd.assert().success().stdout("").stderr("");
}
