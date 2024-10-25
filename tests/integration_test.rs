use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn integration_test() {
    // unitilを実行するとファイルが上書きされるため、テスト用のディレクトリを作成してそこにコピーを作成する
    let temp_dir = tempdir().unwrap();
    let original_file_path = "tests/fixtures/before/fullwidth-tilde-1.txt";
    let input_file_path = temp_dir.path().join("fullwidth-tilde-1.txt");
    std::fs::copy(original_file_path, &input_file_path).unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let expected_output = format!("{}:\n", input_file_path.to_str().unwrap());
    cmd.arg(&input_file_path)
        .assert()
        .success()
        .stdout(expected_output);

    // ファイルが正しく変換されたか確認
    let expected_file_path = "tests/fixtures/after/fullwidth-tilde-1.txt";

    // 中身がEUC-JPなので、バイナリのまま比較する
    let expected_content = std::fs::read(expected_file_path).unwrap();
    let actual_content = std::fs::read(&input_file_path).unwrap();

    assert_eq!(expected_content, actual_content);
}

#[test]
fn integration_test_no_change() {
    let temp_dir = tempdir().unwrap();
    let original_file_path = "tests/fixtures/before/wave-dash.txt";
    let input_file_path = temp_dir.path().join("wave-dash.txt");
    std::fs::copy(original_file_path, &input_file_path).unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let expected_output = format!(
        "{0}:\n    No fullwidth dash in {0}\n",
        input_file_path.to_str().unwrap()
    );
    cmd.arg(&input_file_path)
        .assert()
        .success()
        .stdout(expected_output);

    // ファイルが変更されていないか確認(変更されていないはずだけど、一応afterに置いている)
    let expected_file_path = "tests/fixtures/after/wave-dash.txt";

    // 中身がEUC-JPなので、バイナリのまま比較する
    let expected_content = std::fs::read(expected_file_path).unwrap();
    let actual_content = std::fs::read(&input_file_path).unwrap();

    assert_eq!(expected_content, actual_content);
}

#[test]
fn integration_test_multiple_files() {
    let temp_dir = tempdir().unwrap();
    let original_file_path = [
        "tests/fixtures/before/fullwidth-tilde-1.txt",
        "tests/fixtures/before/wave-dash.txt",
        "tests/fixtures/before/fullwidth-tilde-2.txt",
        "tests/fixtures/before/fullwidth-tilde-3.txt",
    ];

    let input_file_path = [
        temp_dir.path().join("fullwidth-tilde-1.txt"),
        temp_dir.path().join("wave-dash.txt"),
        temp_dir.path().join("fullwidth-tilde-2.txt"),
        temp_dir.path().join("fullwidth-tilde-3.txt"),
    ];

    let expected_file_path = [
        "tests/fixtures/after/fullwidth-tilde-1.txt",
        "tests/fixtures/after/wave-dash.txt",
        "tests/fixtures/after/fullwidth-tilde-2.txt",
        "tests/fixtures/after/fullwidth-tilde-3.txt",
    ];

    for (i, path) in original_file_path.iter().enumerate() {
        std::fs::copy(path, &input_file_path[i]).unwrap();
    }

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let expected_output = format!(
        "{0}:
{1}:
    No fullwidth dash in {1}
{2}:
{3}:\n",
        input_file_path[0].to_str().unwrap(),
        input_file_path[1].to_str().unwrap(),
        input_file_path[2].to_str().unwrap(),
        input_file_path[3].to_str().unwrap(),
    );

    cmd.args(&input_file_path)
        .assert()
        .success()
        .stdout(expected_output);

    for (i, path) in input_file_path.iter().enumerate() {
        // ファイルが正しく変換されたか確認
        let expected_file_path = &expected_file_path[i];

        // 中身がEUC-JPなので、バイナリのまま比較する
        let expected_content = std::fs::read(expected_file_path).unwrap();
        let actual_content = std::fs::read(path).unwrap();

        assert_eq!(expected_content, actual_content);
    }
}

#[test]
fn integration_test_count() {
    let temp_dir = tempdir().unwrap();
    let original_file_path = "tests/fixtures/before/fullwidth-tilde-1.txt";
    let input_file_path = temp_dir.path().join("fullwidth-tilde-1.txt");
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

    // ファイルが変更されていないか確認
    let expected_file_path = "tests/fixtures/before/fullwidth-tilde-1.txt";

    // 中身がEUC-JPなので、バイナリのまま比較する
    let expected_content = std::fs::read(expected_file_path).unwrap();
    let actual_content = std::fs::read(&input_file_path).unwrap();

    assert_eq!(expected_content, actual_content);
}

#[test]
fn integration_test_count_multiple() {
    let temp_dir = tempdir().unwrap();
    let original_file_path = [
        "tests/fixtures/before/fullwidth-tilde-1.txt",
        "tests/fixtures/before/wave-dash.txt",
        "tests/fixtures/before/fullwidth-tilde-2.txt",
        "tests/fixtures/before/no-fullwidth-tilde.txt",
        "tests/fixtures/before/fullwidth-tilde-3.txt",
    ];

    let input_file_path = [
        temp_dir.path().join("fullwidth-tilde-1.txt"),
        temp_dir.path().join("wave-dash.txt"),
        temp_dir.path().join("fullwidth-tilde-2.txt"),
        temp_dir.path().join("no-fullwidth-tilde.txt"),
        temp_dir.path().join("fullwidth-tilde-3.txt"),
    ];

    for (i, path) in original_file_path.iter().enumerate() {
        std::fs::copy(path, &input_file_path[i]).unwrap();
    }

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let expected_output = format!(
        "{}:
    Wave dash       (0xA1C1)   : 0
    Fullwidth Tilde (0x8FA2B7) : 4
{}:
    Wave dash       (0xA1C1)   : 1
    Fullwidth Tilde (0x8FA2B7) : 0
{}:
    Wave dash       (0xA1C1)   : 0
    Fullwidth Tilde (0x8FA2B7) : 1
{}:
    Wave dash       (0xA1C1)   : 0
    Fullwidth Tilde (0x8FA2B7) : 0
{}:
    Wave dash       (0xA1C1)   : 0
    Fullwidth Tilde (0x8FA2B7) : 10\n",
        input_file_path[0].to_str().unwrap(),
        input_file_path[1].to_str().unwrap(),
        input_file_path[2].to_str().unwrap(),
        input_file_path[3].to_str().unwrap(),
        input_file_path[4].to_str().unwrap(),
    );

    cmd.arg("-c")
        .args(&input_file_path)
        .assert()
        .success()
        .stdout(expected_output);

    for (i, path) in input_file_path.iter().enumerate() {
        // ファイルが変更されていないか確認
        let expected_file_path = &original_file_path[i];

        // 中身がEUC-JPなので、バイナリのまま比較する
        let expected_content = std::fs::read(expected_file_path).unwrap();
        let actual_content = std::fs::read(path).unwrap();

        assert_eq!(expected_content, actual_content);
    }
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
