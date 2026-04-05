pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_tree;

pub struct CliConfig {
    pub work_dir: String,
    pub git_dir: std::path::PathBuf,
}

impl CliConfig {
    pub fn build(work_dir: String) -> CliConfig {
        CliConfig {
            work_dir: work_dir.clone(),
            git_dir: Self::resolve_git_dir(std::path::PathBuf::from(work_dir.clone())),
        }
    }

    fn resolve_git_dir(path: std::path::PathBuf) -> std::path::PathBuf {
        let mut dir = std::fs::canonicalize(std::path::PathBuf::from(path)).unwrap();
        loop {
            let git_dir = dir.join(".git");
            if git_dir.exists() {
                return git_dir.parent().unwrap().to_path_buf();
            }
            if dir.parent().is_none() {
                panic!("Could not find .git directory");
            }
            dir = dir.parent().unwrap().to_path_buf();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn resolve_git_dir_finds_git_in_work_dir() {
        let tempdir = TempDir::new().unwrap();
        let absolute_path = std::fs::canonicalize(tempdir.path()).unwrap();
        fs::create_dir(absolute_path.join(".git")).unwrap();

        let config = CliConfig::build(absolute_path.to_str().unwrap().to_string());
        assert_eq!(config.git_dir, absolute_path);
    }

    #[test]
    fn resolve_git_dir_finds_git_in_ancestor() {
        let tempdir = TempDir::new().unwrap();
        let absolute_path = std::fs::canonicalize(tempdir.path()).unwrap();
        fs::create_dir(absolute_path.join(".git")).unwrap();
        let child = absolute_path.join("sub").join("deep");
        fs::create_dir_all(&child).unwrap();

        let config = CliConfig::build(child.to_str().unwrap().to_string());
        assert_eq!(config.git_dir, absolute_path);
    }

    #[test]
    fn resolve_git_dir_from_within_git_dir() {
        let tempdir = TempDir::new().unwrap();
        let absolute_path = std::fs::canonicalize(tempdir.path()).unwrap();
        fs::create_dir_all(absolute_path.join(".git").join("objects")).unwrap();
        let child = absolute_path.join(".git").join("objects");

        let config = CliConfig::build(child.to_str().unwrap().to_string());
        assert_eq!(config.git_dir, absolute_path);
    }
}
