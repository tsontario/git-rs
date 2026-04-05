mod common;

use assert_cmd::Command;
use my_git::commands::CliConfig;

#[test]
fn test_cat_file_missing_required_arg() {
    let (tempdir, _tempfile, git_dir) = common::init_simple_git_dir().unwrap();
    let config = CliConfig {
        work_dir: format!("{}", tempdir.path().display()),
        git_dir,
    };
    Command::cargo_bin("my-git")
        .unwrap()
        .arg("cat-file")
        .assert()
        .failure();
}

#[test]
fn test_cat_file_print_type() {
    let (tempdir, tempfile, git_dir) = common::init_simple_git_dir().unwrap();

    let hash_object_out = Command::cargo_bin("my-git")
        .unwrap()
        .args([
            "-C",
            tempdir.path().to_str().unwrap(),
            "hash-object",
            "-w",
            tempfile.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .get_output()
        .clone();
    let object_hash = String::from_utf8(hash_object_out.stdout)
        .unwrap()
        .trim()
        .to_string();

    let cat_file_out = Command::cargo_bin("my-git")
        .unwrap()
        .args([
            "-C",
            tempdir.path().to_str().unwrap(),
            "cat-file",
            "-t",
            object_hash.as_str(),
        ])
        .assert()
        .success()
        .get_output()
        .clone();

    let err = String::from_utf8(cat_file_out.stderr).unwrap();
    assert_eq!(
        String::from_utf8(cat_file_out.stdout).unwrap(),
        "blob\n".to_string()
    );
}

#[test]
fn test_cat_file_print_size() {
    let (tempdir, tempfile, git_dir) = common::init_simple_git_dir().unwrap();
    let hash_object_out = Command::cargo_bin("my-git")
        .unwrap()
        .args([
            "-C",
            tempdir.path().to_str().unwrap(),
            "hash-object",
            "-w",
            tempfile.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .get_output()
        .clone();
    let object_hash = String::from_utf8(hash_object_out.stdout)
        .unwrap()
        .trim()
        .to_string();

    let cat_file_out = Command::cargo_bin("my-git")
        .unwrap()
        .args([
            "-C",
            tempdir.path().to_str().unwrap(),
            "cat-file",
            "-s",
            object_hash.as_str(),
        ])
        .assert()
        .success()
        .get_output()
        .clone();

    let err = String::from_utf8(cat_file_out.stderr).unwrap();
    assert_eq!(
        String::from_utf8(cat_file_out.stdout).unwrap(),
        "11\n".to_string()
    );
}

#[test]
fn test_cat_file_print_content() {
    let (tempdir, tempfile, git_dir) = common::init_simple_git_dir().unwrap();
    let hash_object_out = Command::cargo_bin("my-git")
        .unwrap()
        .args([
            "-C",
            tempdir.path().to_str().unwrap(),
            "hash-object",
            "-w",
            tempfile.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .get_output()
        .clone();
    let object_hash = String::from_utf8(hash_object_out.stdout)
        .unwrap()
        .trim()
        .to_string();

    let cat_file_out = Command::cargo_bin("my-git")
        .unwrap()
        .args([
            "-C",
            tempdir.path().to_str().unwrap(),
            "cat-file",
            "-p",
            object_hash.as_str(),
        ])
        .assert()
        .success()
        .get_output()
        .clone();

    let err = String::from_utf8(cat_file_out.stderr).unwrap();
    assert_eq!(
        String::from_utf8(cat_file_out.stdout).unwrap(),
        "hello world\n".to_string()
    );
}
