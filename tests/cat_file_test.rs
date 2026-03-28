#[cfg(test)]
mod tests {
    use std::io::Write;
    use my_git::commands::{cat_file, hash_object, CliConfig};
    use my_git::objects::{object};

    #[test]
    fn test_cat_file_print_type() {
        let tempdir = tempfile::tempdir().unwrap();
        let mut tempfile = tempfile::Builder::new().tempfile_in(tempdir.path()).unwrap();
        tempfile.write_all(b"hello world").unwrap();

        let git_dir = tempdir.path().join(".git");
        let config = CliConfig{
            work_dir: format!("{}", tempdir.path().display()),
            git_dir: Some(git_dir),
        };

        let obj_hash = hash_object::call(&config, &hash_object::HashObjectArgs {
            obj_type: object::ObjectType::Blob,
            write: true,
            file: Some(tempfile.path().to_str().unwrap().to_string()),
        }).unwrap();

        let result = cat_file::call(&config, &cat_file::CatFileArgs {
            obj_hash: format!("{}", obj_hash.hash),
            show_type: true,
            show_size: false,
            show_content: false,
        });
        assert_eq!(result.unwrap(), "blob".to_string());

        let result = cat_file::call(&config, &cat_file::CatFileArgs {
            obj_hash: format!("{}", obj_hash.hash),
            show_type: false,
            show_size: true,
            show_content: false,
        });
        assert_eq!(result.unwrap(), "11".to_string());

        let result = cat_file::call(&config, &cat_file::CatFileArgs {
            obj_hash: format!("{}", obj_hash.hash),
            show_type: false,
            show_size: false,
            show_content: true,
        });
        assert_eq!(result.unwrap(), "hello world".to_string());
    }
}