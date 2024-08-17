use assert_cmd::Command;
use indoc::indoc;

#[test]
fn empty() {
    let expected_stderr = indoc! {r#"
        error: the following required arguments were not provided:
          --css <CSS>
          --template <TEMPLATE>
          --config <CONFIG>

        Usage: note-type-generator --css <CSS> --template <TEMPLATE> --config <CONFIG>

        For more information, try '--help'.
    "#};

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert()
        .failure()
        .code(2)
        .stderr(expected_stderr)
        .stdout("");
}
