use std::io::{Seek, Write};
use my_git::commands::{hash_object, CliConfig};
use my_git::objects::{store};
use my_git::objects::object_hash::ObjectHash;

#[cfg(test)]
mod tests {
    use my_git::objects::object;
    use super::*;
    #[test]
    fn test_hash_object_print_only() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut tempfile = tempfile::Builder::new().tempfile_in(tempdir.path()).unwrap();
        tempfile.write_all(b"hello world").unwrap();
        let config = CliConfig::build(format!("{}", tempdir.path().display()));

        let obj_hash = hash_object::call(&config, &hash_object::HashObjectArgs {
            obj_type: object::ObjectType::Blob,
            write: false,
            file: Some(tempfile.path().to_str().unwrap().to_string()),
        }).unwrap();

        tempfile.seek(std::io::SeekFrom::Start(0)).unwrap();
        let expected_hash = ObjectHash::build(&mut tempfile, &mut std::io::sink(), object::ObjectType::Blob, 11).unwrap();
        assert_eq!(
            expected_hash.hash,
            obj_hash.hash
        )
    }

    #[test]
    fn test_hash_object_write_to_file() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut tempfile = tempfile::Builder::new().tempfile_in(tempdir.path()).unwrap();
        tempfile.write_all(b"hello world").unwrap();
        let config = CliConfig::build(format!("{}", tempdir.path().display()));

        let obj_hash = hash_object::call(&config, &hash_object::HashObjectArgs {
            obj_type: object::ObjectType::Blob,
            write: true,
            file: Some(tempfile.path().to_str().unwrap().to_string()),
        }).unwrap();

        let compressed_path = tempdir.path().join(".git").join("objects").join(store::path_for_object(&obj_hash));
        let mut buf: Vec<u8> = Vec::new();
        store::load(&mut std::fs::File::open(compressed_path).unwrap(), &mut buf, 16).unwrap();
        assert_eq!(b"blob 11\0hello world", buf.as_slice());
    }
}