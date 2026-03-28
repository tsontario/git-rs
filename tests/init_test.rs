use my_git::commands::init;
use std::os::unix::fs::PermissionsExt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();
        init::call(temp_path).unwrap();
        assert!(temp_path.join(".git/refs/heads").exists());
        assert!(temp_path.join(".git/refs/tags").exists());
        assert!(temp_path.join(".git/objects/info").exists());
        assert!(temp_path.join(".git/objects/pack").exists());

        assert!(temp_path.join(".git").exists());
        let head_contents = std::fs::read_to_string(temp_path.join(".git/HEAD")).unwrap();
        assert_eq!(head_contents, "ref: refs/heads/main\n");
    }

    #[test]
    fn test_init_with_non_existent_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let non_existent_path = temp_dir.path().join("non_existent_path");
        init::call(&non_existent_path).expect("init should create the root path if it doesn't exist");
    }

    #[test]
    fn test_fail_to_write_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();

        std::fs::set_permissions(&temp_path, std::fs::Permissions::from_mode(0o577)).unwrap();
        init::call(temp_path).expect_err("init should fail if permission denied");
        std::fs::set_permissions(&temp_path, std::fs::Permissions::from_mode(0o777)).unwrap();
    }
}