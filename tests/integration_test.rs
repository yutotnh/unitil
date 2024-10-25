use std::io::{stdout, Read, Write};

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn integration_test() {
    // unitilを実行するとファイルが上書きされるため、テスト用のディレクトリを作成してそこにコピーを作成する
    let temp_dir = tempdir().unwrap();
    let original_file_path = "tests/fixtures/fullwidth-tilde.txt";
    let copied_file = temp_dir.path().join("fullwidth-tilde.txt");
    std::fs::copy(original_file_path, &copied_file).unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let expected_output = format!("{}:\n", copied_file.to_str().unwrap());
    cmd.arg(&copied_file)
        .assert()
        .success()
        .stdout(expected_output);
}

#[test]
fn integration_test_count() {
    let temp_dir = tempdir().unwrap();
    let original_file_path = "tests/fixtures/fullwidth-tilde.txt";
    let input_file_path = temp_dir.path().join("fullwidth-tilde.txt");
    std::fs::copy(original_file_path, &input_file_path).unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let expected_output = format!(
        "{}:
    Wave dash       (0xA1C1)   : 0
    Fullwidth Tilde (0x8FA2B7) : 4\n",
        input_file_path.to_str().unwrap()
    );

    cmd.arg("-c")
        .arg(&input_file_path)
        .assert()
        .success()
        .stdout(expected_output);
}

#[test]
fn integration_test_error() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let input_text_path = "not_exist.txt";

    let expected_stdout = format!("{}:\n", input_text_path);
    let expected_stderr = "    No such file or directory (os error 2)\n".to_string();

    cmd.arg(input_text_path)
        .assert()
        .failure()
        .stdout(expected_stdout)
        .stderr(expected_stderr);
}
