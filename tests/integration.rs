use assert_cmd::Command;
use color_eyre::eyre::Result;
use assert_fs::prelude::*;

#[test]
/// make sure help runs. This indicates the binary works
fn test_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden")?;
    let assert = cmd.arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
/// make sure write help runs. This indicates the binary works
fn test_write_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden")?;
    let assert = cmd.arg("write").arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
/// execute write command, saving a file out
fn test_write() {
    let mut cmd = Command::cargo_bin("garden").unwrap();
    let fake_editor_path = std::env::current_dir()
        .expect("Expect to be in a dir")
        .join("tests")
        .join("fake_editor.sh");

    if !fake_editor_path.exists() {
        panic!("Fake editor shell script could not be found");
    };

    let assert = cmd
        .env("EDITOR", fake_editor_path.into_os_string())
        .arg("write")
        .arg("-t")
        .arg("atitle")
        .write_stdin("N\n".as_bytes())
        .assert();

    assert.success();
}
