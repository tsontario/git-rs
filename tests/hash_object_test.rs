use assert_cmd::Command;
use my_git::objects::object;
use my_git::objects::object::Object;
use my_git::objects::object_hash::ObjectHash;
use my_git::objects::store;
use std::io::Seek;

mod common;
#[test]
fn test_hash_object_print_only() {
    let (tempdir, mut tempfile, _) = common::init_simple_git_dir().unwrap();

    let hash_object_out = Command::cargo_bin("my-git")
        .unwrap()
        .args([
            "-C",
            tempdir.path().to_str().unwrap(),
            "hash-object",
            "-t",
            "blob",
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

    tempfile.seek(std::io::SeekFrom::Start(0)).unwrap();
    let expected_hash = ObjectHash::build(
        &mut tempfile,
        &mut std::io::sink(),
        object::ObjectType::Blob,
        11,
    )
    .unwrap();
    assert_eq!(expected_hash.hash, object_hash)
}

#[test]
fn test_hash_object_write_to_file() {
    let (tempdir, tempfile, _) = common::init_simple_git_dir().unwrap();
    let store = store::Store::new(tempdir.path().to_path_buf()).unwrap();

    let hash_object_out = Command::cargo_bin("my-git")
        .unwrap()
        .args([
            "-C",
            tempdir.path().to_str().unwrap(),
            "hash-object",
            "-w",
            "-t",
            "blob",
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

    let mut decoded_object = store.decode_object(&object_hash).unwrap();
    assert_eq!(b"blob 11\0hello world", decoded_object.as_slice());
}
